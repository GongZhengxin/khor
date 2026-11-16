# 第一阶段（S1）：Windows 上的 Agent 大同世界

> 目标与取舍见本文 §1；已完成与未完成见 §2、§7。每一项都留了命令与输出，
> 没做到的写「未做」，不把没验证的写成通过。

## 1. 定下的方向（用户决策）

1. **默认 Khor 对话**，原生终端可选；两条栏位并存。
2. **两个都做**：Khor 自建对话（`_ghost` + ACP/Pi）与「原生应用拥有对话」。
3. **原生会话在手机上接受按键中转**（`term_key`），并在界面上标注为远程终端。
4. **npm 可选、自动探测、绝不强制**：主路径（内置 Codex、Pi/DeepSeek、原生接管）零依赖；
   只有注册表里的第三方 ACP 适配器需要包运行器。
5. DeepSeek 走 Khor 自有 agent（Pi）；ACP 注册表里没有 DeepSeek 适配器。

## 2. 已完成（本阶段）

### 2.1 窗口形态：Windows 恒为 GUI 子系统（提交 `40093bd7`）

**问题**：`#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` 让 debug 构建成为
控制台程序，而 release/安装包是在标题栏修复之前打的（release 二进制 21:11、安装包 20:41，
修复提交在 21:39）。于是这台机器上仅有的两个可运行产物各错一半：**一个带控制台，一个有双层标题栏**，
轮换使用就成了「窗口在两种形态间横跳」。

**改法**：Windows 恒为 GUI 子系统；原本由控制台承担的诊断改由 `diagnostics` 模块写日志文件
（`%LOCALAPPDATA%\Khor\logs\gui.log`，`KHOR_LOG_FILE` 可覆盖，8 MiB 一轮转），
`KHOR_GUI_CONSOLE=1` 改分配真实控制台给开发用。

**证据**：

| 项 | 结果 |
| --- | --- |
| debug 二进制 PE 子系统 | 3 → **2（WINDOWS_GUI）** |
| 启动后可见控制台窗口 | **0** |
| 日志文件 | 已创建，首行为 `[gui] 2026-11-17 00:19:53 start 0.2.0 (…\khor-gui.exe)`，随后是 `khor: window transport on 127.0.0.1:1432 (4 workers)` 与页面自身流量 |
| `KHOR_GUI_CONSOLE=1` | 出现 conhost 子进程 → 逃生门有效 |

### 2.2 子进程不再闪控制台（提交 `992f2ebd`）

GUI 进程没有控制台，Windows 会给每个控制台子进程**新开一个可见控制台**。仓库里此前**从未**设置过
`CREATE_NO_WINDOW`。新增 `console::hidden`（返回带 `CREATE_NO_WINDOW` 的 `Command`，
而不是接收 `Command`——`creation_flags` 是**替换**语义，见模块注释），覆蓋 doctor 的两处
（`khor.exe version`、`khor serve`）、编辑器的两处（`code --version`、`code serve-web`）、
以及添加机器时走的 6 处 ssh/ssh-keygen。

顺带修掉一个真 bug：`khor_cli_binary` 在 Windows 上找的是没有 `.exe` 的 `khor`，永远找不到，
于是「诊断」在明明有 CLI 的机器上报「没有 CLI」。现在用平台自己的后缀。
证据：`doctor_fix(serve)` 返回「a resident was started」（修复前这条调用定位不到二进制）。

测试：`doctor` 1、`code` 9、`satellite` 26 全过。node 全量 lib 首轮 8 失败、复跑 4 失败（1053 / 1057 passed），
逐个隔离跑全部通过（35 + 15 + 2），是既有的全量并发 flaky，与本次改动无关。

### 2.3 Agent 清单：这台机器实际有什么（提交 `678c25c7`、`d277c83d`）

**为什么需要**：`agents known` 回答的是「khor 知不知道启动它的 argv」，是注册表快照；
而人先问的是「**这台机器上有什么用、在哪、khor 能拿它做什么**」。两件事不能并成一张表：
本机装着的 DSH 从不在 ACP 注册表里，注册表里十几个条目本机一个都没装。

