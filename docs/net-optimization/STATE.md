# feat/net 当前状态与接续入口

更新于 2026-11-13T05:58:13.375460+00:00。活动目标是 [GOAL](GOAL.md)、[PLAN](PLAN.md)完整N0–N5，**未完成**；117c39当前因Release高BDP交互退化被拒绝，不得晋升为最终候选。root串行，不派子智能体。

此前STATE逐字保存在[本次完整历史](HISTORY-20261113-through-qlog-cache.md)，SHA256 `e5fdf9aed5ef256161015e24ec96c51c8a3fbc1f77db07600b09a63f6b63ce94`；它继续链接全部更早历史。历史“在途”只指其原时点，不重启旧批次。逐项未完范围见[REQUIREMENTS](REQUIREMENTS.md)。

## 现场与边界

- canonical `/Volumes/disk0/Khor/worktrees/current/khor-net`，branch feat/net；运行时仍5064fe2a、iroh1.2/Noq1.3。升级只在独立候选，文档/control提交不等于已合入canonical运行时。
- 产品I `khor-net-product-integration-v1`，integrate/net-product-v1，HEAD **117c39e5432741198c924177fae1775ff292eada**、tree **ace4fd8223ed7fcd8d2b29477d9e651a2df53ff2**；parent是冻结对照当时的公开main **c3b098bdbf52a9b3b38e887ae74b90155292b544**，apps/与.github/逐字一致。iroh1.4.1/Noq1.4.0。不是对持续变化main的自动重基/合并。
- 所有下述树同在 `/Volumes/disk0/Khor/worktrees/current/`。S `khor-net-product-sole-peer-v1` clean ade7decb4aa2d8616fb028b0b0221873d9d6f494，原Debug唯一middle固定3已闭。
- B/C `khor-net-sole-release-{baseline,candidate}-v1`，两experiment分支、各基c3b098/117c39；共同测试overlay staged，闭合首业务及共存证据，不按摘要覆盖。
- T `khor-net-sole-cache-v1`，experiment/net-sole-cache-v1，HEAD117c39；三个新测试文件staged。闭合v1源tree **14e9f060422b3cfbd28976364d781e80daedeef7**/snapshot **05325c56df1ab25d53878bcc16537187a9abff7a**。其后sole_cache.rs又追加新sp6（**unstaged、未fmt/编译/运行**），旧sp5原函数保持。见下，不把现场新diff误作旧已验证源。
- Q `khor-net-workload-qlog-v1`，experiment/net-workload-qlog-v1，HEAD117c39；staged诊断overlay，冻结tree **d372d293f69f4980bc0926797bf99f99ffa60f3a**/snapshot **7e328dc5d000873f1cca809683fa41c0fa472e99**，三qlog病例已闭。
- A（新ACK诊断） `khor-net-workload-ack-v1`，experiment/net-workload-ack-v1，HEAD117c39；staged冻结tree **f92abcfffe5084335feb0c50007a3327d54fae07**/snapshot **9c5a4829badfa1c351fddbe38c3def3fc446937c**。仅Q上测试环境/manifest变化，qlog optional代码默认关保留；actual CLI额外编译已有net-diagnostics feature。
- control `khor-build-net` HEAD保持 **5d3691f252f9dee0297fa3c494374be6119bca29**；控制仅新建Git对象与refs/khor/net-controls。E=`/Volumes/disk0/Khor/evidence/net`。

本机source-only：Git、stdlib Python源码/元数据/数据/ZIP处理及SSH编排；不运行本机Cargo/Rust/rustfmt/Node/Bun/Xcode或下载的原生产物。重型工作只Turing/macserver/可信GitHub。用户main、已安装App、其他任务Lab23961（源98285、PID4769/exec18781）不动；无main合并、发布/标签/TestFlight、生产Relay采购部署或已安装App替换授权。用户已明确公开colehank/khor-build，Actions通道已恢复，不再按旧私库额度阻塞。

## ACK串行coordinator已闭：root恢复唯一ledger writer

**E/ops/run-workload-native-ack-three-v1.py，exec76657，localPID64594已终态0**，progress/results completed，ledger无pending；root恢复唯一writer，不重启旧批次。详[WORKLOAD-NATIVE-ACK](WORKLOAD-NATIVE-ACK.md)。fixed3共2160ok、三file全，但未复现原2秒超时、未定因；原普通Release候选拒绝保持。

