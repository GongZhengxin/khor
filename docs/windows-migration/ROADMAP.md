# Khor Windows 迁移路线图（W0 → W8）

> 状态：2026-11-14 起由 Lead 接管执行。本文件是里程碑的唯一来源；每个里程碑在开工前必须具备"目标 / 范围 / 验收门 / NOT_TESTED 边界"，收尾时必须提交验收文档（`docs/windows-migration/W<n>-*.md`）。
> 上位约束见 [APPROVAL-PACKAGE](APPROVAL-PACKAGE.md)：不 push、不合 main、不签名、不发布；平台范围 Windows 11 x64（主），Windows 10 x64（W7 验证），ARM64/Server 不纳入。

## 0. 当前进度

| 里程碑 | 状态 | 交付 |
| --- | --- | --- |
| W0 基线 / 功能矩阵 / 审批包 | ✅ 完成 | [BASELINE](BASELINE.md)、[FEATURE-MATRIX](FEATURE-MATRIX.md)、[APPROVAL-PACKAGE](APPROVAL-PACKAGE.md) |
| W1 构建底座 + CI | ✅ 完成 | [W1-ACCEPTANCE](W1-ACCEPTANCE.md)、[W1-CI](W1-CI.md)；远端 run 43559639618 success |
| W2 节点层 Windows 适配 | 🟡 进行中 | W2-A ✅（[验收](W2-TABS-WINDOWS-LOCK.md)）；W2-B ✅ 收敛到 4 条（3 条平台无关预存红 + 1 条已知 flake，[验收](W2-B-NODE-SUITE.md)）；W2-C/D 待做 |
| W3 进程 / 终端 / 文件 smoke | ⬜ 未开始 | — |
| W4 设备网络 + Session 端到端 | ⬜ 未开始 | — |
| W5 GUI / Web 桌面端 | ⬜ 未开始 | — |
| W6 打包 / 更新 / 回滚 | ⬜ 未开始 | — |
| W7 性能稳定性 + Windows 10 | ⬜ 未开始 | — |
| W8 发布就绪包 | ⬜ 未开始 | — |

## 1. 里程碑定义

### W2 · 节点层 Windows 适配（crates/node）

**目标**：`crates/node` 在 Windows 上不再是"能编译但一堆失败"，而是**要么真能工作、要么显式返回结构化 unsupported**；lib 测试基线可解释、可回归。

- **W2-A tabs 写锁** ✅：已完成并验收。
- **W2-B 失败分诊与分批修复**：把当前 125 条全量 lib 失败收敛为 0，或逐条给出 `NOT_TESTED` + 理由（Linux-only 语义）。分批见 [W2-FAILURE-TRIAGE](W2-FAILURE-TRIAGE.md)。
- **W2-C 评审收尾**：W2-A 评审提出的 6 项（Windows CI 跑 tabs 套件、仓内 rename-vs-reader 回归、锁测试握手、place-pins 幂等、临时文件清理、`Hold` 重入防护）。
- **W2-D 剩余功能 stub**：`khor serve` keeper 生命周期、`khor quit`、attach 原始终端（ConPTY）、`secret` askpass 父进程检测、`foreground_group` / `tty_path`。

**验收门**：`cargo test --locked -p khor-node --lib` 在 Windows 上 0 失败或全部 `NOT_TESTED` 有理由；`cargo check --locked -p khor-node` exit 0；`cargo fmt --check` exit 0；Windows CI 包含 node lib 套件。

### W3 · 进程 / 终端 / 文件 smoke

**目标**：真实 Windows 上的端到端冒烟，而不只是单测。

- 进程：Job Object 生命周期、取消、清理（沿用/扩展现有 `windows-process-harness`）。
- 终端：ConPTY 下的启动、尺寸、取消、TUI 输出、attach。
- 文件：读写/复制/打开/删除/替换、长路径、只读与 ACL。
- **tabs 真实双进程**：GUI 窗口 + `khor tabs` 并发写同一张表。

**验收门**：一份可执行的 smoke 清单 + 每条的实际命令/退出码/日志；失败项必须是显式 unsupported 或已修。

### W4 · 设备网络 + Session 端到端

**目标**：两台 Windows（或 Windows ↔ 现有平台）的配对、连接、断线重连、Session 创建/恢复/关闭、跨设备同步、远程控制与审批。

**验收门**：至少一次真实双机配对与一次断线重连的证据；Session 恢复后内容与审批状态一致。

### W5 · GUI / Web 桌面端

