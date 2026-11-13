# Windows 迁移 W0 基线

本报告基于 `task-2 / revision 3 / attempt 9` 的隔离工作树，源码基线为 `97bbf2a4e036f1a334c9921961d3d0b283f46c59`。结论：已核对真实 Cargo、前端、Windows 专项 CI 与候选发布入口；**本轮尚无产品构建或测试成功的执行证据，不能据此宣布 Windows 全量可用**。下文明确区分 Host 状态回执、源码事实和待执行项。

## 1. 隔离、HEAD 与修改边界

| 项目 | 观察结果 | 证据与限制 |
| --- | --- | --- |
| 分配的工作树 | `C:\Users\admin\AppData\Local\Temp\steward-wt-3e084e4e\c945f53c-d6c3-54f3-b4d4-c7ab50a852a2` | 当前 runtime context 与 `steward_status(task-2)` 一致；这是 Host 分配的隔离工作树 |
| 分支 | `steward/c945f53c-d6c3-54f3-b4d4-c7ab50a852a2` | Host task 状态；没有自行创建分支或 worktree |
| 起始 HEAD / base | 均为 `97bbf2a4e036f1a334c9921961d3d0b283f46c59` | Host `baseline_observed`，以及本轮 `steward_workspace diff` 的 base/head |
| 起始 clean | `true` | Host marker，指纹 `aac3f2d747f130e2663458cdbddbfaca1f077c3407477602d4147b0518aca6a4`；写入前 diff 为空 |
| 主工作树状态 | 未独立检查 | 没有在原工作目录运行命令；不能将隔离树 clean 推广为主树 clean |
| Git 实际 common-dir、顶层根、remote、默认分支 | 未执行 Git 命令核验 | CI 的 `main` 触发条件仅是配置事实，不能代替远端 HEAD 或默认分支观测 |
| 唯一允许修改 | [本报告](<BASELINE.md>) | 不修改产品源码、manifest、锁文件或 CI；提交 hash 以 Host commit 回执和结构化 handoff 为准 |

已加载 using-git-worktrees、repository-scout、systematic-debugging、rust-engineering、typescript-react、tauri-desktop、windows-systems、ci-build-release、debugging-qa 方法。工作树、命令、自检、提交均服从 Host 契约，方法中的自动安装、创建工作树等示例不构成本轮授权。

本轮可用角色工具只有受限的工作区读写/提交、状态读取、方法读取和 Host 自检；没有可调用的 shell/exec。因而 `pwd`、Git 与工具版本命令均未执行。Host 后续又明确返回 `Context exploration budget exhausted (24 read/list operations)`；被拒绝的读取不计作已读。运行时文件策略为 `danger-full-access`，不是操作系统级只读沙箱；本轮的工作树与写范围由角色工具控制，隔离强度最多按 **partial** 陈述，不标为 full。

## 2. 真实构建入口与工具链要求

### Cargo 与前端的依赖关系