`agent_fleet` 用文件系统能回答的事实构建清单（PATH / 已知安装目录 / 配置目录是否存在），
**绝不启动任何东西**（起 10 个 CLI 判断它们活不活要花几秒，而多数 agent 未登录前什么也不答）。
路线的优先级写死在数据里：`builtin`（khor 自带 shim，零 npm）→ `native-acp`（工具自带 ACP，零 npm）
→ `registry-acp`（包运行器适配器，唯一需要 runner 的一档，行上直接标 `needs-runner`）
→ `terminal`（地板，任何 agent 都能被敲进终端）。

**证据（CLI）**：

```
$ khor agents fleet
9 known, 2 installed, package runner: pnpm dlx
codex   Codex              installed  C:\Users\admin\AppData\Local\OpenAI\Codex\bin\…\codex.exe  builtin   acp run list read resume send terminal
claude  Claude Code        not installed  -  -  terminal
dsh     DeepSeek Harness   installed  F:\DeepSeek Harness\…\dsh.cmd  terminal  run resume send terminal
gemini  Gemini CLI         not installed  -  -  terminal
…
```

**证据（界面）**：诊断 → 会话关联 → 「这台机器上的智能体」。
Codex 显示 `已安装 builtin` + 路径 + `acp · run · list · read · resume · send · terminal`；
DeepSeek Harness `已安装 terminal` + `run · resume · send · terminal`；未安装的九个仍在表上。
`POST /agent_fleet` 经窗口传输返回 200、`runner: pnpm dlx`、`agents: 9`（截图见
`F:\Temp\khor-try\shot-fleet2.png`）。

测试：`agent_fleet` 5 个单元测试（含「按安装目录查找」——第一版在 Windows 上漏了 `.exe` 后缀，
正是这条测试抓出来的）；`khor-gui-core` 的 `command_tables` 4 过（桥接与 Tauri 两张表一致）；
gui-core `agents` 12 过；前端 `tsc --noEmit` 干净。

### 2.4 列表默认按项目分组（提交 `586043a7`）

Windows 上列表打开时按 `place` 分组（"哪个项目在忙"是这台机器上先要回答的问题），
macOS/Linux 仍是 `recent`。只改**没有偏好时的默认**：`storedArrange` 仍返回人选过的值。
证据：同一批行在改动前是平铺的，改动后列表出现 `∨ F:\#Gong\#AI4S\khor`（8 条）与 `∨ 没有目录` 两个分组头。

### 2.5 产物与安装包（本轮最终构建）

用当前 HEAD 重建：`Khor_0.2.0_x64_en-US.msi` / `..._zh-CN.msi`（各 49.72 MiB）、
`Khor_0.2.0_x64-setup.exe`（36.12 MiB）。
release 二进制 PE 子系统 = **2（GUI）**，启动后无控制台、单标题栏、列表按项目分组，
日志文件收到 `khor: window transport on 127.0.0.1:1432 (4 workers)`。
`scripts/windows-installer-check.ps1` → **6/6 PASS**（install 6059 ms、layout 89、start 2100、stop 711、uninstall 2222、removed 57；既有的 `logs`、`runtime` 保留）。

### 2.6 DSH 通道：已用实验验证（未提交代码，结论先行）

在隔离 `DSH_HOME` 下实跑 `dsh --profile headless --json`：

```
{"type":"session","sessionId":"session-8e39627a-…","cwd":"F:\\Temp\\dsh-exp\\ws"}
{"type":"status","phase":"turn_start","turn":1}
{"type":"final","sessionId":"session-…","reason":{"kind":"error","error":{"code":"MISSING_CREDENTIAL"}},"text":""}
```

- 第一条事件就带 **sessionId + cwd**，会话按项目目录持久化到 `<home>/sessions/<编码路径>/session-<uuid>/`；
- 用 `--session-id <同一个 id>` 再跑：回同一个 id，且 **`"turn":2`** —— 证明接着上一轮历史继续；
- 不存在的 id → 结构化 `{"type":"error","message":"session … does not exist"}`，退出码 1；
- 失败（隔离 home 无凭据）也是结构化事件，不是崩溃。

