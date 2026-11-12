# feat/net 当前状态与接续入口

更新于 2026-11-12 12:51 UTC。用户已恢复“继续”，完整目标“完成计划”仍为[PLAN](PLAN.md) N0–N5，**未完成**；root串行，不派发子智能体。用户“我没有验收 打错了”有效，没有新的Lab/真机接受。最新U18授权跟进最新稳定iroh并尽量减少custom；旧AGENTS仅规划状态已被后续授权覆盖。
原STATE逐字保存于[本次历史](HISTORY-20261112-iroh-upgrade-through-mcp-preparation.md)，SHA256 `3ed8de20f1d15b0088559416195ff5ac0070398554387c3993c43f7094ea17ec`；其中继续链接原stream准备/暂停/恢复历史与完整旧证据，不按摘要覆盖未提交内容。

## 当前执行与下一步

**当前在途 turing-20261112T125007Z-645fe88c**：task net-vendor / control bd13e1e7f8b2f12a6089978a69180d108eec4f14，pid1875236。独立修复候选 **80aeeff919aa641dae55ba1a18b2020fead3e05e** / tree **f857f33c60e45cb7fee862c47c510cf5a8a27765** / snapshot **bc35d760bc6d0cf9d47aeee390e1913c175b8a10**；仅确认probe在同PTO时沿用现有代表的recent count/time排序，无新timer/状态/预算。先核对receipt/execution，不重复调度。
红回归源9f3a6ee6的 **124312Z-e33e746e** 已101/338.074690s终态：Noq467全过，iroh原197过/原ignored1，仅新增quality-copy行为回归真实红（实际90ms、应85ms）；node lib/tests编译/三锁/源码diff与17原hash/group空均核对通过，proof/ledger已闭。红绿回归文件逐字相同，源码差异仅probe_priority/doc，E/iroh14-confirm-copy-source-audit-v1.json。原失败行2118/2119证明两copy在同tick均PTO0；+2298.518ms moving仍pending，+2551.014ms已None/restNone，E/iroh14-confirm-copy-observation-v1.json；精确取消调用仍非原日志直接观测。
全库门后固定三轮原物理peer replug/原四秒selection+decision门，全部结果保留；不是唯一根因/完整恢复或升级接受。最小设计与反例见[IROH-CONFIRM-COPY](IROH-CONFIRM-COPY.md)。

**前一固定被动诊断批次已全部终态。** 源 **08ecab50f81f5e9e357a73614dface145feef432** / tree **bb049e979141bd90bc1ab4a980b232fa78a0b99e** / snapshot **84b69692f39fe9473a0da4316ac59334e77dbdac**，control **69d23d4f912c2e3674d47010eea5b2577230e182**，仅DEBUG取消/Rest补点，算法、物理和四秒门均不改，不晋升。
首轮 **121515Z-e8d79c86**：0/261.855773s、16files/124请求全ok；warm门过51请求，X Go3285193us，Y没有Go，原decision门红保持。5条取消均是当前Custom/Ip→候选Relay，未捕获目标Relay→Custom取消。固定追加两轮计划 `E/iroh14-pending-cancel-two-diagnostics-plan-v1.json` 已执行并关闭：R1 **122705Z-1248c9ae** 0/167.663671s，两门过，X/Y Go3505368/3506880us；R2 **123405Z-8a26c7cf** 0/179.241810s，两门过，X/Y Go3283424/3538071us。各16原files/124请求，identity/hash/frozen attempts独立重算/group空核对，proof/ledger全部终态；原失败不豁免。
结果 `E/iroh14-pending-cancel-two-diagnostics-results-v1.json` SHA **60fc1b2c7003109b2976f854231d3f6d5da65dff5f82e7ee0810e14131ec6b2a**。两轮后按预先计划停止采样，**未重现9.3s/未捕获目标取消，无唯一根因/修复**。继续分析原失败日志，不能再原样重跑挑绿。

**普通性能对照已全部终态。** [结果](IROH-WORKLOAD-RESULTS.md)与[冻结协议](IROH-WORKLOAD-COMPARISON.md)：旧dc738/新028、rolling8/priority0、37份fixture/metadata逐字相同，ABBAAB六block共8640交互全ok、12file完整、102原report SHA独立核对。E/iroh14-workload-comparison-v1.json SHA **940dda764a0f7787bf2439194b9bed6802ee3dbeed3736d60fe709358328732a**。全部proof/ledger/campaign终态。
low file中位19.222→18.798s，high75.729→66.124s；low bulk终端单轮p95中位84.194→90.816ms、目录115.843→125.784ms。新block3 high终端service p95/max681.495/1102.768ms、scheduled-to-end p95 1893.762ms；新block6 low idle_before终端p95另有246.149ms。全失败/人口保持，少量fresh/共享宿主负载/终端长尾使它仍非N3/p99/全局最优/手机接受；不以文件优势宣布升级胜出。

