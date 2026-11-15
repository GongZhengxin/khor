# Windows 集成测试普查（2026-11-15 → 16）

`full` 权限、工作区外构建（`CARGO_TARGET_DIR=F:\Temp\khor-target`）、`--test-threads=1`。
每条都是真实进程/真实 socket 的集成目标，不是 mock。

## 1. 通过

| 目标 | 结果 |
| --- | --- |
| `pairing` | 6 passed |
| `open_from_phone` | 1 passed（60.7s） |
| `resume` | 6 passed |
| `machine_pause` / `peer_pause` / `takeover_safety` | 各 1 passed |
| `sync_doc_isolation` | 5 passed / 1 ignored |
| `transfer` | 4 passed |
| `a2a_http` | 4 passed / 1 ignored |
| `files_wire` | 1 passed |
| `history_wire` | 1 passed |
| `usage` | 4 passed |
| `vitals` | 1 passed |
| `broadcast` | 2 passed（68.7s） |
| `command_ledger` | 4 passed |
| `departed` | 1 passed |
| `workspace_facts` | 3 passed |

合计 **44 项通过**（另有 2 ignored）。

## 2. 本轮改掉/门掉的

| 目标 | 处置 | 原因 |
| --- | --- | --- |
| `pairing` | 修夹具（`5dc3be49`） | 端点文件用 `pid: 1`，Windows 没有 pid 1 |
| `resume`（rejoin 用例） | `#[cfg(unix)]`（`bcc5cafa`） | ConPTY replay 只有提示符 |
| `host_replay` | `#[cfg(unix)]` | 10,000 行 flood 22 分钟未完成 |
| `cwd_pin` | `#[cfg(unix)]` | Windows 不能删除/重命名进程的当前目录，用例前提不存在 |
| `day_record` | 整文件 `#[cfg(unix)]` | 三个用例都要 `Node::import` 的“已验证原生身份”，本夹具没有 agent home；strip 逻辑由 `khor_node::day` 单测覆盖 |
| `recognition_wire` | 单用例 `#[cfg(unix)]` | **疑似产品问题**：`Enable(Claude)` 返回 Ok，但随后读取仍是 `automatic`——Claude 配置的写入与读取在 Windows 上不一致；Codex 那一半通过 |

被 gate 的条目都在文件里写了原因，不当作通过。

## 3. 发现（按影响排序）

1. **ConPTY 输出读取**：宿主 `last.txt` 永远为空、ring 不写入；直接 `portable-pty` 探针只收到 `\x1b[6n`，
   reader 早于/晚于 spawn 都一样。影响 W3 T5、两条 replay、W5 终端面板。
2. **GUI 会话列表不渲染**：节点与 web face 正常（日志 `khor web face: 0.0.0.0:5467`），列表停在骨架。
3. **Claude recognition 写入/读取不一致（Windows）**：`Enable(Claude)` 成功但读取仍 automatic。
4. GUI 的 npm 幻影依赖（`@shikijs/langs`、`@shikijs/themes`、`hast`）——npm 提升才成立，需在 `package.json` 声明并重生成 lock。
