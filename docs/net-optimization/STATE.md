# feat/net 当前状态与接续入口

更新于 2026-11-12 11:11 UTC。用户已恢复“继续”，完整目标“完成计划”仍为[PLAN](PLAN.md) N0–N5，**未完成**；root串行，不派发子智能体。用户“我没有验收 打错了”有效，没有新的Lab/真机接受。最新U18授权跟进最新稳定iroh并尽量减少custom；旧AGENTS仅规划状态已被后续授权覆盖。
原STATE逐字保存于[本次历史](HISTORY-20261112-iroh-upgrade-through-mcp-preparation.md)，SHA256 `3ed8de20f1d15b0088559416195ff5ac0070398554387c3993c43f7094ea17ec`；其中继续链接原stream准备/暂停/恢复历史与完整旧证据，不按摘要覆盖未提交内容。

## 当前唯一产品在途与下一步

**turing-20261112T110840Z-1e90b3dd**，task `netlab-peer-mobility-samples` / revision `iroh14_peer_relay_replug_confirmation_v1`，role diagnostic，pid1651595。
source **02807c79e04a92c46fb2e6c14d2ecf107b98336c** / tree **f87d31f97b89474fe52297d87e3827de33550493** / snapshot **dc21d783909981141bd197c0b2ad1bdc22da150f**；control **69d23d4f912c2e3674d47010eea5b2577230e182**。这是原失败的一次不改源码/控制确认，不因通过删除原红；先核对execution，禁止重复提交。
旧dc738同control **110236Z-baeba5f5** 已0/213.826096s、16原files、124逻辑请求/248起止全ok、内核middle IPv4从198.18.2.2变198.18.3.2、变前两端连续报告carried≥10s，变后再次观察carried。proof `peer-mobility-availability-verification-v1.json`及ledger终态。新红启动load90.18/128逻辑CPU，旧绿9.78；这是观察差异，不是唯一因果/回归归因。

新028的 **turing-20261112T105138Z-c9de2969** 原101/150.987066s，16原files；唯一Rust case失败，只有第一笔setup读取（10.039537s，error），**尚未注入middle换网**。准确frozen attempts重算、原source/control/hash/group空通过；`peer-mobility-preparation-failure-v1.json`与ledger终态。不得把此红写成换网恢复失败，也不在旧版同条件对照前归因上游。
新028静默first-action **100747Z-15afeb63** 与三换网 **102059Z-2a7ba71e** 已收件/核验/ledger终态，详见下表。其余本轮run同样终态。

先完成peer relay前置红的旧版对照与定位；再继续IPv6重叠/实际QUIC NAT端口恢复、负载体验与补丁退出。新依赖尚无同条件性能接受，不借旧性能证明新源。Mac当前50GiB启动余量门红，原Mac退出对照不重发/不改门；已有worker-storage只读inventory工作流 **43205919189** 被GitHub账户付款/额度在prepare开始前拒绝（原check annotation保存在E/mac-storage-inventory-43205919189/receipt.json），storage skipped、零库存/产品测试。首不可解析旧SHA调度422同样零job；已用同脚本private control ref纠正。已异步请求用户恢复Actions或给可用常规SSH；Turing继续。未清缓存/改guard。
root为E/source-refs.json唯一串行原子writer；submit/launch/collect/verify必须待真实exec会话终态，unknown handle先恢复receipt。

## 升级位置与授权边界

- Canonical `/Volumes/disk0/Khor/worktrees/current/khor-net` / `feat/net` 仍原运行时**5064fe2a**（后续均docs）；依赖仍iroh1.2.0/Noq-proto1.3.0。没有合并升级或rolling实验到canonical。
- 最新稳定升级候选 `/Volumes/disk0/Khor/worktrees/current/khor-net-iroh-upgrade-v1` / `experiment/net-iroh-upgrade-v1`，HEAD **02807c79** clean。从旧整合源dc73837e建树并合入规划文档，使用SHA匹配的发布包三方迁移到**iroh1.4.1/Noq1.4.0**，根及vendor锁已统一；七处冲突已处理，原本地专属回归和上游blocked-Relay Initial回归均保留。
- app的net/node Rust及四份wire metadata相对dc738逐字不变（E/iroh-upgrade-runtime-diff-v1.json）；依赖更新仍改变运行时，旧性能不能沿用。
- iroH补丁22的手工IPv6 trim已替换为上游helper，两上游测试函数体亦同，库门已过；IPv6 QAD测试含地址不可用提前返回，不能据普通ok扩称IPv6 WAN/手机通过。
- Noq2、5移除对照真实红，保留当前保护。E/iroh-upgrade-patch-ledger-v4.json记录60历史条目：**1项上游替代库验证、2项保留、57项待逐项退出核查**，不是57项全部必要；详见[IROH-UPGRADE](IROH-UPGRADE.md)与[UPSTREAM](UPSTREAM.md)。BLE/Tor/Nym不新增到本轮。
- 仍无main合并/发布、生产Relay采购部署、已安装App替换授权；没有新手机窗口。MacLab与Turing同LAN，构建macserver独立，不能混淆拓扑。

