# W2-B：Windows 全量 lib 失败分诊

## 1. 这份文件是什么

2026-11-14 在 Windows 11 + rustc 1.95.0 上跑 `cargo test --locked -p khor-node --lib -- --test-threads=1`：

```text
963 passed; 125 failed; 14 ignored          （HEAD b5596fe0，342.31s）
```

本文件把这 125 条按**症状聚类**、给出初判与批次。**初判不是结论**：每一批开工时先用复现命令确认根因，再决定"真修"还是"gate 成 NOT_TESTED"。

判定规则（来自 [ROADMAP](ROADMAP.md) §2）：产品 bug 必须真修；只有**平台语义本身不存在**的测试才允许 `#[cfg(unix)]` gate，并计入 NOT_TESTED；不允许把 skipped 写成 passed。

## 2. 聚类总览

方法：解析 `.w2\p1-fulllib.log` 的每个失败块（`---- <test> stdout ----`），按 panic 信息与调用点聚类。原始分类数据：`.w2\failures-buckets.json`、`.w2\failures-classified.json`。

| 簇 | 条数（近似） | 症状 | 初判 | 批次 |
| --- | --- | --- | --- | --- |
| C1 扩展路径前缀 | 13 | `git ["fast-import", "--export-marks=\\\\?\\F:\\Temp\\..."]` → `Invalid argument` | **产品 bug** | B1 |
| C2 程序查找失败 | ~19 | `Error { kind: NotFound, message: "program not found" }`；`sidecar program not found in explicit ...` | 混合：可能是测试夹具用 `sh`/`khor` 名字，也可能是产品 PATH 解析 | B2 |
| C3 POSIX 绝对路径 | ~17 | `"路径必须是绝对路径：/work/theirs"`、`/etc/hosts`、`/usr/bin/...` | **测试夹具**（Unix 语义） | B4 |
| C4 权限 / 共享冲突 | ~33 | `Os { code: 5, kind: PermissionDenied, message: "拒绝访问。" }`、`"拒绝访问。 (os error 5)"` | **疑似产品 bug**（Windows 文件共享语义，可能一处共享修复清一大片） | B3 |
| C5 tmux | ~9 | `its process sits in tmux`、tmux 计数断言 | **平台专属**（tmux 只有 Unix） | B4 |
| C6 其它 | ~28 | 路径分隔符、`Elapsed`、设置页、usage/vitals 等逐条不同 | 逐条复现 | B5 |

合计 125。聚类按"第一条匹配到的症状"归并，个别测试可能同时命中两类，批次开工时以该测试的独立复现为准。

### 2.1 高价值假设

C4 的 33 条集中在 `session_content::journal`、`live::*`、`khor_agent::*`、`delegation`、`workspace` —— 都是"写临时文件再 rename / 同时开多个句柄"的模式。tabs 写锁那一轮的探针已经证明：**Windows 的 rename 可以覆盖一个被读句柄打开的文件**，但**若目标文件被写句柄打开、或打开时没带 `FILE_SHARE_DELETE`，rename 会 SharingViolation**。如果这些模块共用一个 `atomic_write` 或 journal 替换路径，一处修好可能连带清掉 20-30 条 —— 这是 B3 优先做的原因。

## 3. 批次计划

每个批次 = 一个可独立提交的切片，流程固定：**复现（RED）→ 定性（产品/夹具/环境）→ 修 → 同命令复验（GREEN）→ 提交**。

### B1 · `\\?\` 扩展前缀（13 条）

- 代表：`git::repository_history_tests::git_watch_plan_covers_upstream_and_external_index_without_a_status_walk`
- 复现：`cargo test --locked -p khor-node --lib git::repository_history_tests:: -- --test-threads=1`
- 期望：`--export-marks` 收到的是 `F:\Temp\...`，不是 `\\?\F:\Temp\...`；`git fast-import` 接受。
- 修法方向：在把路径交给外部程序前剥掉 `\\?\` 前缀（`dunce` 风格），或改用 canonicalize 后的短路径；只对"传给外部程序"的路径生效，不影响 Rust 内部 IO。

### B2 · 程序查找失败（~19 条）

- 代表：`khor_agent::accounts::tests::*`、`live::tests::a_dead_row_says_how_it_ended`
- 复现：按测试名逐条跑 `--exact`。
- 定性要点：先确认"找不到的是 `sh`、`khor`、还是 sidecar 名字"。若测试夹具假设了 Unix 的 `sh`，那是夹具问题；若产品在 Windows 上不会自行解析 `.exe`/PATH，那是产品问题。

### B3 · 权限 / 共享冲突（~33 条，优先）

- 代表：`session_content::journal::tests::base_is_atomic_and_pending_recovery_does_not_claim_committed_content`（journal.rs:1152）、`workspace::tests::newest_file_looks_two_levels_down_past_hidden_and_build_directories`（workspace.rs:292）、`live::tests::gui_*`
- 复现：`cargo test --locked -p khor-node --lib session_content::journal:: -- --test-threads=1`
- 定性要点：是哪一次系统调用返回 5（open? rename? delete?）。若是"rename 覆盖被打开的文件"，考虑统一改为"先删后改名 / 用 `REPLACE_EXISTING` 语义 / 打开时补 `FILE_SHARE_DELETE`"；若是"删目录时里面有打开的文件"，考虑重试。

### B4 · POSIX 夹具与 tmux（~26 条）

- 代表：`tests::forgetting_this_machine_points_at_leave`（`/work/theirs`）、`files::make_tests::a_machine_with_no_opener_refuses_by_naming_the_variable`（`/etc/hosts`）、`live::tests::*tmux*`
- 处理：`#[cfg(unix)]` gate + 文档记 NOT_TESTED；若能在不改变语义的前提下换 Windows 等价夹具（例如用临时目录代替 `/work/theirs`），优先移植而不是 gate。
- 注意：tmux 相关是**功能在 Windows 不存在**，不是测试问题；要确认产品在 Windows 上是显式 unsupported（而不是 panic）。

### B5 · 其它逐条（~28 条）

- 代表：`usage::tests::*`、`vitals::tests::*`、`watch::native::tests::*`、`place::tests::*`、`road::tests::*`、`ipc::tests::*`、`files::search_tests::*`
- 处理：逐条复现；路径分隔符类（`\\README.md` vs `README.md`）优先，因为可能是一处公共路径规范化函数。

## 4. 一个批次做完的标准

1. 该批次的失败数 → 0，或每条都有 `NOT_TESTED` + 理由（写进当批验收文档）；
2. 全量 lib 无新增失败（与 `b5596fe0` 的 125 条对照，逐名比对）；
3. `cargo check --locked -p khor-node` exit 0、`cargo fmt --check -p khor-node` exit 0；
4. 提交信息写明验证命令；
5. 更新 [ROADMAP](ROADMAP.md) §0 与本文件 §5。

## 5. 进展

| 日期 | 批次 | 结果 | 提交 |
| --- | --- | --- | --- |
| 2026-11-14 | 分诊（125 条） | 完成初判，B1 起手 | — |
