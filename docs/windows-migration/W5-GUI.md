# W5：GUI / Web 桌面端

## 1. 目标与验收门

见 [ROADMAP](ROADMAP.md) §1 W5：Tauri 窗口在 Windows 启动、连接本地/远程节点、WebView2 正常；
前端构建在 Windows 可重复。验收门：冷启动到可交互、打开会话、终端面板可用，附截图/日志。

## 2. 环境约束

1. **路径不能含 `#`**：Vite 7 拒绝含 `#` 的工程根。本轮用无 `#` 的 worktree：
   ```powershell
   git worktree add -b w5-gui-check F:\Temp\khor-gui dbc30156
   ```
2. **本机没有 npm**（只有 node 与 pnpm）。GUI 的脚本写的是 `npm run gen`，且依赖 npm 的**提升布局**：
   `src/lib/highlight.ts` 直接 import `@shikijs/langs/*`、`@shikijs/themes/*`，`src/lib/md/file.ts` import `hast`，
   而 `package.json` 只声明了 `shiki`、`hast-util-*` —— 这些是 npm 提升出来的**幻影依赖**。
   pnpm 默认隔离布局解析不到，必须：
   ```powershell
   pnpm install --frozen-lockfile --node-linker=hoisted --shamefully-hoist --store-dir F:\khor-toolchain\pnpm-store
   node scripts/generate.mjs
   node_modules\.bin\tsc.cmd --noEmit
   node_modules\.bin\vite.cmd build
   ```
   注意：`apps/gui/.npmrc` 在本机 pnpm 11 里未生效（`pnpm config get node-linker` 仍为 undefined），
   所以用命令行参数；该文件只建在一次性 worktree，不进仓。

## 3. 结果（2026-11-15，Windows 11 x64）

| 步骤 | 命令 | 结果 |
| --- | --- | --- |
| 依赖 | `pnpm install --frozen-lockfile --node-linker=hoisted --shamefully-hoist` | 完成（558 包；esbuild 的构建脚本被 pnpm 拦下，但 vite 可用） |
| 生成 | `node scripts/generate.mjs` | exit 0，生成 `src/gen/bindings` |
| 类型 | `tsc --noEmit` | **exit 0** |
| 前端 | `vite build` | **exit 0**，34.68s，`dist/`（主 bundle 2,929.10 kB） |
| Tauri | `cargo build --locked -p khor-gui`（`CARGO_TARGET_DIR=F:\Temp\khor-target`） | **exit 0**，5m09s，`khor-gui.exe` |
| 冷启动 | `khor-gui.exe`，`KHOR_HOME`/`KHOR_RUNTIME_DIR` 指到 `.cache/w5` | 窗口出现：标题 Khor、会话侧栏、搜索、机器行「DESKTOP-1CC5MUC 本机」；WebView2 数据目录已建 |
| 会话可见 | 同 home 下 `khor open -d -- cmd.exe` → `shell/281f2d7c1c2f1718` | 会话落盘成功，但 GUI 列表 60s 内仍是加载骨架，未渲染该行 |

截图：`.cache/w5/screen.png`（首次冷启动）、`.cache/w5/screen2.png`（建会话后）。

## 4. 未通过 / 未测

| 项 | 状态 | 说明 |
| --- | --- | --- |
| 会话列表渲染 | **未通过** | GUI 与本地节点的握手未完成，列表停在骨架；需查 GUI 侧启动的本机节点（进程存在、store 有 endpoint.json，但列表不更新） |
| 打开会话 / 终端面板 | **NOT_TESTED** | 依赖上一条；终端还依赖 ConPTY 输出（见 [W3 T5](W3-SMOKE.md)），本会话环境里读取会阻塞 |
| `npm ci` 路径 | **NOT_TESTED** | 本机无 npm；CI 用 npm，需在 CI 复核幻影依赖仍能通过 |
| 幻影依赖修复 | 未做 | 应在 `package.json` 声明 `@shikijs/langs`、`@shikijs/themes`、`@types/hast` 并重生成 `package-lock.json`（需要 npm，本机没有） |

## 5. 结论

- **构建链在 Windows 上可重复**（在 pnpm hoisted 前提下）：generate / tsc / vite / Tauri 全部 exit 0。
- **GUI 冷启动与渲染通过**：真实窗口、WebView2、侧栏与机器行都出现。
- **端到端未通过**：列表不渲染会话，终端面板无法验证。W5 判为 **未完成**，下一步先查 GUI 侧节点握手。