## 新稳定版的已核验证据

| 层 | run | 原结果与范围 |
| --- | --- | --- |
| Turing全vendor库门 | turing-20261112T084724Z-35d6dfe3 | 0/226.384657s，17原files；Noq467过、iroh197过/原flaky ignored1；node lib/tests编译过、三锁与冻结源同、post-check源码diff空 |
| Mac同树库门 | 43186999295 | 16原files/11controls；与Linux同467/197命名人口、原ignored1；node编译与三锁/源码diff通过；ZIP f8cbf61034871f8591f02bbbea4ff58953ac8c0974ade643cdec2a8278c48024 |
| Turing固定12应用门 | turing-20261112T085339Z-ab57a92e | 0/385.723121s，21原files；273过/原identity辅助ignored1，命名人口与旧dc738完全一致；无独立RPC计数扩称 |
| 36真实流合同 | turing-20261112T090144Z-223b1901 | 0/31.781419s，31原files；4份准确frozen metadata重算，独立74 Fetch RPC/10peer；273与36人口重叠不相加 |
| 实际CLI取消合同v2 | turing-20261112T095017Z-a01289cd | 0/186.335657s，21原files；两parent合同/两声明fixture helper，source2795文件独立匹配Git、CLI prior→node provenance/选中binary哈希核对，6不同任务Cancel IPC实际到达；两case4/24观测owner均退出、无强制清理/遗留owner/私有可执行文件足迹 |
| netlab集成目标编译 | turing-20261112T100217Z-d286d5bb | 0/120.570580s，15原files；锁定1.4组、目标artifact与binary SHA记录、post-compile diff空；**runtime0、非native_provenance或性能接受** |
| 新稳定版静默first-action | turing-20261112T100747Z-15afeb63 | 0/332.109970s，29原files；2 diagnostics-off guards/6参数fresh读；12准备、12完整≥10s静默与12真实STREAM阳性通过；普通时延样本0，非完整N1/能耗 |
| 实际接口换网三场景 | turing-20261112T102059Z-2a7ba71e | 0/186.422335s，13原files；dual/v4-NAT/v4→v6实际eth0重接，372逻辑请求/744起止全ok；非移动端先请求、内核地址独立核对；非真实QUIC端口rebind/同连接/手机或完整N2 |

静默proof为`iroh14-stream-quiet-verification-v2.json`；换网为`iroh14-roam-verification-v1.json`。后者各场景首remote单次原us为710238/777650/646640，不构成p99/SLO；UDP reflector使用独立固定端口，不能替代实际QUIC端口重绑定证据。

前述每run的identity/source/bundle/原hash/原group空与behavior/compile proof分别在E/run目录，root ledger均终态。库门proof为stable-upgrade-gate-verification-v1.json；固定12为stable-upgrade-node-suite-verification-v1.json；真实流为stable-upgrade-transfer-verification-v1.json；CLI为actual-mcp-verification-v2.json；编译为netlab-compile-verification-v2.json。
实际CLI合同用控制Python sidecar，不是真实agent bundle/WAN/手机/能耗。Turing现有conda Python3.13.13（任务临时wrapper -E -S）SHA c46ed00dd978dbb20e32eb34d9e16660775b2ed4fc84395a73120804e3e66379；没有改系统环境。旧Noq447、iroh184等数量只对旧源成立，不把不同平台/重叠套件累加成独立覆盖。

## 原红与退出对照（保持）