**未测**：带真实凭据的完整一轮；接管 DSH 桌面创建的会话；与运行中的 DSH 桌面并发写同一会话。

## 3. 调研结论：接入面分档（"无论什么架构"的答案）

| 层级 | 形态 | 例子 | 需要 npm |
| --- | --- | --- | --- |
| 1 | 原生 ACP 模式 | `gemini --acp`、`opencode acp`、Copilot CLI、Goose、Cline | ❌ |
| 2 | ACP 适配器（注册表 22 个可启动） | `npx -y @agentclientprotocol/codex-acp` | ✅ |
| 3 | Khor 内置 shim | `_codexagent` → `codex app-server`、`_cagent` → `claude --print` | ❌ |
| 4 | 原生 CLI 结构化输出 | DSH `headless --json --session-id`、Codex `resume`/`queue` | ❌ |
| 5 | HTTP 服务器 | `opencode serve`（OpenAPI 3.1）、DSH web | ❌ |
| 6 | PTY 兜底 | 任何 CLI | ❌ |

**同一段对话能否两端同时读写**：手机能遥控桌面，是因为**桌面是唯一写者、手机只是它的一个"脸"**
（`chat_say` 在 `api::answer` 里，与 Windows 的 `?bridge=` 走同一张表；原生 TUI 走 `term_key` 按键中转）。
两个独立进程各自作为写者写同一个 Codex rollout 不行——Codex 对已装载 thread 持独占 OS 锁
（`~/.codex/thread-writer-locks/<id>.lock`），khor 只做共享探针并明确拒绝。DSH 则不同：它的 web face
是带认证的多客户端服务器，所以 khor 可以作为客户端接入——那是真正的"同时"。

## 4. 参考对象：CC Switch 做了什么、不做什么

CC Switch（MIT，桌面三平台）管 **13 个工具**，方式是**写各 CLI 的原生配置文件**
（`~/.claude/settings.json`、`~/.codex/config.toml`、`~/.gemini/.env`…），提供 90+ 供应商预设、
一键切换、本机路由（协议转换）、聚合模型列表、MCP/Skills/提示词管理。
它是**配置层**产品，不是会话层。Khor 的定位不同：**发现 + 会话 + 跨设备**。
因此第一阶段**不做**供应商配置切换；那张「发现表」的思路被借用到了 `agent_fleet` 的清单里。

## 5. 尚未开始（后续切片）

- **B 项目化会话**：Windows 默认按 `place` 分组；新建向导默认 Khor 对话；Codex 原生接管
  （`codex resume <id>` / `codex queue --thread <id> --message`）；DSH 适配器
  （读 `session.v4.jsonl.zstd` + headless 驱动）。
- **D 两机互联**：Windows ↔ LAB 配对与跨机会话。
- **E 启动性能**：`session_create` 6.4 s 的四项改造（本机 `ls` 跳过 endpoint、契约跨进程缓存、
  命中后不重复 inspect、已验证载荷不重复哈希）。

## 6. 已知限制

- 「已安装」= 程序在 PATH 或已知目录里；**不代表**能跑（未登录、缺运行器等要起的时候才知道）。
- 清单里的路线与能力是**本阶段代码能做到的**，不是厂商能做的一切。
- DSH 的会话列表/读取尚无适配器，所以行上没有 `list`/`read`；不要把它读成"不能读"。
- 修复只保证今后会话不被误杀；`Failed` 的旧行不会自愈（截图里那些就是修复前的遗留）。

## 7. 复现方式

```powershell
# CLI（debug 构建，见 F:\Temp\khor-check\debug\khor.exe）
khor agents fleet

# 界面：release 或任意 GUI 构建
# 左上「…」→ 设置 → 诊断 → 会话关联 → 「这台机器上的智能体」

# 诊断日志（无控制台时）
Get-Content "$env:LOCALAPPDATA\Khor\logs\gui.log" -Tail 20
```
