# feat/net 当前状态与接续入口

更新于 2026-11-13T15:28:50.668005+00:00。完整目标 [GOAL](GOAL.md) / [PLAN](PLAN.md) N0–N5 **未完成**；逐项缺口见 [REQUIREMENTS](REQUIREMENTS.md)。root 串行，不派子智能体。用户已授权持续实施与远端验证，不因阶段完成停工。

E=`/Volumes/disk0/Khor/evidence/net`；工作树均在 `/Volumes/disk0/Khor/worktrees/current/`。此前 STATE 逐字存 [HISTORY-through-terminal-direction-launch](HISTORY-20261113-through-terminal-direction-launch.md)，SHA256 `b6712ae6174fb1be910c4f43f4b11424d6be545645358fc2a7d50c63c4eded47`；其中更早历史链接保留。历史 pending/PID 不能用来重新派发。

## 当前唯一在途

Turing **151349Z-c16a9107 / PID461002**，角色 terminal_direction，普通 Release 固定 6 场，原/capture/capture/原/原/capture。单一远端 controller 串行，无本地后台协调器；root 是唯一 source-refs writer。恢复时先读实际 receipt / execution，不能重新 stage。最近只读观察前 4 场均原门成功，第 5 场进行中，尚未完整闭证。

- control `0f50c7bcaa76a37897431f4dc0832ce271579f5a`。
- source tree `776ecc6b2bf358bb9421dbbd5c988cb4f1a52a7b`，snapshot `f2131be0c896a4b9ad94ae285a0cc0b0a9a98092`。
- 测试树 `khor-net-terminal-direction-v1` / `experiment/net-terminal-direction-v1`，现 clean commit `ac261817d4d4e897acf2614a286f6e4ead78621c`，tree 未变。原 descriptor 中 basef6b9/dirty=true 是提交前历史实情，不重写。
- 冻结协议 E/terminal-direction-runtime-plan-v1.json，SHA `a6893aaa724d3f121c3c9f316ccdfbaf7b1808b9e99a5ea37f25f5475c7330ef`。
- producer **145811Z-6f3704a7 0/544.370115s**，Rust8m14s，22files/identity/native 来源已闭；test SHA `499340a0655fc827e5e792af7c364ade14188560901a1a8aa332975c05462954`。CLI 精确原失败版本 `a7e3565f15ed15d59f5215bceeb3900d153d053dd853846f623d86efbe6e4ac9`，产品无改动；准备了 test 与显式 CLI 入口。

全部 6 场同可用 Lab Relay，原 720 意图/192MiB/2秒 caller/600秒 owned 保持，运行编译0、替代0。Python echo 保持真实 PTY，私有非阻塞 FIFO 只记录程序完整输入/回复和测量后 summary。helper 远端4/0/0，数据分析合成检查6/0/0。首个 capture（case2）中期核验有效：724条连续前缀、输入/回复各361（含预热）、epoch bracket130ns、无丢记录；仅中期证据，不代整个任务闭证。

[TERMINAL-DIRECTION](TERMINAL-DIRECTION.md)定义解释边界：at_ns 在 I/O 后采样，只是完成上界。早于观测超时下界的 reply_written 才能证明程序已写出；晚 stamp 不证明晚到/晚写。summary 只验证截至第一 summary 的连续无丢前缀，后续原记录全保留；缺 input 最多证明程序截至 summary 未处理完整行，不能推成包未到内核。要求同 time namespace、epoch bracket≤1ms、无读错/溢出。带记录程序不是最终性能/能耗接受。

执行完成后顺序：collect-network-ssh.py → E/ops/verify-turing-record.py → E/ops/verify-terminal-direction-block-v1.py → E/ops/summarize-terminal-direction-v1.py，均传 E/该run（collect 传 receipt.json）。验证与汇总脚本已准备，尚无最终结果。若前置或 owned 清理不完整，保留 partial 并解释，不能把失败替换成新场次。之后依据实测方向定位；本批未重现也不能宣称修复。

