# W7 · 性能矩阵

按 [APPROVAL-PACKAGE](APPROVAL-PACKAGE.md) §5.2：同 HEAD、同依赖锁定、同工作负载，Linux 作为现有基线，记录中位数 / P95 / 峰值；预热 2 次、之后 5 次有效。

**两台是不同硬件，这里是跨机器对比，不是受控的平台对比**——记录硬件是结论的一部分，不能省略：

| | Windows | Linux |
| --- | --- | --- |
| CPU | Intel i9-10900（10 核 / 20 线程） | AMD Ryzen 9 9950X（16 核 / 32 线程） |
| 系统 | Windows 11（26100） | Linux 7.0.0-42 x86_64 |
| 二进制 | `khor.exe` 103.9 MB（release） | `khor` 97.6 MB（release） |
| HEAD | `3fe1dab0` | 同一提交（`git archive` 打包上传） |
| 工具链 | rustc 1.95.0 | rustc 1.95.0 |

测量脚本：Windows `scripts/windows-perf.ps1`；Linux `scripts/windows-perf.sh`（同一指标表、同样预热/次数）。

## 1. 矩阵

| 指标 | Windows 中位 / P95（ms） | Linux 中位 / P95（ms） | 比值 | 判定（§5.2：>30% 需解释，>100% 阻断） |
| --- | --- | --- | --- | --- |
| `cli_version` | 36.9 / 49.5 | 1 / 1 | 37× | **阻断项**，已归因：Windows 进程创建成本，见 §2.1 |
| `cli_doctor` | 5871.9 / 5917.8 | 2619 / 2619 | 2.2× | **阻断项**，已归因：探针含网络超时，见 §2.4 |
| `session_create` | 6405.8 / 6613.7 | 273 / 275 | 23.5× | **阻断项**，见 §2.2（待进一步测量） |
| `sessions_list` | 132.3 / 134.4 | 20 / 20 | 6.6× | **阻断项**，已归因：进程创建 + NTFS，见 §2.1 |
| `file_scan` | 1117.6 / 1150.7 | 28 / 33 | 40× | **阻断项**，见 §2.3（待进一步测量） |
| `file_copy` | 49.0 / 49.6 | 2 / 2 | 24.5× | **阻断项**，已归因：进程创建占 70%以上 |
| `gui_cold_start` | **530.2 / 594.7** | 不适用 | — | Linux 无桌面会话，该行只在能显示窗口的平台测量 |

`gui_cold_start` 的定义是「窗口自己的传输开始应答」，不是「窗口出现」——一个能画但所有命令都挂着的窗口不算就绪。

## 2. 归因（哪些是平台常数，哪些是待查项）

### 2.1 Windows 的进程创建是 ~35 ms 的常数

实测同一台 Windows 上，一个什么都不做的 `khor` 调用中位 **58.9 ms**（含错误路径与 PATH 处理），`cmd.exe /c exit` 中位 **56.3 ms**。即：在这台机器上**启动一个进程本身就要几十毫秒**，与 Khor 的代码无关（PE 装载 + Defender 实时扫描）。

据此：
- `cli_version` 36.9 ms ≈ 纯进程创建；Linux 1 ms 是 `date +%s%3N` 的分辨率下限。
- `sessions_list` 132 ms ≈ 36 ms 启动 + ~96 ms 读存储；Linux 20 ms ≈ 1 ms + 19 ms。
- `file_copy` 的 49 ms 里，Windows 上 `cp` 等价的进程成本占大头（Linux 2 ms 是纯内核拷贝）。

**结论**：这几行相对 Linux 的倍数主要反映**平台常数**，不是 Windows 实现劣化。按 §5.2 仍需列为阻断项等待用户确认，但归因明确。

### 2.2 `session_create`：23.5×，待进一步测量（最可能是 Defender 扫描）

6.4 s 对 273 ms。这 6 s 花在「派生宿主并等它就位」上，而宿主是从**不可变运行时镜像**重新执行的
（`crates/node/src/runtime_image.rs`，`%LOCALAPPDATA%\Khor\runtime\<hash>`，与主二进制同为 ~104 MB）。
每个新会话都要重新映射/扫描这 104 MB，Windows 的实时防护对每次执行都要过一遍。

**这不是结论，是待验证的假设。** 下一步测法（未做）：
1. 把 `%LOCALAPPDATA%\Khor\runtime` 加入 Defender 排除项前后各测 5 次，比较。
2. 直接计时 `Start-Process` 该运行时镜像（不经 khor），与 `Start-Process cmd.exe` 对比。
3. 若确认，产品侧的解法是缩小运行时镜像（宿主只需要 node 的一小部分），而不是改会话逻辑。

### 2.3 `file_scan`：40×，待进一步测量

