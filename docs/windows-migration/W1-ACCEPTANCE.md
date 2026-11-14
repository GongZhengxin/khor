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

隔离重跑该测试为 `ok`，说明是**顺序相关的 Windows flake**，不是稳定失败。完整产物见：

```text
F:\khor-w1-evidence\process-harness.err
F:\khor-w1-evidence\fanout-*.err
```

结论：process harness PASS；fanout 本地 CI 等价运行 not green（1 flake），W1 不能声称 CI 全部通过。详见 [W1-CI](W1-CI.md)。


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
- Windows process harness：**PASS（12/12）**
- Windows fanout harness：**FAIL（9/10，1 个顺序相关 flake；隔离重跑通过）**
- Windows CI 远端执行：**NOT_RUN**
- Windows 10 兼容验证：**NOT_RUN**
- 全功能 Windows 等价：**NOT CLAIMED**

建议 W1 以“本地构建底座通过、process harness 通过、fanout 待修、远端 CI 与 Win10 待补”为验收范围；是否把 fanout flake 作为 W1 阻断项，需要用户确认。