- protocol **E/workload-native-ack-diagnostic-plan-v2.json**；fixed3 fresh high病例、替代0，修订在runtime前冻结。source A如上；原192MiB/三phase各目录+真实TTY各120样本/2秒caller/240秒file/600秒owned不改。
- compile **054642Z-d37db21c** 已 **0/300.285793s**，19files及identity/source/hash/group空、原native verifier的Release opt3/20first-party unit/实际CLI与test核。18项目依赖复用，仅CLI/test fresh；old Q两ELF只读保全。actual CLI/test features均net-diagnostics，khor_net实际qlog。E/run/native-ack-release-compile-verification-v1.json。原compile.json继承upstream_qlog标签未覆写，专属audit明确标注这个元数据限制；实际feature/CLI及runtime合同另核。
- runtime控制 **ab2ac0803d207a3d98271a38f7ce2ccfa08134da**，E/workload-native-ack-runtime-control-v2.json。v1仅修suite标签、未执行；不可误启动。旧qlog控制不能在已切A的stable candidate source运行。
- 首runtime **turing-20261113T055445Z-f9df1c24**，receipt E/run/receipt.json，remotePID3488460；exec28549仅提交已终态0。后两轮055802Z-ad8c39fd、060113Z-5af34dfa已由coordinator串行执行完并闭。**E/workload-native-ack-diagnostic-progress-v1.json / results-v1.json**是执行索引。
- QLOGDIR不传且显式unset，无qlog采集；NETLAB_DIAGNOSTICS=1启用已有native ACK/选路日志，保留原两resident日志及wall/monotonic锚点。300MiB日志保护只向实际Popen-owned test group TERM，原512MiB压缩/1GiB展开collector门不放宽。触cap归采集资源中断、非产品根因，停后续。
- 专属 E/ops/verify-workload-native-ack-case-v1.py 核原720总体、原Rust出口/bytes/geometry/overlap、全部日志SHA/字节及实际ACK/stream snapshot、无qlog、native owner；Rust红保留。三次均未复现，停止同样诊断，不当修复或普通性能接受。原unlogged两红仍在。

Turing原UID1110/flock/jobs16/nice10/Rust1.95/50GiB启动/40GiB停止/90分钟outer守护保持。所有老coordinator（72887、45051、68965及旧61277）均终态，不恢复。

## 当前关键失败与定位

[WORKLOAD-RELEASE](WORKLOAD-RELEASE.md)、E/workload-release-comparison-verification-v2.json：公开冻结main对I117c39，同12文件测试overlay、生产overlay0、双方原Release opt3，fixed **A B B A A B**、low40ms/8Mbit与high300ms/50Mbit各fresh；12业务case、8640原记录、12完整file，额外0。原业务caller2秒不改。两preflight沿已定MODEL-DECISION分层准入，原high cold TCP门红仍保留，独立UDP容量及真实payload/RTT/几何/owned取消守护支持Node测量；不伪造TCP校准绿。

baseline low/high各3/3；candidate low3/3、high **1/3**。candidate high共1925ok/3timeout/232blocked；blocked为未发送的意图，不当成功或传输请求。首次041753Z-362039e0为bulk TTY index1与目录index2超2秒、其后118TTY blocked；二次043619Z-e3822070为idle_after TTY index5超2秒、其后114blocked。全部file均完整；high median file19.3299→22.4087Mbit/s不抵销交互失败，idle_before TTY计划至完成经验p95约0.303→8.229秒，失败总体不能造完整成功p99。原client调度probe无同量级暂停，只限其客户端，不证明resident/host。两arm都是BBRv3/PULL_AHEAD8/response priority0，不能误称旧Cubic。

[WORKLOAD-QLOG](WORKLOAD-QLOG.md)：独立optional上游qlog诊断固定3（051025Z-5907dd5c、052027Z-1edb3353、052344Z-509f2cd6）全2160ok，三file全，8trace/case约327/334/330MB；每case分别6/6/5个EOF末条不完整，原尾字节/hash保留，不造事件缺失结论。所有owner/source/hash/原collector核；不复现原超时，停止不加同源试验，不能覆盖原两红。

已从通过的qlog case2进一步匹配同CID/stream0/方向/offset/长度的727个完整frame：TTY bulk index38耗1459798us，Y收到输入后约0.543ms就发送回复，Noq path1回复的两个传输观察点相隔1308.782ms。path1经双方tuple_assigned及iroh fd15:70a:510b:1::/64定义核为Relay；X送出364frame全path0直连，Y350frame直连、13frame Relay；后续Y回直连。file名同kernel壁钟有毫秒量化；n0-qlog0.2.0事件单位ms已核。packet_sent不是NIC时间，未细分TCP队列/物理链路，且不是原超时捕获。证据 E/workload-qlog-tty-delivery-correlation-v1.json。**新诊断问题是双向选路/共享样本及旧路径在途恢复，尚未改参数或宣称根因。**

## peer relay当前已过范围和新准备

