# W3：进程 / 终端 / 文件 smoke

## 1. 目标与验收门

见 [ROADMAP](ROADMAP.md) §1 W3：真实 Windows 上的端到端冒烟，而不是只跑单测。

**验收门**：一份可执行的 smoke 清单 + 每条的实际命令 / 退出码 / 日志；失败项必须显式 unsupported 或已修。

## 2. 环境前提（先读这条）

W3 的每一项都要真实创建进程、伪终端或管道。**受限令牌父进程会拒绝命名管道**，因此 W3 必须在**非沙箱**环境执行：
普通 PowerShell，或 DSH 的 `full` preset（C 盘仍由 `dsh-delete-guard` 拦截，F 盘与工作区完全可写）。

2026-11-15 在 DSH `workspace-write` 沙箱内的三条直接证据：

```text
NamedPipeServerStream + NamedPipeClientStream.Connect(800)  → Access to the path is denied.
cargo test -p khor-node --lib khor_agent::accounts::tests::  → 2 failed: "khor agent 无法启动：拒绝访问。 (os error 5)"
cargo test -p khor-cli --test console                        → 2 failed: 退出码 78，stderr "拒绝访问。 (os error 5)"
```

沙箱内跑出的 `os error 5` 先按**环境**分类，不记为产品缺陷；产品缺陷要在非沙箱环境复现后才成立。

## 3. 矩阵（状态随执行更新）

| # | 区域 | 用例 | 方法 | 状态 |
| --- | --- | --- | --- | --- |
| P1 | 进程 | 子进程启动与退出码透传 | 由 node 启动一个短命子进程，断言退出码 | 待执行 |
| P2 | 进程 | Job 关闭即杀子进程树（`KILL_ON_JOB_CLOSE`） | 启动带孙进程的树，关闭 job，枚举确认全灭 | 待执行 |
| P3 | 进程 | 取消（`TerminateJobObject`）后无残留 | 取消后按 pid 复核不存在 | 待执行 |
| P4 | 进程 | 父进程退出、子进程被回收 | 直接终止 supervisor，检查宿主/子进程 | 待执行 |
| T1 | 终端 | ConPTY 启动 shell 并读到提示符 | `portable-pty` 走 node 的 host 路径 | 待执行 |
| T2 | 终端 | 键入回显 | 写入一行命令，读回输出 | 待执行 |
| T3 | 终端 | 改变尺寸 | `ResizePseudoConsole` 后子程序读到新列宽 | 待执行 |
| T4 | 终端 | 关闭后无宿主残留 | 关闭终端后按 pid 复核 | 待执行 |
| T5 | 终端 | attach 到已存在终端（协议） | `attach_version` 目标 | 待执行 |
| F1 | 文件 | 目标被读句柄打开时的原子替换 | journal/durable 的 rename 语义 | 待执行 |
| F2 | 文件 | 只读属性文件的替换与删除 | `attrib +R` 后写/删的错误语义 | 待执行 |
| F3 | 文件 | >260 字符长路径创建/读取/删除 | `\\?\` 前缀与 plain path 的边界 | 待执行 |
| F4 | 文件 | 权限拒绝返回结构化错误 | 受限目录下的写/删 | 待执行 |
| F5 | 文件 | 只读/受限目录的清理路径 | `lab clean` 与产品删除路径 | 待执行 |
| B1 | tabs | 两个进程并发写同一张表 | 两个 `khor tabs` 并发 open/close | 待执行 |
| B2 | tabs | GUI 与 CLI 并发（GUI 可用时） | 窗口内开 tab + 外部 `khor tabs` | 待执行 |

## 4. 执行记录

| 日期 | 命令 | 结果 | 证据 |
| --- | --- | --- | --- |
| 2026-11-15 | 命名管道探针（沙箱内） | 客户端连接被拒 | 本文 §2 |

## 5. 下一步

1. 在非沙箱环境按矩阵逐项执行，记录命令 / 退出码 / 日志；
2. 失败项定性：产品缺陷 → TDD 修复；平台不存在 → 结构化 unsupported 并记 NOT_TESTED；
3. 全部执行完后把本文件升为 W3 验收文档，并在 [ROADMAP](ROADMAP.md) §0 更新状态。