**目标**：Tauri 窗口在 Windows 启动、连接本地/远程节点、WebView2 运行正常；前端构建在 Windows 可重复。

- 已知问题：Vite 7 拒绝含 `#` 的工程根路径，构建必须在无 `#` 路径（`F:\khor-build2`）或给 Vite 稳定别名；这是 W5 必须消解的工程约束（文档化或修构建脚本）。
- GUI 侧阻塞锁：`tabs_*` 同步 Tauri command 在争用时可能卡 UI 线程，W3/W5 评估 `#[tauri::command(async)]` 或 `spawn_blocking`。

**验收门**：Windows 上 GUI 冷启动到可交互、打开会话、终端面板可用；截图/日志证据。

### W6 · 打包 / 更新 / 回滚

**目标**：产出 Windows 安装包（MSI/NSIS/zip 按现有 `tauri.conf.json` 能力），并验证更新检查、安装、回滚。

- 签名：**只做验证设计，不签名**（用户未授权）。
- 安装布局、版本切换、卸载、`bundle.active` 的 release matrix 调整。

**验收门**：安装包可在干净 Windows 11 上安装→启动→卸载；更新路径至少一次成功与一次失败回滚。

### W7 · 性能与稳定性 + Windows 10

**目标**：按 [APPROVAL-PACKAGE](APPROVAL-PACKAGE.md) §5 的负载测性能：CLI 冷启动、GUI 冷启动、Session 创建/恢复、配对与重连、终端吞吐、文件扫描/复制/同步、更新检查。

- 阈值：相对现有平台 >30% 需解释，>100% 阻断（除非有平台原因并经用户批准）。
- Windows 10 x64 至少一次兼容验证。

**验收门**：性能矩阵表 + 中位数/P95/峰值；Win10 记录；阻断项清单。

### W8 · 发布就绪包

**目标**：把 W0-W7 的成果整理成可交付物：安装/更新/凭据文档、已知限制清单、验收证据索引、最终独立 review、合并建议（是否合 main 由用户决定）。

**验收门**：一份 `docs/windows-migration/RELEASE-READINESS.md`；所有 NOT_TESTED 有归属；用户签字前的最后一次全量证据。

## 2. 执行规则（每个里程碑都适用）

1. **证据三件套**：命令 + 退出码 + 日志路径；日志在仓外时，仓库文档记录 sha256。
2. **不做静默降级**：平台不支持返回结构化错误；不能把 skipped 写成 passed。
3. **切片独立提交**：一个可交付切片一个 commit，提交信息写清验证命令。
4. **A/B 对照**：涉及"修好/回归"的声明，必须给出同工具链、同命令的前后对照。
5. **工具链固定**：rustc 1.95.0 + rustfmt 1.9.0（与 CI 一致）；全部装在 `F:\khor-toolchain`，不写 C 盘。
6. **构建路径**：前端构建必须在无 `#` 路径；`cargo` 不受影响。
7. **边界**：只在 `steward/s2-windows-20261114` 分支，不 push（除非为触发 CI 且用户明确同意）、不合 main、不签名、不发布。

## 3. 依赖与顺序

```text
W2 (node 层) ─┬─> W3 (进程/终端/文件 smoke) ──> W4 (网络 + Session)
              └─> W5 (GUI/Web) ──> W6 (打包/更新) ──> W7 (性能) ──> W8 (就绪)
```

- W3 依赖 W2 的进程/文件原语可用；
- W5 依赖 W1 的前端构建链 + W3 的终端；
- W6 依赖 W5 的 GUI 产物与 W2 的更新/凭据能力；
- W7/W8 依赖全部。

## 4. 风险登记

| 风险 | 影响 | 应对 |
| --- | --- | --- |
| Windows 与 Unix 语义差异（路径、锁、权限、进程） | 测试大面积红，或行为静默不同 | W2-B 分诊逐条定性；不做"跳过当通过" |
| 工具链 / 构建环境被外部清理（本次已发生一次） | 无法复现证据 | 工具链固定在 F:，env 备份 + 版本记录；每阶段开始先跑 `doctor` 式自检 |
| 路径含 `#`（Vite） | 前端构建失败 | 无 `#` 构建 worktree；W5 消解 |
| 签名/发布未授权 | 无法交付安装包 | W6 只做验证设计；发布由用户决定 |
| 长跑目标漂移 | 里程碑失焦 | 本文件为唯一来源；每阶段收尾更新 §0 |

## 5. 变更记录

- 2026-11-14：建立本路线图；W2-A 已验收，W2-B 分诊启动。