- [SOLE-PEER-RELEASE](SOLE-PEER-RELEASE.md)：相同Release唯一M的Direct首腿/真实Relay第二腿首目录ABBAAB已闭。基线3次5秒右删失；I3次345236/366587/366320us，真实marker/唯一middle/no_relay=false/cold/实际drop与owner全核。右删失不能变成功5秒延迟或有限加速倍数。S Debug先前固定3也通过，其适用范围独立。
- [SOLE-PEER-CACHE](SOLE-PEER-CACHE.md)：T v1 compile053516Z-5aa15758 0/81.366352s；runtime **053846Z-4b1bebf2 原1/365.502303s**、36files/identity/source/hash/owner与ledger已闭。不可用第二腿hint（JSON null，非删map）、过时Direct第二腿hint、X历史Direct目标三类各3/3，hint前后保持、实际唯一M的Direct/Relay两腿和目标marker/5秒门通过。该9case不重复。
- v1 sp5三次失败原件保持，middle_down前置全false、前两次first_leg_up=false、末次middle_resumed=false。**前置未成立，不能据此将根因归给30秒miss缓存。**已核set_machine_paused接受设备名，不误诊其名字参数。
- 新 **E/sole-peer-cache-resume-plan-v2.json** 与T本地追加sp6已预写：用实际M→Y暂停拒绝、persisted policy bit和X→M真实内容见证，原X失败读期间5次有界只读keepalive维持首腿；从unpause开始含实际M恢复目录及一次新X目标读共享原总5秒，same caller PID、hint held、实际两腿。固定3、仅新case，不替换旧sp5分母。**目前仅源码/协议准备，未fmt、未freeze新source、未编译运行**；ACK coordinator已闭；下一新任务仍须准确source/control与原所有权准入。

## 可复用的其他已闭证据

- 产品I完整Quality **43348869616** FIRSTATTEMPT13/13原门成功，8原artifact SHA/字节/CRC保留 E/public-quality-net-product-v1；scope为准确I源，不能盖过N3红。
- 最新I Mac Lab **43355810513 attempt2 success**（port23963，request ad847c8fb3874591bf786afd5c347a24，native_contract71e64768d0b001dcf6f8c45deaab341e845b226ae3d58443264bdb3e083ac4d7）。原WebKit settings15s失败保留，仅失败job同源重跑1次、77browsercase全过。继承native/frontend未重建，原artifact ID/digest相同。inner native.zip108065996字节/SHA2ca9754e5831d6d1dbfaa259abe1d80da8e211484a19bb3ab6adf8e3daa0b031与ZIP CRC/MachO arm64/Info.plist独立Lab/source/tree/snapshot/contract/probe核；无下载native本地执行或安装替换。详 E/public-native-lab-net-product-v1/lab-acquisition-verification-v1.json。物理iPhone仍旧0.2，不是候选真机/能耗接受。
- 新旧CLI真实双向row/pin及两常驻host/child见证 **011900Z-19b49119** 0/160.457763s；[IROH-MIXED](IROH-MIXED.md)。只证明两懂Versions版本的窄互通。
- 策略长稳A155449Z-56a8ac18/B180542Z-c09f6a3c各5case/600秒原窗口/32逻辑目录全成功；p1h包+33.970%/字节+37.908%，p5有轻退化，非闲时或能耗最优，原用户取消164807保持。
- 3b恢复first-ACK红→Noq467、iroh198+原ignored1、12app276+ignored1及固定3次物理124/两4秒门过；[IROH-RECOVERY-ORDER](IROH-RECOVERY-ORDER.md)。旧严重红与后续未复现均保留，不追加同源挑绿。
- 上游2026-11-13T05:25UTC registry最新稳定仍iroh1.4.1/Noq与noq-proto1.4.0，E/upstream-latest-recheck-20261113T0526-v1.json。Noq GitHub latest UI旧tag不代registry。custom ledger v4共60历史项：1上游替代已证、2退出对照红保留、57仍待退出审查，绝不是57都必要。iroh21 Lagged/Openings及iOS同interface地址-only dedup只有静态观察，不冒充当前根因或已测。

## 接下来与完整交付

1. ACK三个固定病例已闭，不重复已dispatch任务。根据实际ACK/选路与连接身份，设计最小修复或明确消融；普通unlogged同源Release复核必须另立固定协议，原失败保留。
2. 修正前置的新sp6三次及其余NET-16策略/缺字段/多middle/移动范围，不能把9窄合同代全部peer relay。
3. N3补同步/预览/SSH/hop并行负载、背压/公平/取消真实资源与共享/分连接比较；1/3/5设备闲时CPU/线程/字节/唤醒/手机能耗，现有成本回退需解释/改进。
4. N4双方与peer relay IPv4/IPv6/NAT独立或同时变址、prefix/Wi-Fi↔蜂窝/锁屏休眠、寻址与Relay登记/出口故障域/容量/布局；MacLab与Turing同LAN，macserver另机。新候选物理iPhone窗口未执行。
5. 所有custom逐项退出依据、状态/职责及持续短回归/完整矩阵/前后对照/中继运维设计，最终收敛到同一准确source；与届时main隔离集成并跑准确源完整Quality/Mac原生/手机接受。无main合并或正式发布授权。

所有已闭结果只复用于其准确源码/范围。失败、控制前置失败、用户取消、采集资源中断及未验证分别记；源码计划准备不冒充实现/接受。最终完整目标、[DESIGN](DESIGN.md)、需求审计的未完项不因局部绿缩减。

首ACK病例的1.664秒TTY尾与单Direct copy2PTO/另一copy较新收包和全局silence重合，move仍Hold；精确hash-bound行证据见E/workload-native-ack-first-tail-observation-v1.json。静态off_direct guard限制旧copy wake仅为待检假设，不冒充已证根因。接下来先修正前置的新sp6，并以明确受控问题收敛路径/连接停顿，不再追加相同日志试验。
