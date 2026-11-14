# Windows W1 CI 基线

## 1. 现状

`.github/workflows/quality.yml` 已包含 Windows 专项 job：

```yaml
windows-agent-process:
  runs-on: windows-2025
  # PowerShell
  cargo run --locked --manifest-path crates/node/tests/windows-process-harness/Cargo.toml
  rustc --edition=2024 --test crates/node/tests/fanout_harness.rs -o "$env:RUNNER_TEMP/gui-fanout-tests.exe"
  & "$env:RUNNER_TEMP/gui-fanout-tests.exe" --nocapture --test-threads=1
```

同时 `.github/workflows/frontend.yml`、`quality.yml` 声明 Rust 1.95.0、Node 22.19.0。

## 2. 本轮验证

- 已读取 `.github/workflows/quality.yml`、`frontend.yml` 与 `.github/actions/native-tools/action.yml`；
- 本地按 Windows 等价路径完成：
  - 前端生产构建（tsc + vite）成功；
  - `cargo build --locked -p khor-cli --bin khor` 成功；
  - 诊断入口成功。
- **远端 GitHub Actions 未执行（NOT_RUN）**；本机不是 windows-2025 GitHub runner。

## 3. W1 判定

- CI 配置文件存在且 Windows job 可识别：**PASS（静态）**
- 本地构建等价路径：**PASS（本地）**
- 远端 CI 真实执行：**NOT_RUN**
- 不把静态 YAML 当作 CI 绿灯。

## 4. W2+ 待办

- 在真实 windows-2025 runner 上执行 `quality.yml`；
- 前端 job 与 Tauri build job；
- 让 CI 使用无 `#` 的构建路径或修正 Vite 路径假设；
- 记录 artifact hash 与 job URL。