- 新028完整node诊断 **090450Z-bfc82a31** 原101/552.029829s，旧dc738同control340b3243的 **092140Z-5216816b** 原101/636.509801s：同1387人口，同1274过/38失败/75ignored，new-only与baseline-only失败均空，E/iroh-upgrade-full-node-comparison-v1.json。**二者不算完整node接受，也不豁免原失败**。20项直接观察Git2.25.1不认initial-branch参数，tmux3.0a；两项缺同源CLI（已另跑正确前置后两合同），两项账户runtime状态失败仍待定位。Quality全前置/真实agent bundle/isolated接受与其他原红保持。
- Noq5删除源e69b137c，**091835Z-cb319e57** / control958dc554，101/33.786267s，12原files：原4fixture三过一红，合法已分配/回收ACK-only最大号加低号ACK-eliciting包，误增RTT计数5→6。fixture/锁不变、仅mod.rs移除保护；基线1.4同四命名绿。E/iroh-noq-ack-exit-comparison-v1.json。此为状态unit模型，不扩称wire测量。
- Noq2删除源79d2ca64，**093340Z-deb1f7de** / controlfc6fe1d1，101/37.609123s，12原files：双方reporting both、服务端响应前发早数据，移除保护后ObservedAddr TX1而应0；原fixture/锁不变，基线同命名绿。E/iroh-noq-zero-rtt-exit-comparison-v1.json。两ablation源不晋升。
- 首准备 **071440Z-2f31345c** 101/226.706692s：Noq stats重复FxHashMap编译红，19原files；准确返回锁/formatter patch应用后纠正得到028。首gate **072057Z-c53a26f8** 因准备写共享source cache被保护拒绝、零测试，2原files；独占锁核准旧HEAD/tree/完整patchSHA后，将整个cache及owner marker改名保存在原准备run/work/preserved-preparation-source，未删除/reset。后续准备control94b1a637改一次性clone，未重复执行。
- Mac退出 **43189070840** 原50GiB余量门拒绝、零产品测试，5原files/ZIP 4f7ca2212b4ad3968ec775e54e747c6e9f1be01ea0061f6352984a6d5891c981；只验证control/bundle transport，缺source/environment执行证据，不记identity/behavior接受。已转Turing，不盲目重发Mac。
- 首实际CLI **094221Z-91240bdf** 原1/197.852856s、20原files：CLI/node provenance已过，首parent+合法mcp_cancel_worker均Rust过，但控制未声明helper导致严格metadata拒绝，第二case零执行；原failure保持。v2只补准确helper声明并在验证前持久attempt所有权结果，原源码/函数/门不改；E/actual-mcp-metadata-parser-check-v2.json对原日志与六种错误合成副本校验，零新增产品/网络样本。

完整设计与未闭范围仍见下文，原24性能场景、尾部问题、手机失败观察、Native/Finder红与所有旧证据保持，不因最新库门/CLI合同而缩小目标。

## 已冻结输入与四真实RPC合同

E/pipeline-source-quad-v1.json：宽8A0c4e4e35/B e5527f93、宽2A ea414bfc/B2438172c；E/pipeline-w{8,2}-{batch,rolling}-source-v1.json完整四树。priority0、A原batch函数逐字不改，B函数逐字来自57ff原型；同helper/逻辑/长files_wire/所有vendor锁，宽度内只有调度循环/必要短读重启差异，跨宽只有PULL常量。共同Python wrapper只标记本轮，沿用原recorder全部人口/期限/失败门。
宽8batch36a4357d、rolling436c6664、宽2batch0a61687c、rolling19528761各四逻辑/三真实合同全过，13原files/准确metadata/独立人口/hash/identity/原group空与ledger终态，各pipeline-wire-verification-v1.json。
宽8慢尾A已发9/客户端解码8/提交1片，B已发11/解码8/提交8片；慢头两者已发9/解码8/提交1片未超开。两宽两armfuture drop均收到全部等待流STOP_SENDING且同wire独立Sessions成功。
这只证明控制responder与原future drop，**不替代生产server、Job stop、host-child或最终N3**。首格式红5f783fc8原1/零行为、原patch准确映射b020a801与原证据保持。

N1诊断静默最新a09450d0原0/339.545858s/29files，十二准备/十二≥10秒静默/十二真实请求阳性见证过；仅full日志诊断准入，不是ordinary性能、手机或完整N1。旧v1静默红、普通静默缺证与全剩余范围保留在下文及历史。

## 工作位置、执行与授权

