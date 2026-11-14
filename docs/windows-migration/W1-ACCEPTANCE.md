# Windows W1 构建底座验收

## 1. 结论

W1 本地构建底座已在 Windows 11 x64 上真实跑通：

- 前端生产构建成功，生成 `apps/gui/dist/index.html`；
- `khor` CLI 成功构建；
- `khor --version` 与 `khor doctor --json` 可执行；
- CI 基线已核对，远端 CI 未执行，标 NOT_RUN。

本文件是 W1 本地证据；不宣称 Windows 全功能完成，也不替代远端 CI 或用户验收。

## 2. 环境

| 项目 | 值 |
| --- | --- |
| OS | Windows 11 专业版 10.0.26100 x64 |
| Rust | rustc 1.100.0 (x86_64-pc-windows-msvc) |
| Cargo | cargo 1.100.0 |
| MSVC | Visual Studio 2022 BuildTools，MSVC 14.44 |
| NASM | 3.02 |
| Node | 24.21.0（DSH bundled）；仓库 CI 声明 Node 22.19 |
| pnpm | 11.28.5 |
| 构建路径 | `F:\khor-build`（无 `#` 字符；见 §5） |

`rustc -vV`：

```text
rustc 1.100.0 (0ef1bc386 2026-11-09)
host: x86_64-pc-windows-msvc
LLVM version: 23.1.5
```

## 2.1 最新 main 基线

分支已 rebase 到远端最新 main：

```text
origin/main: 9a4219ffe5e3ceba1f09cd3288cb0e27f91613d7
rebased HEAD: 6201b91e4dab0a292df1770dcab5c1e15b03d76a
```

在无 `#` 的新构建 worktree `F:\khor-build2` 基于该 HEAD 重新执行：

- frontend `gen_ts` / `gen_bindings`：exit 0
- `tsc --noEmit`：exit 0
- `vite build`：exit 0，`dist/index.html` 已生成
- `cargo build --locked -p khor-cli --bin khor`：exit 0
- Windows process harness：12/12 PASS
- fanout harness：9/10（同一顺序相关 flake），不是稳定失败

远端 CI 需要在 rebase 后重新运行；旧 run 43557690271 是 rebase 前 commit 03d861ca 的证据，不能单独代表最新 main。

## 3. 前端构建

### 3.1 依赖

在 `F:\khor-build\apps\gui`：

```text
CI=true pnpm install --frozen-lockfile --shamefully-hoist
exit 0
```

注：pnpm 提示 `esbuild` postinstall 被忽略；平台包 `@esbuild/win32-x64` 已存在，Vite 构建可用。

### 3.2 生成与构建

```text
mkdir src/gen
cargo run --locked -q -p khor-catalog --bin gen_ts > src/gen/catalog.ts
cargo run --locked -q -p khor-gui-core --bin gen_bindings
pnpm exec tsc --noEmit
pnpm exec vite build
```

结果：

```text
gen_ts: exit 0；catalog.ts 663,010 bytes
gen_bindings: exit 0；bindings -> apps/gui/src/gen/bindings
tsc --noEmit: exit 0
vite build: exit 0，47.21s
dist/index.html: 7,102 bytes
dist/index.html sha256: 5446B67D5A227EFE5D3A22470FBF55995B3ACE6CF42A177D93323675AA1CD280
```

Vite 产物包含 `dist/assets/*`（含 mermaid/elk 等大 chunk），构建成功。

## 4. CLI 构建

在 `F:\khor-build`，使用 vcvars64 + NASM + `cargo`：

```text
cargo build --locked -p khor-cli --bin khor
exit 0
```

产物：

```text
F:\khor-target-build\debug\khor.exe
sha256: 050AEA2FE83C44AED412EA0134230C6B476995B7F9070F2E7031020770A88789
```

运行：

```text
khor --version
0.2.0
exit 0
```

## 5. 路径发现：`#` 字符

原 worktree 路径 `F:\#Gong\#AI4S\...` 含 `#`，Vite 7 明确警告并构建失败：

```text
The project root contains the "#" character ... may not work when running Vite.
```

因此 W1 在无 `#` 的独立构建 worktree `F:\khor-build`（同一 commit a425166f）完成。  
该问题是真实 Windows 工具链/路径约束，不是源码逻辑错误；后续开发建议使用无 `#` 的构建路径或给 Vite 提供稳定别名。

## 6. 诊断入口

```text
khor doctor --json
exit 0
```