`khor ls <本机> <40 个小文件的目录>`：1118 ms 对 28 ms。其中进程创建只占 ~36 ms，
剩下 ~1080 ms 落在目录遍历/元数据读取。可能的来源：Defender 对每个文件的首次访问、NTFS 元数据、
以及 `khor ls` 走的是「列出后逐个 stat」而非批量。

**待验证**：同一目录连续调用第二次是否变快（缓存）；用 `Measure-Command` 直接 `Get-ChildItem` 同目录作对照。
若直接 `Get-ChildItem` 也是秒级，则与 Khor 无关。

### 2.4 `cli_doctor`：2.2×

`doctor` 含中继连通性探针，单条中继可耗到 2 s 级超时。两台机器的网络出口不同，
这行测的是「这台机器的网络」，不是 CPU 或代码。要可比，需固定探针目标或排除网络项。

## 3. Windows 10

**`NOT_TESTED`。** 本席位只有一台 Windows 11（26100）。归属：需要一台 Win10 x64 机器执行
`scripts/windows-smoke.ps1` 与 `scripts/windows-perf.ps1`。不把它写成 passed。

## 4. 本轮已做的优化（有前后对照）

| 项 | 前 | 后 | 证据 |
| --- | --- | --- | --- |
| 首屏 FCP（窗口内实测） | 1280 ms | **664–712 ms**（3 次预热后） | `first-contentful-paint`，见 [W5-GUI §6](W5-GUI.md) 与提交 `3056754b` |
| 首屏 JS 堆 | 51 MB | 32 MB | `performance.memory` |
| 慢命令阻塞轮询 | 单线程，一条 6.4 s 命令冻结所有轮询 | 4 worker：`doctor`（单跑 8.8 s）在飞时，`sessions` 轮询 312/190/176/174 ms | 传输层并发，提交 `7d067d49` |
| 会话轮询叠加 | 每 2 s 无在途保护，多次读数乱序落地 | 有在途保护（同 `fetchDevices` 的既有做法） | 同上 |

**冷启动方差**：重新链接后的第一次启动 FCP 读到 **2844 ms**（Defender 扫描新二进制 + 磁盘缓存冷），
随后三次为 712 / 664 / 672 ms。报告数字时用预热值，但要知道用户升级后的**第一次**启动更接近前者。
这也是 §5.5 那个方向（资源走 loopback）值得做的原因之一：它去掉 250 次预载请求与首屏的竞争。

## 5. 下一轮优化的量化清单（已由 bundle/性能分析测出，未实施）

启动图为 2.79 MB 入口，其中：应用代码 55.4%、markdown 栈 12.5%、React 10.9%、UI kit 9.5%、
xterm core 6.2%、shiki core 4.9%。可回收的部分（按可回收量排序）：

1. **详情区子树懒加载**（`ChatView` + `MachineChat` + `DelegatedSessionView` 一起放到一个 `lazy()` 边界），
   可把 markdown 栈 + shiki core 移出启动图，**约 18%**。⚠️ 本轮试过只把 `DetailPane` 的三处改懒：
   入口只从 2861 KB 降到 2857 KB，因为 `App.tsx:178` 仍静态引用 `MachineChat`，所以**必须先处理 App 的引用**
   （`retainKhorPromptTokens` 拆成小模块，`MachineChat` 在 App 里也懒加载）。已回退，未留半成品。
2. **设置/更多页懒加载**（`AppearancePage` 等），可移出 xterm core 与 react-simple-keyboard，**约 7.4%**。
3. **seti 图标 JSON 移出首图**（`FileKindIcon`），**约 2.9%**。
4. **`gen/catalog.ts` 按语言拆分**，**约 5.6%**（需改生成器，语言切换时再取另一份）。
5. **把资源也交给 loopback 服务**（`crates/web` 已有嵌入的 `ASSETS` 表）：页面与资源同源后
   可以完全恢复正常代码分割，并把 250 个 `modulepreload` 去掉。代价：页面 origin 变成
   `127.0.0.1`，需要 `capabilities` 的 `remote.urls`，且该 origin 的 localStorage 会重置一次。

## 6. 证据索引

| 项 | 命令 | 退出码 | 结果 |
| --- | --- | --- | --- |
| Windows 矩阵 | `pwsh scripts/windows-perf.ps1 -Khor <release khor.exe> -Gui <release khor-gui.exe> -Runs 5 -Warmup 2` | 0 | `F:\Temp\khor-perf\perf-windows.json` |
| Linux 矩阵 | `bash scripts/windows-perf.sh -b ./target/release/khor -s ~/khor-perf/perf-home` | 0 | `~/khor-perf/perf-linux.json`（LAB 192.168.3.6） |
| 首屏时序 | `.cache/w7-page-timing.ps1`（CDP `performance` + `paint`） | — | §4 表 |
| 并发 | 同上 §4；`doctor` 在飞时 4 次 `sessions` 轮询 | — | 312/190/176/174 ms |