- PRODUCT `/Volumes/disk0/Khor/worktrees/current/khor-net`，branch **feat/net**，创建基线6485cb83；当前已测代码提交5064fe2a，之后STATE/历史文档提交不冒充重新测试的源码。
- CONTROL `/Volumes/disk0/Khor/worktrees/current/khor-build-net`，HEAD仍 **5d3691f252f9dee0297fa3c494374be6119bca29**。新控制只建Git对象/ref，不移动HEAD；现有两个未跟踪__pycache__保留。
- 隔离旧rolling原型 `/Volumes/disk0/Khor/worktrees/current/khor-net-pipeline-v1`，branch experiment/net-pipeline-v1，clean **57ffe973ed11d03dc76c5fa01904719b27651b36**；未集成、priority仍-1，不能直接与当前priority0比较。
- EVIDENCE `/Volumes/disk0/Khor/evidence/net`，ops在其下ops；canonical ledger `source-refs.json`仅root串行原子写。
- 本机source-only：不跑本机Rust/Cargo/rustfmt/Node/Xcode/产品。Git、Python标准库数据/合成检查、bash -n可用。重型验证Turing与macserver+私库colehank/khor-build；具体控制和收据不得混用。
- Turing原UID1110/flock/user-netns、Rust1.95/jobs16/nice10、50GiB启动/40停止、90分钟保持；不sudo/改宿主网络/在同Turing计时中插入编译。Mac共享guard/jobs2/90分钟保持，可独立重叠。
- 用户已授权“完成计划”实施，旧AGENTS“仅保存计划”不是永久暂停。无本任务main合并/发布、中继采购/生产部署、已装App变更授权；中央GUI任务的ship授权不套用。
- MacLab与Turing同LAN；macserver为另一构建机。localhost是旧0.2.0 iPhone；此前人工反复开关恢复不算候选自动接管通过，旧手机窗口已结束。
- 图谱partial/stale，不覆盖vendor时的0nodes/0flows不是覆盖证明；未建新的CodeGraph索引。不派发子智能体。

## N1首次操作与静默：已知证据及未闭合同

协议 [IDLE-ACTION](IDLE-ACTION.md)。原p7 active10秒/idle20秒门不改。原idle选路恢复16–17秒与15秒备用保活量级接近，只是假设，不证明唯一根因或极致体验达标。

**fixture所有权旧红→修复绿已闭**：

- 旧source e471a2f1、control e9a443c4，**turing-20261112T015902Z-e8569de4**：真实caller drop后device future继续，原1失败/1负控过，清理后才断言，原101/90.034秒/13files。
- 修复 **0a8b90d9986d3426f65b61ae9d994083f1fa3321** 只在LabNode.with由调用者持有AbortHandle Drop。**turing-20261112T020453Z-b9c5278d** 原0/53.827秒/两项全绿/13files。
- 两份idle-action-ownership-verification-v1.json：guard逐字相同、control相同、准确metadata/原marker/footer/hash、旧红绑定、原group空与ledger终态。只证明fixture caller→device future；不替代server RPC/host-child/产品恢复。
- 原workload filepull是父future直接借用tokio::join!，没走LabNode.with；不据此改写prefetch结果。

**普通六次首次读取已观测，静默合同未由它们证明**：

- 首个源码725e12f7的e6569ee7仅格式红/零行为，原件保存。
- source8f87adcb的 **turing-20261112T022049Z-7b65087d**：六次读取全ok，但metadata误期望两个netem都在alpha，缺router ingress读回，原1/226.568秒/17files失败保持。
- 修正source **5a5d7e84a557b2f2000aa0dcf513d61adc4538cb** / tree d1e6496d / snapshot67db6c6f，control2cfce93d；**turing-20261112T023151Z-8d7b0ea1** 原0/224.643秒/17files。六caller/十二起止/162条命令、真实双向150ms安装及恢复清除、veth ifindex/link_index双向绑定、逐接口计数独立复算、全部原SHA与ledger终态。
- alpha phase0/14/5首次读取297.685/327.577/321.867ms；beta230.688/265.406/250.937ms。实际phase与两端事后Relay路径、全部接口差值见first-action-verification-v2.json；每格只有一个fresh，无p95/p99/显著性/唯一因果或能耗结论。
- quiet-scope-audit-v1.json绑定上述proof：没有主动前台调用不证明无后台STREAM；完整N1/quiet未接受。普通source-owned输出显式continuous_stream_silence_verified=false/full_quiet_contract_accepted=false。

**Iroh50被动见证、v1真实红与v2准入绿**：

