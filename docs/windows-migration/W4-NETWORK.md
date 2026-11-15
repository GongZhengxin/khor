# W4：设备网络 + Session 端到端

## 1. 目标与验收门

见 [ROADMAP](ROADMAP.md) §1 W4：两台机器（或 Windows ↔ 现有平台）的配对、连接、断线重连、
Session 创建/恢复/关闭、跨设备同步、远程控制与审批。

**验收门**：至少一次真实双机配对与一次断线重连的证据；Session 恢复后内容与审批状态一致。

## 2. 本机条件

只有一台 Windows 主机，因此"两台机器"用**两个独立 home 的两个 node，经真实 TCP 回环**代替；
物理双机验收需要第二台 Windows（用户决定）。所有命令都在工作区外构建（见 [W3-SMOKE](W3-SMOKE.md) §2）：

```powershell
$env:CARGO_TARGET_DIR='F:\Temp\khor-target'
cargo test --locked -p khor-node --test <target> -- --test-threads=1
```

## 3. 结果（2026-11-15，Windows 11 x64 + rustc 1.95.0）

| 目标 | 结果 |
| --- | --- |
| `pairing`（邀请/加入、双方表、双向 chat、第三台加入、并发配对、过期清扫） | **PASS 6/6** |
| `open_from_phone`（从手机侧打开会话） | **PASS 1/1**（60.7s） |
| `resume`（暂停/恢复后对端可达、耳朵保留、回答顺序） | **PASS 6/6** |
| `machine_pause` / `peer_pause`（整机与对端暂停） | **PASS 各 1/1** |
| `takeover_safety`（接管安全） | **PASS 1/1** |
| `sync_doc_isolation`（文档同步隔离） | **PASS 5/5**（1 ignored） |
| `transfer`（跨设备传输） | **PASS 4/4** |

合计 **25 项通过，0 失败**（其中 `pairing` 的两 node 是真的 TCP 配对，不是 mock）。

## 4. NOT_TESTED（显式记录，不当作通过）

| 用例 | 现象 | 归属 |
| --- | --- | --- |
| `node/resume.rs::a_far_terminal_rejoins_after_resume_and_its_replay_has_what_happened_meanwhile` | 宿主能起、attach 能连，但 replay ring 只有一段清屏与提示符，没有程序的 `sayer-up`；已改用本测试二进制自带 fixture，现象不变 | 终端/ConPTY 读取，W5 排查 |
| `node/host_replay.rs::a_replay_trimmed_past_the_modes_still_starts_in_them` | 10,000 行 flood 经 ConPTY 在 Windows 上 22 分钟仍未完成 | 同上 |

两条 NOT_TESTED 另有两份独立证据（2026-11-15）：宿主跑 `cmd /c echo hello-host` 后 30s 内 `last.txt` 仍不存在、`ring` 从未写入；
直接调用 `portable-pty` 读同一个命令 15s 无输出且 `child.wait()` 不返回。因此问题在 ConPTY 输出路径，不在这两个用例的断言。
| 真实双机配对 | 本机只有一台 Windows | 需第二台设备；本机已用两 node 回环覆盖协议 |
| 远程控制与审批的真人流程 | 需要 App/GUI | W5 之后随 GUI 一起验 |

两条 NOT_TESTED 已用 `#[cfg(unix)]` gate，文件里写明原因；不计入 PASS。

## 5. 结论

- 配对、恢复、暂停、接管、同步与传输在 Windows 上跑通，25 项全绿。
- 断线重连的**服务端**语义（resume 后对端可达、耳朵保留）已通过；"远端终端重连后 replay 内容"这一条卡在
  ConPTY 读取，和终端面板同源，留到 W5。
- W4 判为 **PASS_WITH_EXCEPTIONS**；物理双机与审批真人流为外部条件项。