## 产品源与当前接受边界

| 源 | 当前事实 |
| --- | --- |
| canonical `khor-net` / feat/net | runtime 仍5064fe2a、iroh1.2/Noq1.3；本树后续文档提交不代表产品集成 |
| I1 `khor-net-product-integration-v1` | clean117c39e5432741198c924177fae1775ff292eada，iroh1.4.1/Noq1.4.0；N3原红，未晋升 |
| R `khor-net-live-middle-retry-v1` | clean9081b94dce827cbd3fb9b9e7bb13633d762e3cd9 / treecf9d172bb92778ccfd97899ddea27bdf0e6d2f6e；窄 live-middle 修复，不含W |
| I2 `khor-net-product-integration-v2` | clean3093f86662b9b9ff139f29f85692e84ec2e47358 / treebf4d38d73fbb8eda558d08446deff2d930b65304；runtime逐字R，另两份验证工具 scope 修正；apps/.github逐字冻结main c3b098bd，未最终准入 |
| A `khor-net-relay-availability-v1` | cleanf6b9a75633485b8ab29f187479506f186ba88618 / tree953fd88f5f39fa99bc726296e5a84559996ce5cc；runtime逐字I1，用于当前诊断 |
| S `khor-net-builtin-selector-v1` | clean5e712671e53620c14f6ac8931da64f677b557fc1 / treececa08abc132f807e74fe0b463a231b26d942645；只删RoadOrder注册，只诊断 |
| builder `khor-build-net` | HEAD5d3691f252f9dee0297fa3c494374be6119bca29未移动，只新增控制对象/refs |

## 最近已闭证据