**严格peer恢复门仍红。** 旧110236与新110840完整日志本地应用准确frozen原4秒selection/decision门（零新产品run）：旧两门过，Go3472631/3462188us；新两门红，Go **9301772/9349684us**，52个eligible warm请求中15个AB未选carried。E/iroh14-peer-strict-gate-comparison-v1.json，ledger additional_contracts保留原availability的独立scope。原物理business124全ok并不接受最佳路及时恢复；新028首次setup超时原红105138同样保留，尚未注入换网。
E/iroh14-peer-recovery-sample-gap-v1.json绑定原行：新X可见carried质量111.16ms在2.535–6.998s未刷新，age增到4.835s，Relay约135ms；原25%gain门不选该值是源码/data推断，7.372s附近新carried约98ms后才重启2s确认。查pending被取消/Rest、测量代表copy/probe copy、有界恢复预算、外层积压与调度，**无唯一根因/修复，四秒/3probe/2s/250ms和新鲜性规则不得为此放宽**。

最新已闭地址场景均准确028：roam三场景102059Z-2a7ba71e（372逻辑请求），IPv6同前缀overlap两合同111507Z-d7912aa7（248），实际QUIC NAT端口111859Z-de624e0b（124，30/30 warm实际streams选中新tuple，mismatch0），全部proof/ledger终态，详见下表；不是同连接/逐packet/手机/完整N2。

下一步依据原失败精确copy事实形成最小可证伪回归：X在+2051.718ms发起Relay→Custom确认时，质量代表为Conn140064923605712/Path1（95.483590ms），实际候选probe却取Conn140065527340576/Path1（当时138.971123ms）；后者的新111.157583ms成为代表，收益门随后不再满足。此为原行可观察的copy不一致，不独立证明全部停顿原因。核查同PTO副本tie-break与测量所有权，保留故障副本优先、候选低PTO、原freshness/预算/四秒门及反例。冻结普通源028及旧sourcequad不动。
混版后续优先复用crates/cli/tests/sync_mixed.rs与scratch::pair_built；需准确旧/新CLI及OLD_KHOR非skip执行标记，缺OLD_KHOR直接返回不能算接受；该fixture只覆盖双向session row/pin，不扩称peer relay/全RPC。其余完整N0–N5和补丁退出继续。

Mac仍50GiB启动余量门红；只读库存43205919189又被GitHub付款/额度在prepare前拒绝（E/mac-storage-inventory-43205919189/receipt.json），storage skipped、零Mac操作。已异步请求恢复Actions或常规SSH，无回复；最新一次SSH仍banner timeout255。Turing继续，不降门/清未知缓存。
root为E/source-refs.json唯一串行原子writer；submit/launch/collect/verify待真实exec会话终态，unknown handle先恢复receipt。源/任务与所有旧证据范围见下文，不因阶段结束停止完整目标。

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
| IPv6地址重叠替换两合同 | turing-20261112T111507Z-d7912aa7 | 0/165.659413s，13原files；2tests/248逻辑请求/496起止全ok，内核/IPv4隔离及非移动端先请求合同通过；非手机/同连接/完整N2 |
| 实际QUIC NAT端口重绑定 | turing-20261112T111859Z-de624e0b | 0/99.774935s，18原files；r0校准+r1远端先请求，124请求全ok、30/30 warm streams选中新tuple，mismatch0；非同连接/手机/完整N2 |

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


## 本轮补丁与手机静态审查补充

E/iroh14-event-buffer-exit-audit-v1.json为补丁21容量覆盖的后续退出候选：已有上游Lagged/paths快照和app选择恢复，但mine/Openings库存尚未在Lagged对账，需真实burst消费落后回归；主动ping另评估，未删补丁/未跑新产品测试。
E/iroh14-ios-notification-static-audit-v1.json核对OS callback→resident→iroh/有界恢复链存在；callback去重键仅status+接口类型，同Wi-Fi地址-only不独立穿过该去重；iroh另有监控，不能判漏通知或现场根因。手机须记录地址/前缀/地址族变化、事件所有权及remote-first/休眠顺序，未获真机接受。详见IROH-UPGRADE末两节，运行时均未改。


## 最小peer取消诊断源（已启动，编译/诊断在途）

隔离树 `/Volumes/disk0/Khor/worktrees/current/khor-net-iroh-peer-diagnostic-v1`，branch experiment/net-iroh-peer-diagnostic-v1，源 **08ecab50f81f5e9e357a73614dface145feef432** / tree **bb049e979141bd90bc1ab4a980b232fa78a0b99e** / snapshot **84b69692f39fe9473a0da4316ac59334e77dbdac**，clean。descriptor E/iroh14-peer-diagnostic-source-v1.json，两refs refs/khor/net-experiments/iroh14-peer-diagnostic-v1/{source,snapshot}已保留。
仅在原khor_net_samples DEBUG下补confirm_move因selector无候选/保持当前而clear时的pending/requested/current/since/now/readings/silent；Moving在已有pending进入Rest时补一次原日志。两文件差异为18行日志/谓词和KHOR说明。E/iroh14-peer-diagnostic-source-audit-v1.json逐字证明去掉新增诊断/日志谓词后生产Rust复原028，Noq/app/锁/原物理fixture/两严格parser均不变；这只是结构证明，**尚未编译/格式/运行，不是修复或新的验收**。
六轮普通计时全部终态、收件/核验之后，已用原control69d23d4f、task netlab-peer-mobility-samples、revision iroh14_peer_pending_cancellation_v1、该descriptor、最后block6 receipt为predecessor启动顶层唯一诊断，保持原4秒门及所有预算，记录失败/成功，不原样无限重跑。重点区分pending取消/Rest、采样copy与probe copy、外层恢复质量；不要在原因不明时改gain/样本门/预算。当前诊断已启动，canonical/冻结性能源未改，尚无修复。