根 [Cargo manifest](<../../Cargo.toml#L1>) 是 resolver 2 workspace，包含 16 个成员：13 个 [crates](<../../crates/>) 加 GUI core、native-files 和 src-tauri；排除 vendor。工作区版本 `0.2.0`、edition `2024`；[工程约定](<../code-quality/ENGINEERING.md>) 另指出 ACP 使用 2021，不能将继承规则概括成全仓均为 2024。根目录列举确认 [Cargo 锁文件](<../../Cargo.lock>) 存在，本轮未读取其解析图。

[CLI manifest](<../../crates/cli/Cargo.toml#L1>) 的 package 是 `khor-cli`，二进制是 `khor`，入口为 `src/main.rs`；依赖 `khor-commands` 和 `khor-web`。因此精确构建入口为 `cargo build --locked -p khor-cli --bin khor`，不是猜测 package 名为 khor。

[README](<../../README.md#L9>) 要求先构建 GUI 前端；[Web build script](<../../crates/web/build.rs#L1>) 证实会检查 GUI 的 `dist/index.html`，读取 `/assets/index-*.js`，生成 `UI_REVISION` 与嵌入资产表。缺少前端入口文件会主动 panic；仅创建空目录不能满足前置条件。脚本按运行时 `CARGO_MANIFEST_DIR` 定位资产，并将 Windows 路径分隔符转换为 URL 的 `/`。这是源码事实，本轮未触发该 panic，也未测量资产指纹。

[GUI package manifest](<../../apps/gui/package.json#L7>) 的真实生命周期如下：

```text
工作目录：apps/gui
npm run build
  prebuild -> npm run gen
  gen -> mkdir -p src/gen && cargo run --locked -q -p khor-catalog --bin gen_ts > src/gen/catalog.ts && cargo run --locked -q -p khor-gui-core --bin gen_bindings
  build -> tsc --noEmit && vite build
npm test
  pretest -> npm run gen
  test -> npm run test:unit && npm run test:types
npm run test:types
  -> tsc --noEmit
npm run test:interaction
  -> manifest 中列出的交互检查链
```

因此前端 build 并非纯 Node 命令，生成阶段也需要 Rust。`gen` 中的 `mkdir -p` 和重定向是待验证的 shell 假设；**本轮未在 Windows shell 复现失败**，不将其列为已知失败或永久阻塞。生成命令可能写第一方生成文件，后续必须在允许这些生成输出的隔离验证范围运行；本轮文档写权限不涵盖这些修改。

[GUI Rust manifest](<../../apps/gui/src-tauri/Cargo.toml#L1>) 的 package 为 `khor-gui`，默认 features 为 `custom-protocol`、`devtools`；仓库注释说明桌面可直接 `cargo build -p khor-gui`。显式开发资产模式是 `--no-default-features`。开发验收时应记录是否使用该模式，不能混淆嵌入资产与 Vite 服务。

[Tauri 配置](<../../apps/gui/src-tauri/tauri.conf.json#L1>) 指向 `../dist`，开发地址 `http://localhost:1430`，beforeBuild 为 `npm run build`，**bundle.active 当前为 false**。这只是仓库现状，不代表本轮关闭打包，也不构成 Windows 安装包已验证的证据。manifest 在桌面 Windows/Linux/macOS 上纳入 updater/runtime，macOS 的 proxy feature 单独 target-gated；仅从这些声明不能断言运行时能力已验证。

[Agent package manifest](<../../apps/agent/package.json#L7>)：`npm test` 实际为 `tsc --noEmit && node --test test/*.test.ts`，build 为 `node build.mjs`。[Agent 构建脚本](<../../apps/agent/build.mjs#L1>) 先用当前 Node 执行 provider 生成，再调用 Bun compile；目标表明确包含 `x86_64-pc-windows-msvc -> bun-windows-x64-baseline`。目标表存在不等于已生成、运行或发布 Windows sidecar。

### 声明版本与本机版本分开记录

| 工具/平台 | 读取到的要求 | 本轮本机实测 |
| --- | --- | --- |
| Rust | [质量 CI](<../../.github/workflows/quality.yml>)、[前端 CI](<../../.github/workflows/frontend.yml>)、[native-tools](<../../.github/actions/native-tools/action.yml>) 使用 `1.95.0` | `rustc -vV`、`cargo -V`、`rustup show` 未执行；不宣称已安装该版本 |
| rustfmt / Python | [工程约定](<../code-quality/ENGINEERING.md>) 指定 rustfmt `1.9.0-stable`、Python `3.11+`，统一格式入口 `python3 scripts/check-rust-format.py --check` | 均未执行版本探测 |
| Node | 上述 CI 指定 `22.19.0` | `node --version` 未执行；契约 `{node}` 的实际解析路径未取得 |
| Bun | native-tools 与 agent CI 指定 `1.3.14` | `bun --version` 未执行 |
| npm | 已确认 GUI、agent 各有 package-lock；CI 使用 `npm ci` | npm 版本未探测；本轮禁止安装依赖，未运行 npm ci |
| Windows | workflow policy 为 `win32`，隔离路径为 Windows 路径；专项 CI runner 为 `windows-2025` | Windows 10/11 的版本、build、架构、MSVC linker、SDK、WebView2 未探测；runner 名称不能代替客户端兼容证据 |

## 3. 已读取的 CI 与精确命令

[前端 workflow](<../../.github/workflows/frontend.yml#L1>) 在 Ubuntu 24.04 上先运行版本门，再安装和构建 GUI，保存 dist、生成 bindings 和 Lab 数据，并生成 frontend manifest。关键原文命令：

```text
仓库根：
python3 scripts/release_manifest.py version
npm ci --prefix apps/gui
npm run build --prefix apps/gui
python3 scripts/khor_lab/fixture_data.py .
python3 scripts/release_manifest.py create --root frontend --kind frontend
```

[质量 workflow](<../../.github/workflows/quality.yml#L1>) 包括 scripts、agent、node-native、GUI、mac 更新/原生 UI、Windows agent process 等 jobs，不是一个可等同于全部质量门的单命令。已读取的代表性精确入口：

```text
仓库根：
python3 -m unittest discover -s scripts/tests -p 'test_*.py'
node scripts/check-native-io.mjs
cargo build --locked -p khor-cli --bin khor --message-format=json
cargo test --locked -p khor-node --lib --no-run --message-format=json
cargo test --locked -p khor-node --lib acp_process::tests -- --test-threads=1
cargo test --locked -p khor-gui-core --test terminal_current -- --test-threads=1
cargo test --locked -p khor-cli --test runtime_image -- --test-threads=1

apps/gui：
npm run test:unit && npm run test:types
npm run gates && npm run mobile:static

apps/agent：
npm test
node build.mjs x86_64-unknown-linux-gnu <隔离输出路径>
```

node-native CI 使用私有 `CARGO_TARGET_DIR`、runtime 目录、真实 CLI/Agent 和 provenance prepare/verify，再用独立进程运行验收。上述摘录省略了 job 的环境和准备步骤，**不可直接当作已具备前置条件的完整本机运行脚本**。GUI 交互依赖 Chromium/WebKit 等浏览器；mac/iOS jobs 依赖 Xcode/模拟器，本轮均未执行。

Windows 专项 job 的真实命令为：

```powershell
# 仓库根；后续由 Host 在允许构建的隔离环境执行
cargo run --locked --manifest-path crates/node/tests/windows-process-harness/Cargo.toml
rustc --edition=2024 --test crates/node/tests/fanout_harness.rs -o "$env:RUNNER_TEMP/gui-fanout-tests.exe"
& "$env:RUNNER_TEMP/gui-fanout-tests.exe" --nocapture --test-threads=1
```

已读取 [Windows harness manifest](<../../crates/node/tests/windows-process-harness/Cargo.toml#L1>)：独立 `[workspace]`，edition 2024，`publish=false`，依赖 khor-runtime 与固定 `windows-sys =0.61.2`，features 涉及 JobObjects、Pipes、Threading 等。harness 实现和 fanout 源码本轮未读，不推断具体断言数量。CI 明确保留原始退出码并上传 process/fanout 日志；这些是**配置中的验证范围**，本轮没有对应 GitHub job 的运行回执。

[候选发布 workflow](<../../.github/workflows/release.yml#L1>) 的 CLI/native 与 Agent runtime 矩阵均列出 Linux x86_64/aarch64 和 macOS arm64/x86_64，另有 mac App、iOS archive；**已读矩阵未列出 Windows**。原生 CLI 使用 `cargo build --locked -p khor-cli --bin khor --target "$TARGET" --release`，Linux 分支用 cargo zigbuild；发布资产与共享 frontend/runtime manifest 关联。Windows 专项 job 存在并不补齐 Windows 发布产物。未查询线上 releases、未下载产物，故不对“历史上从未发布 Windows”作断言。[分发 workflow](<../../.github/workflows/distribute.yml>) 已列举、未读；本轮不运行候选构建、签名、公证或发布。

## 4. 执行记录、退出结果与环境边界

| 操作 | 本轮实际结果 | 能证明什么 |
| --- | --- | --- |
| `steward_workspace map` | 成功；rev 与基线一致，总文件数 2412，仅返回前 120 项且 truncated=true | 有界目录索引，不能视作全仓已读 |
| `steward_workspace diff`（写入前） | 成功；base=head=基线，diff 为空 | 配合 Host clean marker 记录起始状态；不是 shell `git status` 回执 |
| 本轮文中标为已读的文件/目录 | 角色工具成功返回内容 | 仅建立静态事实，工具未提供进程退出码 |
| 最后的扩展读取批次 | 被 Host 的 read/list 配额拒绝 | 未取得 node/runtime/update manifests、secret、Windows process/harness 实现、根 gitignore、pre-commit 内容；不伪装为已读 |
| 提交前调用 `steward_selfcheck` | Host gate 拒绝：必须先 write + commit | 没有启动测试进程，没有测试退出码；提交后仍需运行正式自检 |
| 产品构建/测试/格式检查 | **NOT_RUN** | 当前角色没有通用命令工具；不得以读源码或空命令策略替代执行证据 |
| 最终提交与提交后自检 | 由 Host commit/selfcheck 回执及 handoff 记录 | 自检是否实际运行产品命令，必须检查 receipt 的 command 列表；空列表不能算 cargo/npm 已通过 |

契约 acceptance 原文如下，超时 `600000 ms`：

```text
{node} "F:\#Gong\#AI4S\khor\meta-control\scripts\cargo-acceptance.mjs" test
```

该脚本位于分配工作树之外；本轮未读其内容、未探测是否存在、未解析 `{node}`，也无法确认其子命令、cwd、target、环境变量或测试数。workflow 的 TRIVIAL self_check 配置为 `commands: []`，与契约 acceptance 是两个不同字段。**不得把自检空命令成功视为该 acceptance 已执行。** 需要 Host 在隔离验证范围运行并保留准确命令、cwd、工具版本、退出码、stdout/stderr、测试数和最终 clean 回执；若工具没有调度该命令，则明确保留 NOT_RUN。

本轮没有发生已复现的产品测试失败，因而没有所谓“已修复”的失败。缺少 dist 的主动检查、Unix shell 假设、Windows 发布矩阵缺口均按各自证据范围记录。以后出现 Unix-only import、外部工具或平台能力缺失，应区分代码适配项与 BLOCKED_ENV；`#[cfg(unix)]` 排除的测试在 Windows 上计为未运行，不能计入通过数。不能把一次 Unix 假设失败直接升级为迁移永久阻塞。

## 5. 功能矩阵可使用的目录与证据边界

本节提供后续矩阵的勘察入口，**不据目录名判定 REQUIRED、已发布或 Windows 已支持**。

| 领域 | 已确认存在的入口 | 本轮证据深度与后续核验 |
| --- | --- | --- |
| CLI / 命令 | [CLI](<../../crates/cli/>)、[commands](<../../crates/commands/>)、[catalog](<../../crates/catalog/>) | CLI manifest 已读；命令实现、帮助输出、用户旅程未运行 |
| GUI / Web | [GUI](<../../apps/gui/>)、[GUI core](<../../apps/gui/core/>)、[Tauri](<../../apps/gui/src-tauri/>)、[Web](<../../crates/web/>) | GUI/Tauri manifests 与 Web build script 已读；[GUI 源码](<../../apps/gui/src/>)、[GUI 检查脚本](<../../apps/gui/scripts/>) 待按行为深入 |
| Runtime / Agent | [runtime](<../../crates/runtime/>)、[Agent](<../../apps/agent/>)、[node runtime image](<../../crates/node/src/runtime_image/>) | Agent manifest/build script 已读；runtime manifest 与平台加载实现未读 |
| 更新 / 安装 | [update](<../../crates/update/>)、[node updates](<../../crates/node/src/updates/>)、[发布脚本目录](<../../scripts/>) | 发布 workflow 已读；更新协议、替换/回滚、安装脚本内容未读，未修改实际安装 |
| 进程 / 管道 | [Windows process 模块](<../../crates/node/src/khor_agent_process_windows.rs>)、[Unix process 模块](<../../crates/node/src/khor_agent_process_unix.rs>)、[Windows harness](<../../crates/node/tests/windows-process-harness/>) | node 目录列举确认模块存在；仅 harness manifest 和 CI 命令已读；Job/取消/清理断言待验 |
| 终端 / 探测 | [node live](<../../crates/node/src/live/>)、[detect](<../../crates/detect/>)、[GUI term](<../../apps/gui/core/src/term.rs>) | 根 manifest 声明 portable-pty/vt100；GUI term 仅 map 索引，不证明 ConPTY、输入法、退出和重连可用 |
| 文件 / 工作区 | [node files](<../../crates/node/src/files.rs>)、[workspace](<../../crates/node/src/workspace.rs>)、[watch](<../../crates/node/src/watch/>)、[GUI files](<../../apps/gui/core/src/files.rs>)、[native-files](<../../apps/gui/native-files/>) | 目录/list/map 存在证据；路径、ACL、rename、长路径、保留名与监听行为待验 |
| 账号 / 凭据 | [accounts](<../../crates/node/src/accounts.rs>)、[secret](<../../crates/node/src/secret.rs>)、[agent accounts](<../../crates/node/src/agent_accounts.rs>) | 文件存在，正文未读；不得从命名推断 Windows credential store 或账户登录可用 |
| 网络 / 同步 / 协议 | [net](<../../crates/net/>)、[sync](<../../crates/sync/>)、[a2a](<../../crates/a2a/>)、[acp](<../../crates/acp/>) | workspace 成员和目录已核对；真实远程链路、网络策略与协议行为待验 |

[文档目录](<../>) 列举确认存在 [产品说明](<../KHOR.md>)、[安装说明](<../INSTALLATION.md>)、[发布说明](<../RELEASE.md>)、[会话说明](<../SESSION.md>)、[终端说明](<../TERMINAL.md>)、[工作区说明](<../WORKSPACE.md>)、[网络说明](<../NET.md>)、[Agent 说明](<../AGENT.md>) 等；这些正文本轮未读，应由功能矩阵结合实际实现与发布产物核对。不能用本文的静态入口表替代矩阵任务。

## 6. 待补的最小验证闭环

1. Host 在分配的隔离工作树/验证副本记录 cwd、`git rev-parse HEAD`、`git status --porcelain=v1`、顶层目录、git-dir/common-dir、superproject、分支、脱敏后的 remote 和默认分支；禁止 reset、clean、init。已提交文档后 HEAD 应与提交回执一致。
2. 记录 Windows 精确版本/架构以及 `rustc -vV`、`cargo -V`、`rustup show`、`node --version`、`npm --version`、`bun --version`、`python --version`，检查 linker/SDK/WebView2。工具缺失记录 BLOCKED_ENV，不安装或改变依赖决策。
3. 在已有依赖且允许生成输出的隔离范围先运行前端构建，再运行有来源的 CLI build、`cargo test --locked`、GUI unit/types、Windows process/fanout。完整命令及必要环境以已读 manifest/CI 为准；未具备浏览器、账户、设备的项明确 NOT_RUN。本轮不擅自运行 npm ci 或依赖安装。
4. 由 Host 核验并执行契约 acceptance wrapper；确认其真的针对隔离树、没有共享主树 target/符号链接污染，并保留完整结果。不得执行 `cargo clean`；[README](<../../README.md>) 对共享 target 的提醒是项目约定，本轮未用文件系统命令核实该链接在当前隔离树的实际形态。
5. 保留前端资产 revision、实际二进制路径/来源、工具版本、退出状态、实际通过/失败/忽略数及资源清理结果；最后确认只有本报告提交且工作树 clean。Windows 10/11 GUI、ConPTY、凭据、安装更新与资源/性能基线另需真实平台证据，不能用 windows-2025 的专项 CI 配置替代。

后续可继续同库渐进适配，但架构、依赖、签名、打包发布方案不在本报告中决策。本轮交付是可追溯且明确缺证的 W0 文档，构建/测试基线和环境实测仍须由具备命令执行能力的 Host/Verifier 补齐。