- **Relay 可用性** 132935Z-b055e94a 1/1161.211608s，18files/identity/native/CPU/owned闭。原同配置固定6场：可用1/3、2095ok/**2timeout/63blocked**；不可用3/3、2160ok，六文件完整。两次原2秒TTY超时在 idle_after/index82 与 idle_before/index93，不仅发生于 bulk。bulk TTY scheduled p95 5.874877/.336874秒；完整 [RELAY-AVAILABILITY](RELAY-AVAILABILITY.md)、E/relay-availability-comparison-verification-v1.json。不能据此建议产品禁用Relay。
- **选择器隔离** 固定 RoadOrder/内建/内建/RoadOrder/RoadOrder/内建六块已闭，E/selector-comparison-verification-v1.json。各3/3、2160ok、共6完整文件，运行编译/替代0；同期 RoadOrder **未重现**原超时，预冻 diagnostic screen=false。bulk目录 p95 584911/409883us、TTY441411/316267us；文件中位23.548850/23.542792Mbit/s。内建仍含vendor补丁且默认Relay=backup，违背速度优先全合同，不晋升。producer135643Z-73d37a5b 0/552.550232s；六runtime run见原验证JSON，不重派。
- **窄 live-middle 恢复** 原I1 sp7真实3/3红保留；R070832Z-6b87e77c固定3在原5秒内恢复，1013894/784788/878995us，真实Direct首腿+Relay第二腿、held hint、同PID/marker/owner均核。有限Ask/Forced按原3秒复查，TRIES2/trust/单跳/写不重放保持。[LIVE-MIDDLE-RETRY](LIVE-MIDDLE-RETRY.md)。Linux92/92；同源Mac **43432815139 92/0/0、105.52s**，14files/CRC/source/control/raw/50-40GiB guard已闭，E/public-mac-live-middle-whole-hop-v1/verification.json。只接受该窄范围。
- **Tuple patch38 退出** R M M R R M六块/12场、744读均ok、180暖开流。强属性R3/3且mismatch0；删补丁M0/3且mismatch215，保留patch38用于已测当前NAT合同。[TUPLE-REFRESH-EXIT](TUPLE-REFRESH-EXIT.md)。不覆盖全部IPv6/手机。
- **N3/W 与 CUBIC** [COPY-WAKE-RELEASE](COPY-WAKE-RELEASE.md)固定18场/12960记录：B6/6，C6/6但目录尾延迟越界，W5/6含2timeout/118blocked；均不晋升。CUBIC真实Node固定3功能2160ok但原门2/3（首轮direct_after=false）；当前Noq固定19场/1101请求均ok，2%丢包CUBIC/BBR吞吐中位.325742/10.067335Mbit/s，否决全局CUBIC。E/cc-matrix-comparison-verification-v1.json。模型绿不是原Node超时重现。
- **验证工具** Cargo dev依赖不传播，typed/legacy ID一致性检查保留；旧37+新增9回归原3fail/2error→修正后46/0/0，真实Release亦准入。native_provenance.py SHA7dbd2c0f4048f8cc96a3c9303565b6298c4999daccdd4fe360825d21d225ac7b。明确是修正后的工具；fixture变更仍须显式 prepare CLI，不能拷贝fingerprint或弱化来源。原未准入/格式/前置失败保留，细节见历史。

E/relay-selector-static-audit-v2.json、E/terminal-direction-flow-audit-v1.json记录静态事实：host Welcome后本来是raw PTY；p.rtt已有传输估计回退；Main等FIN而Tunnel双工，不能只改Proto共享；低multipath配置会被忽略；W实际发path.ping补探测，不是通用驱动wake；程序stdout写出不等于网络收到。避免重复错误假设。

## 全目标剩余与约束

N1完整路径/置信/成本；N2全期限、取消/资源、写与0RTT不重放、全部NET16；N3真实同步/预览/SSH/hop公平、背压取消、共享/分连接比较、1/3/5设备CPU/线程/字节/实际唤醒与手机能耗；N4独立及同时IPv4/NAT/IPv6 IID/前缀、双端/middle、WiFi↔蜂窝/锁屏/休眠/发布发现、中继故障域/容量/布局；N5逐项custom、最终同源Quality/Mac/手机、设计与报告均未齐。旧p1h包+33.970%/字节+37.908%保留。ledger v5的60项=1上游替代已验、2Noq移除红保留、1iroh38退出红保留、**56待审**，不是56均证明必要。

最新官方registry核验2026-11-13T10:15:56Z仍iroh1.4.1/Noq+proto1.4.0，E/upstream-registry-recheck-20261113T101556Z/verification.json；最终交付前再核。[MOBILE-ADDRESS-AUDIT](MOBILE-ADDRESS-AUDIT.md)确认iOS AF_ROUTE独立通知确在，App表面去重不证明漏地址。手机仍旧0.2，新候选真机/能耗未接受；有可用候选再约新窗口。MacLab/Turing同LAN，macserver另机；旧I1 Quality13/13和MacLab同源包不代最终I2验收。

本机source-only：仅Git、stdlib Python源/数据/metadata/ZIP/hash与SSH/GH编排；无本机Cargo/Rust/rustfmt/Node/Bun/Xcode/native产品执行。重型只Turing/macserver/可信GH；khor-build已用户授权public。Turing UID1110、排他flock、jobs16/nice10/Rust1.95、50GiB准入/40GiB停止/90min outer、私有user/net namespace，无sudo/宿主网络修改。Mac guard50/40GiB、target100GiB，SHA998f5723eb447a1634411397c18a524e5ed0d50bd6dbd81001a16626219f64eb，jobs2，不能降门槛。保护其他任务Lab23961；需操作时先读中央治理STATE核实际源/PID，不信历史映射。

无main合并、正式发布/标签/TestFlight、已安装App替换或生产Relay采购/部署授权。使用CRG先探索；没有.codegraph就不建。保持全部原红、取消、无效前置及原意图，不能无变化重复直到选绿。所有原始源码、receipt、log、selected原生归档保持；snapshot envelope的 `source` subtree 才是实际源，不把根envelope当代码。