- 只在原250ms reselect已有stats读取上加DEBUG快照：所有受跟踪连接STREAM tx/rx、actor序号、增删代数及单调/墙钟；连接在采样间新建关闭也会改代数。原合计/ActiveRefresh/策略/生产计时器不改，无新任务/定时器。
- source **09374004906a9a400004874fa957726e857bef1f** / tree b23ff21f / snapshot8ee7e55c，control **1bdb2272f23bfd59961af79cbebf04242791ce33**；**turing-20261112T030608Z-15ebb293** 原1/305.229247秒/29files/identity/原group空。
- Rust原两guard+六读全过、十二真实请求阳性对照全过；十二静默见证全红。stream-quiet-verification-v1.json准确冻结复算、独立原人口/全部原SHA与ledger终态。
- 十三次STREAM变化被见证于候选窗起始后307642–645086us，actor/registry不变。这是观察tick，不是精确发包时刻或唯一业务来源；原失败保留，不把后半段选出来改绿。

**当前v2修复的是准备阶段**：

- 新revision first_foreground_stream_witness_v2，控制73fa35f5只改同task MODE literal；runner/其他分支/原UID/flock/netns/资源/期限逐字不变。
- 仅诊断模式、候选窗之前只读双方serve.log，实际counter/actor/registry签名连续≥1秒相同、快照年龄≤1秒、总准备期限10秒。无请求、无Node状态查询、无关闭后台任务。
- 记录准备起止和各端实际稳定快照；外部逐条绑定原日志。随后原12秒候选窗、固定500ms边界内≥10秒完整见证、整窗恒定、最大1秒采样间隔/100ms时钟增量差、原phase0/14/5与30秒caller、六fresh顺序及阳性门全部不改；候选失败不重试挑窗。
- 普通模式仍10秒/off且不等待此诊断准备，诊断不作为普通计时/CPU/能耗样本。
- 11种见证+6种准备+19种first-action+9种guard合成数据，4 unittest已过；6种调度stub保持guard off→action full与首原73/79失败。全部只是工具合成准入，真实v2已按顶层run/proof通过该诊断准入；完整N1/ordinary性能仍未接受。

## Mac默认vendor与其他回归

**新Iroh50准确源码默认库已绿**：Mac **43146243502** / request **ff36367cf58e4a1ebd0e5d6f5d1c460c**，source09374004/treeb23ff21f/snapshot **901d85407023827999147998c3318192f8be302b**，control **4e34fb7dce174345410a33e892b0eb76a5cc5c23**。
artifact **13911657397** / ZIP **4d8db782bfd2767c88855d5a14f3b6249aa06e338617ed1ba7e715e5a4904ce3**；E/mac-stream-vendor-43146243502，实际文件在artifact/（不是report/）。
16files/11controls/source/bundle/两vendor锁逐字一致/各原footer和SHA核验、vendor-full-verification-v1.json与ledger终态。
Noq完整默认库447过/0忽略/0筛除；Iroh完整默认库184过/原1忽略/0筛除。唯一原ignored flaky为endpoint::tests::connect_via_relay_becomes_direct_and_sends_direct，未运行/删除/称通过；58路径/11home专项与184重叠，不相加。
相对当前5064完整vendor差异只有KHOR.md文档，运行时代码与锁逐字相同；该证明适用于vendor字节，不证明CLI新准备逻辑或最终Node/native/手机。

原e471准确源码Mac **43134579036** 固定12项net-node全部通过：net68+1原worker ignored/hop89/wires16/journal30/budget32/IPC18+六单项。该门不是全部1378 Node，也不能声称新Iroh50之后最终Node已全跑。

隔离rolling原型57ffe：4窗口逻辑、跨八片的files_wire和handoff真实scratch合同已两次过；files库32过/唯一Finder trash AppleEvent -1712失败，Mac **43124311515 / 43126157862**，整轮失败保存。一次确认已用尽，不盲目同源重跑、不豁免该门。只读主机观察43128627304为零产品测试，未证权限/响应唯一根因；后续仍需收口。

## N3已闭对照与接下来rolling

[优先级](SCHEDULING.md)：low/high各完整18fresh，分别12960交互全ok、文件全完整、原population/hash/准确metadata独立复核。低profile目录p95九对-1更快，但high文件/目录/终端服务九个对应位置六次-1更慢、三次更快；不把位置差当逐请求因果。canonical c3287169恢复中性priority0，PULL_AHEAD=8；不是全局最优已证。

