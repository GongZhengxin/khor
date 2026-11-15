# W3：进程 / 终端 / 文件 smoke

## 1. 目标与验收门

见 [ROADMAP](ROADMAP.md) §1 W3：真实 Windows 上的端到端冒烟，而不是只跑单测。

**验收门**：一份可执行的 smoke 清单 + 每条的实际命令 / 退出码 / 日志；失败项必须显式 unsupported 或已修。

## 2. 环境前提（先读这条）

W3 的每一项都要真实创建进程、伪终端或管道。**受限令牌父进程会拒绝命名管道**，因此 W3 必须在**非沙箱**环境执行：
普通 PowerShell，或 DSH 的 `full` preset（C 盘仍由 `dsh-delete-guard` 拦截，F 盘与工作区完全可写）。

2026-11-15 在 DSH `workspace-write` 沙箱内的直接证据：

```text
NamedPipeServerStream + NamedPipeClientStream.Connect(800)  → Access to the path is denied.
cargo test -p khor-node --lib khor_agent::accounts::tests::  → 2 failed: "khor agent 无法启动：拒绝访问。 (os error 5)"
cargo test -p khor-cli --test console                        → 2 failed: 退出码 78，stderr "拒绝访问。 (os error 5)"
khor help                                                    → 退出码 78，"拒绝访问。 (os error 5)"
```

ConPTY 也逃不掉：一次临时探针里 `openpty → spawn cmd.exe → kill → wait` 全部成功（进程确实被回收），
但**从 master 读输出永远阻塞**——ConPTY 自己的内部管道同样受限。因此 T1–T4 只能在非沙箱环境验证。

`khor` CLI 除 `--version` 外都要建立本机管道，因此整个 CLI 也在同一限制内；这不是 Khor 的缺陷。

## 3. 运行器

```powershell
pwsh -File scripts/windows-smoke.ps1 -CargoTargetDir '<workspace>\.cache\target'
# 在受限令牌环境里加 -Sandboxed：含 "os error 5" 的失败记为 ENV 而不是 FAIL
```

每项一条记录：`name / area / status / exit / seconds / first line`，完整输出落在 `.cache/w3/<name>.log`，
汇总写成 `.cache/w3/smoke-<utc>.json`。`FAIL` 才让脚本以非零退出；`ENV` 单独计数。

## 4. 矩阵与当前结果（2026-11-15，沙箱内）

| # | 区域 | 用例 | 运行方式 | 状态 |
| --- | --- | --- | --- | --- |
| P1 | 进程 | 子进程启动与退出码 | `khor --version` | **PASS** |
| P1–P3 | 进程 | Job 生命周期 / 取消 / 清理 | `cargo test -p khor-node --lib khor_agent_process::` | **PASS**（28.6s） |
| P4 | 进程 | 父退出后清理（`khor quit`） | `cargo test -p khor-cli --test quit` | ENV |
| T1 | 终端 | CLI 能起一个会话 | `khor help` | ENV |
| T1–T4 | 终端 | ConPTY 启动 / 回显 / 尺寸 / 关闭 | `cargo test -p khor-cli --test console` | ENV |
| T5 | 终端 | attach 到已存在终端 | `cargo test -p khor-cli --test attach_version` | 待执行 |
| F1 | 文件 | 原子替换 / 句柄语义 | `cargo test -p khor-node --lib session_content::journal::` | **PASS**（2.4s） |
| F2 | 文件 | 只读表：结构化报错而非 panic | `tabs::tests::a_read_only_table_reports_an_error_instead_of_panicking` | **PASS**（1.8s） |
| F3 | 文件 | >260 字符路径仍能读写 | `tabs::tests::a_table_beyond_max_path_still_opens` | **PASS**（2.3s） |
| F4–F5 | 文件 | 权限拒绝 / 受限目录清理 | `cargo test -p khor-cli --test ws` | ENV |
| B1 | tabs | 两个**进程**并发写同一张表 | `tabs::tests::two_processes_do_not_lose_each_others_tabs`（子进程走真实文件锁） | **PASS**（2.2s） |
| B1′ | tabs | 两个 `khor tabs` CLI 进程并发 | `khor tabs ls` ×2 | ENV |
| B2 | tabs | GUI 与 CLI 并发 | 需要 GUI | 待执行 |

沙箱内汇总：**6 PASS / 5 ENV / 0 FAIL**，receipt `.cache/w3/smoke-20261115T123428Z.json`。

F3 的结论值得记下：Rust 的 `std::fs` 在 Windows 上会把绝对路径内部改写成 verbatim（`\\?\`）形式，因此 310 字符的表路径可以正常读写；
风险只剩“调用方交给产品的是相对路径或自己拼的前缀”。

## 5. 执行记录

| 日期 | 命令 | 结果 | 证据 |
| --- | --- | --- | --- |
| 2026-11-15 | `scripts/windows-smoke.ps1 -Sandboxed` | 6 PASS / 5 ENV / 0 FAIL | `.cache/w3/smoke-20261115T123428Z.json` |
| 2026-11-15 | 命名管道探针 | 客户端连接被拒 | 本文 §2 |

## 6. 下一步

1. 切到非沙箱环境（`full` preset）后重跑同一脚本，把 ENV 全部转成 PASS/FAIL；
2. 失败项定性：产品缺陷 → TDD 修复；平台不存在 → 结构化 unsupported 并记 NOT_TESTED；
3. 补齐 T5、B2，并把本文件升为 W3 验收文档，更新 [ROADMAP](ROADMAP.md) §0。