输出 14 项检查，覆盖 machine/network/tools；样例见 [W1-DIAGNOSTICS](W1-DIAGNOSTICS.md)。  
当前 `serve` 为 fail（未启动）、web 未监听；这是预期基线，不是构建失败。

## 7. CI 基线

已核对 `.github/workflows/quality.yml` 存在 `windows-agent-process` job（windows-2025 + PowerShell + Windows process harness/fanout）。  
远端 GitHub Actions 未在本轮执行，状态 NOT_RUN。

### 7.1 本地 Windows process harness

按仓库命令执行：

```text
cargo run --locked --manifest-path crates/node/tests/windows-process-harness/Cargo.toml
exit 0
```

全部 12 项 PASS，包括：

```text
PASS argv / UTF16 / env-clear / removed-env / cwd / stdin / stdout / stderr / exit-status
PASS bare program from explicit PATH with node + host.ts arguments
PASS invalid program / missing PATH / batch / embedded NUL rejected
PASS job termination after leader exit kills pipe-owning descendant
PASS dropping a live process kills leader and descendant via owned job
PASS independent I/O cancellation unblocks full stdin while child is still alive
PASS independent I/O cancellation unblocks both reads while child holds output pipes
PASS supervisor deadline progresses through blocked stdin and repeated Stop
PASS only the MCP worker profile permits explicit durable breakaway; temporary child is terminated
PASS runtime lease is in HANDLE_LIST and survives parent-side lease drop
PASS 10 Windows runtime groups
```

### 7.2 本地 fanout harness

```text
rustc --edition=2024 --test crates/node/tests/fanout_harness.rs -o F:\Temp\gui-fanout-tests.exe
exit 0

F:\Temp\gui-fanout-tests.exe --nocapture --test-threads=1
9 passed; 1 failed
```

失败测试：

```text
fanout::tests::a_history_producer_waits_for_real_capacity_without_retiring_the_reader
```

隔离重跑该测试为 `ok`，说明是**顺序相关的 Windows flake**；远端 windows-2025 CI 的同一套 fanout 测试为 **10/10 PASS**，因此不构成 W1 阻断。完整产物见：

```text
F:\khor-w1-evidence\process-harness.err
F:\khor-w1-evidence\fanout-*.err
```

### 7.3 远端 GitHub Actions

独立分支 `steward/s2-windows-20261114` 已推送并触发 Windows W1 workflow：

- Run: https://github.com/GongZhengxin/khor/actions/runs/43557690271
- Head: `03d861ca2399724313cefcbe29c526b6e0f899bc`
- 结论：**success**
- 全部步骤通过：Frontend build、CLI build、CLI version、doctor JSON、Windows process harness、fanout build、fanout run（10/10）。

远端 CI 是真实 `windows-2025` runner，不是本地模拟；日志保存在：

```text
F:\khor-w1-evidence\ci-run2.log
```



## 8. 已知 Windows 限制（W1 显式保留）

为使 `khor-node` 在 Windows 编译，以下能力暂为显式不支持/安全降级；均返回结构化错误或保守行为，不是静默失败：

- `khor serve` keeper（需要 POSIX exec/信号转发）：返回 unsupported；
- `khor quit` 停止 serve/host（POSIX 信号）：返回 unsupported；
- tabs 写操作（flock）：返回 unsupported；`tabs list` 可读；
- attach 原始终端（termios）：Windows 返回 unsupported；
- secret askpass 父进程检测：Windows 返回 false（保守）；
- `foreground_group`/`tty_path`：Windows 返回 None；
- 进程退出 CRLF/CRT 清理与 Unix 不完全相同。

这些属于 W2+ 的 Windows 适配工作，不应在 W1 被当作“全功能完成”。

## 9. 验收判定

- 前端生产构建：**PASS（本地）**
- CLI 构建：**PASS（本地）**
- `khor --version`：**PASS**
- `khor doctor --json`：**PASS（基线）**
- Windows process harness：**PASS（本地 12/12；remote 12 PASS）**
- Windows fanout harness：**PASS（remote 10/10；本地曾出现 1 次顺序相关 flake）**
- Windows CI 远端执行：**PASS（run 43557690271，success）**
- Windows 10 兼容验证：**NOT_RUN（按用户决定延后）**
- 全功能 Windows 等价：**NOT CLAIMED**

W1 验收：**PASS（本地 + 远端 Windows CI 构建底座；Win10 与全功能等价不在 W1）**。  
后续 W2+ 需继续实现 Windows stubs 背后的真实能力。