[批量prefetch](PREFETCH.md)：原8,1,2,1,2,8,2,8,1九组18fresh完整终态，12功能通过/六high文件原240秒超时；12960交互全ok、153原report文件SHA与原始人口复核。low九份16MiB全完整；ahead8三份192MiB完成82.239568–98.920231秒，1/2各三high超时。超时是右截断attempt，无verified payload rate，不称文件完成时间。
prefetch-comparison-v1.json SHA **7dede5f2c580d46aff0e6138b968da2550db2e095d6de351652a56d8a7e537b0**；全部advance1–9与旧high1–6终态不能重复。

[rolling协议](PIPELINE-COMPARISON.md)及prefetch-to-pipeline-decision-v1.json在任何rolling计时前冻结：**先宽8，再独立宽2**，priority0，每宽A B B A A B、每block原w3两项再low/high各1，共12/宽、总24fresh。宽2在low的完成/交互取舍值得保留但high原超时，不能直接设全网默认。
四源码与campaign已冻结；当前十二个block/二十四场景均收件核验（含三个原high文件超时）；原campaign终态，独立尾部诊断不算新普通样本。匹配baseline/candidate包含同一未调用helper/四逻辑/大于八片的files_wire载荷；A原batch函数不动、B仅rolling循环及必需短读重启；四树审全部Git/runtime/fixture/参数，只在相同宽度比较。不拿旧prefetch当新baseline，原失败/完整人口/期限门不改。
真实RPC的已发/完成/待提交上限、补齐与caller drop已有四源码合同，用户Job/DropStop/Offer stop/cut本轮真实合同已闭，但控制responder不替代生产source。后续仍需生产server/worker、完整run_take与host-child、最终发布所有权；全前台预览、公平/背压/连接架构/规模成本仍必需。

## 完整剩余范围（不以已有绿项缩减）

- N0：最终分场景UX门、充分尾部样本与p99、相同条件完整前后对照、观测成本与成本前沿；对载荷/调度/模型作实际记录。
- N1：严重/quiet/recovery尾部、跨连接独立质量/旧样本、恢复后固定好路径匹配参考、启动/配对旧失败根因、动态模型延迟突变问题、真实静默及后台成本。旧16–17秒/单轮绿不算极致接受。
- N2：同一实际连接独立/同时IPv4与IPv6地址/前缀/NAT映射变化、换网/睡眠/黑洞/LOST，owner/cancel/late/revoke与carrier在调用开始后失效；身份、总期限、写入/0RTT不重放。
- N3：上述24rolling探索已闭；仍需完整前台文件/预览、已发现的终端尾部、持续公平/饥饿/背压、共享与分连接实测、真实server和独立host-child取消/退出、1/3/5设备CPU/字节/唤醒/能耗。文件快不代替其他行为。
- N4：Direct/公共Relay/peer relay/SSH双腿，唯一可达middle自动救通（第二腿可Relay或旧信息）、多个/移动middle、独立双栈变化、北京/香港整机/单出口/寻址故障域，自家relay约束、双家与地域/容量/运维设计。部署采购另授权。
- N5：主60项（Iroh1–41/45–50+Noq1–13；42–44仍隔离）的逐项上游等价/保留退出，其他feature/integration/physical/Noq1+physical5；最终准确源码Quality/完整Node/native/proxy、Finder依赖、多设备/手机自动换网/peer relay与真实能耗及一致报告。

历史动态f2f1775027、恢复ce3b12226、长稳de04bcc1、四peer6970ad46、模型容量c50ab6ef和cold high5d27f45e等既有证据在历史入口完整保留；warm v2路径失败、启动/配对超时及旧严重红项也保留，不能拿新窄场景覆盖。没有用户iPhone候选安装/最终接受。

进度估计：此前2–4天为低置信度估计，不因compact/session重新起算；目前没有可信完成日期。完成必须设计、状态职责、持续矩阵与同条件报告均有准确版本证据。未获准的合并/发布/采购不自动执行。

### 2026-11-12 04:03 UTC 状态回复与估计

用户再次询问；明确尚未完成，普通性能与iPhone自动换网/peer relay/能耗未接受。下一批真实RPC合同加24匹配场景暂按4–8小时预留（含排队与构建、低置信度、不是承诺），完整计划仍数天且无可信日期；此前估计偏乐观，不从本次回复重新起算。全部N0–N5范围保持。
