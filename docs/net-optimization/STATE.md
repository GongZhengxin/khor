# feat/net 当前状态与接续入口

更新于 2026-11-13T04:45:50.939957+00:00。活动目标仍是[GOAL](GOAL.md)和[PLAN](PLAN.md)完整N0–N5，**未完成**。Release高BDP共存2/3失败，117c39不能接受；root串行。

此前STATE逐字保存于[完整历史](HISTORY-20261113-through-product-quality-and-sole-peer.md)，SHA256 `778295b28a3de7a9d18c732891cc257e2e8335a802d59701d68ad369420f134d`；该历史继续链接全部更早收据、失败、中断和旧协议。历史中的“在途”只按原时点理解，不重启已关闭批次。

## 工作树与边界

- canonical：`/Volumes/disk0/Khor/worktrees/current/khor-net`，branch `feat/net`。其运行时仍5064fe2a、iroh1.2/Noq1.3，后续文档/公开builder工具提交不能冒充升级已合入。
- 产品候选I：`khor-net-product-integration-v1`，branch `integrate/net-product-v1`，HEAD **117c39e5432741198c924177fae1775ff292eada**，tree **ace4fd8223ed7fcd8d2b29477d9e651a2df53ff2**。唯一parent为实际公开main **c3b098bdbf52a9b3b38e887ae74b90155292b544**，apps/和.github/逐字保留它；iroh1.4.1/Noq1.4.0。原merge链留refs/khor/net-integrations/product-v1-merge→1cd5092f。
- S：`khor-net-product-sole-peer-v1`，branch `experiment/net-product-sole-peer-v1`，HEAD **ade7decb4aa2d8616fb028b0b0221873d9d6f494**，tree **a448adc27f7502deb7293bbfa376d96de539df3b**。只在I上加两测试文件/模块声明，产品源码相同；先前固定三次已闭，不再追加相同Debug候选试验。
- 新Release配对树：`khor-net-sole-release-{baseline,candidate}-v1`，分支experiment/net-sole-release-{baseline,candidate}-v1，分别基于c3b098与117c39。仅共同测试夹具overlay，均staged；不可按旧摘要覆盖。准确source见下。
- Control树`khor-build-net` HEAD保持 **5d3691f252f9dee0297fa3c494374be6119bca29**；新控制只建Git对象与refs/khor/net-controls。E=`/Volumes/disk0/Khor/evidence/net`，root唯一串行写E/source-refs.json。

本机source-only：Git、stdlib Python源码/元数据/数据处理可用，不运行本地Cargo/Rust/Node/Bun/Xcode或下载的原生产物。重型验证用Turing/可信GitHub/macserver。main合并、发布、生产Relay采购部署、替换已安装App仍无本次授权。用户main/真实App/其他任务Lab23961不动；其源98285及服务PID4769/exec18781保留。

## 当前Turing：Release同条件首业务对照已闭

协议 **E/sole-peer-release-comparison-plan-v1.json**：公开产品main与当前集成候选的全分支效应，共同`netlab_sole_peer`独立harness、同一lab.rs observer/owned取消、原sp1业务函数逐字相同。只overlay测试文件，未把候选生产代码塞到baseline。Release原配置strip=true/thin LTO/codegen-units=1/default opt3；独立稳定owned源码与target目录，双方只编译一次，再固定 **A B B A A B** 各3次，零替代试验。原首目录5秒门、原生产hop3秒预算不改；所有错误/超时/清理失败保留。不作全局p99、全部N3、单补丁因果或真机/能耗结论。

| arm | source tree / snapshot | base | 角色 |
| --- | --- | --- | --- |
| baseline | 62f3e470c666bdf328bca1358201ce649e7e142b / 2987c9efd7d87fe876ceda7bd1004227d329db1f | c3b098 | iroh1.2.0/Noq1.3.0 |
| candidate | 28f04f6b10211a078a05377a8749e7d303b6c42d / 718629a5d931014355235444d37672a45e74bea4 | 117c39 | iroh1.4.1/Noq1.4.0 |

描述符E/sole-peer-release-{baseline,candidate}-source-v1.json。公共夹具四文件SHA与零runtime执行前格式归一记录在协议；独立root绕过无关场景的版本API差异，不改所测业务、物理拓扑或验收门。

两arm compile-only均已完成并闭证：baseline025353Z-e298b390 **0/630.716674s**，candidate030746Z-850f325b **0/655.111898s**，各17原reportfiles、原native verifier的Release test/同producer CLI、opt3/版本/source/hash/identity/group空核验，runtime零。稳定owned源码/target隔离，无计时中编译。runtime control **ed2ccef843a4ed362d369fec6b70d43a3c9b029a** 只重核原来源，复用已过的format与产物，不再Cargo/formatter。

**Release A B B A A B六次已全终态、收件/identity/业务+所有权审计及ledger闭**：[SOLE-PEER-RELEASE](SOLE-PEER-RELEASE.md)，E/sole-peer-release-comparison-verification-v1.json。旧源3次均首目录5秒超时；候选3次实际返回 **345236/366587/366320μs**、经唯一M的Direct/Relay两腿。各case独立新Lab、前态Idle/no_relay=false；所有失败保留，额外试验0，无强制/遗留。旧完成时间是5秒右删失，不能造成功延迟/p99或有限倍数加速；首A与其余约10分钟间隔如实保留。

串行coordinator E/ops/run-sole-peer-release-pair-v1.py **exec72887已终态0**，progress/result均completed，该批结束时无Turing在途/pending；其后共存coordinator状态见下，不恢复旧批，旧61277不恢复。原UID1110/flock/jobs16/nice10/50GiB启动/40GiB停止/90分钟守护保持。相同已闭Release或Debug候选不追加挑绿。

## 当前Mac原生候选：同源重跑通过，包已核、不启动本地App

公开builder完整Lab **43355810513 attempt2 success**，request ad847c8fb3874591bf786afd5c347a24，source **117c39**/tree **ace4fd**，snapshot4140e11e727ee0d43e859a6c1dd8d4659d793744，native_contract71e64768d0b001dcf6f8c45deaab341e845b226ae3d58443264bdb3e083ac4d7，port23963，control9b671817。准确报告 **E/public-native-lab-net-product-v1/lab-acquisition-verification-v1.json**。

首次checks WebKit桌面settings稳定性15秒超时完整保留；只重跑失败job一次，同源/断言/预算零修改，根因未证。attempt2全部4job成功，77完整App浏览器case零errors，hot/CSS更新/草稿门过，chat取消28/poll12/recovery22/scroll12原case均过。79处本地存在的报告源码hash与I一致；10处生成绑定未在本地源码出现，明确未以本地hash冒充核验。新checks ZIP41,651,437字节/SHA1130ab59及旧失败ZIP35,419,759字节/SHA06c455ba均留盘/CRC过。

API给继承job新ID；已核新IDnative日志仍是03:05–03:19及原artifact14006788158，frontend/native artifact ID和digest均未变，非重建。原生inner native.zip108,065,996字节/SHA2ca9754e及ZIP CRC、Mach-O arm64字头/可执行位、Info.plist独立Lab ID、准确tree/snapshot/contract/port和远端real_backend probe均核。整个获取exec96116终态；callback只记盘，无manager/下载native本地执行、无替换安装App。不是用户或物理iPhone接受；旧3b Lab不代替本源。

## 已闭证据及其范围

- **产品完整Quality**：run **43348869616**，source117c39，suite=all/interaction_scope=all，FIRSTATTEMPT13/13 success（原生Node/真实签名SSH安装认证、GUI、Lab、Agent、Mac updates/WebKit、iOS原生UI、三OS runtime等全部原门）。E/net-product-quality-v1.json与integration-v1.json为准。8原artifact保存在E/public-quality-net-product-v1，全部GitHub原SHA/字节/ZIP CRC过，archive-retention.json；尚需按具体用途核payload，不运行其中二进制。不是候选手机/移动网络接受。
- **唯一middle实际Debug合同**：[SOLE-PEER-PHYSICAL](SOLE-PEER-PHYSICAL.md)，run022908Z-54fab5de，control9f6d4d09，0/78.088398秒；三次首目录531136/524952/516796μs，原Rust各1/0/0，初态Idle、no_relay=false，目标独有marker、实选经M、Direct/Relay两腿及XRelay/YUDP实际drop counter均过。20/22/21个owner、无强制/遗留，18原files/hash/identity/专属proof/ledger闭。固定3/新增0。formatter020722及两个controller preflight022103/022453均零产品runtime、原失败保留；runtime v3只用已录producer环境和private TMPDIR，原Scratch清理，homes不保留而完整日志/owners保留。
- **真实新旧混版**：[IROH-MIXED](IROH-MIXED.md)，011900Z-19b49119，controlde74a134，0/160.457763秒；原row/pin双向、old/new host及sleep child实见并干净退出，Rust1/0/0，23files/source/hash/identity/group空，ledger闭。只是两懂Versions版本的窄互通，非pre-Versions fallback。旧preflight/缺new-host见证失败不抹除。
- **策略长稳**：baseline155449Z-56a8ac18与恢复后candidate180542Z-c09f6a3c各5/0，600秒原窗口；28journal与22CPU记录、32逻辑目录请求各全ok。p1s最大4690364→868648μs，但p1h包+33.970%/字节+37.908%、p5最大828673→920018μs；描述性E/iroh14-policy-stability-resume-results-v1.json，不宣称全局最优/p99/能耗。原用户取消164807部分保留，旧coordinator61277不恢复。
- **3b恢复ACK顺序**：[IROH-RECOVERY-ORDER](IROH-RECOVERY-ORDER.md)，实际红只新增first-ACK合同；绿Noq467/iroh198+原ignored1、12app276+原ignored1，固定3次物理124各全ok/两门过，六Go3.532–3.798秒。此批已闭，不追加同源挑绿。早先严重红与后续未复现并存，完整收据见历史。
- **上游/custom**：最近官方latest仍iroh1.4.1；E/iroh-upgrade-patch-ledger-v4.json的60历史项为1上游替代已验证、2移除对照红而保留、57待退出核查，绝不是57项均必要。iroh21容量64/Lagged对账、同interface地址-only iOS callback去重仍需行为证据；静态审计不冒充故障根因或补丁必要性。42–44旧隔离方案未合。
- **公开builder已恢复**：用户明确公开colehank/khor-build，public/admin已核，原历史保留；README9b671817。共享require_source_builder只对这一个已授权公共builder放行，45044工具验证43283055508的32tests过。旧3b完整Lab43282299770四job成功且原archive/native_contract/source核，E/public-native-lab-3b-v1；它不是最新GUI候选。旧私库额度失败和macserver SSH banner timeout仍留历史。

## 完整目标的剩余交付

N0–N5保持原范围，不把13个Quality或3个namespace病例当作全部接受。接下来依次：

1. 当前同夹具Release ABBAAB首业务对照已闭，接续原结果、不重复提交；进一步补缺失/过时hint和负向策略等原NET-16范围。
2. 在最终候选上完成发行配置目录/真实TTY/大文件/同步共存的分段长尾、公平和取消/资源成本；之前旧Debug对照仍有效于其原问题，不能冒充该项。
3. 唯一middle缺失/过时第二腿hint、旧Direct/失败冷却及策略负向，双方/peer-relay IPv4/IPv6/NAT变址和故障域；已有单元及部分物理结果不替代整个矩阵。
4. 核最新原生Mac Lab具体payload并准备隔离验收；物理iPhone仍是旧安装版，尚无候选真机/能耗窗口。MacLab/Turing同LAN，构建macserver另机；不把UI模拟器或旧App恢复当新源接受。
5. 逐项custom退出、状态/职责与中继/寻址/容量运维设计收敛，持续短回归与完整矩阵，最终准确源完整Quality及原生/真机/对照报告。采购/生产部署/main合并/发布按独立授权处理。

旧成功仅复用于其准确源码与范围；原始数据、产品红、控制前置失败、用户取消、未验证分别保留。所有运行/证据先查现有receipt、源码tree与真实进程/Actions状态，观察超时不等于任务终止，禁止盲目重提。

### 下一轮Release共存准备

在已闭首业务两配对树中只加入共同netlab_workload_release测试target及当前workload.rs与8个辅助文件；旧原夹具/receipt保持。已核公开main实际拥有wire_counts、pull_path、open_on、close_on API，**不采纳摘要误报“无wire_counts”**，双方共同workload业务体、observer及wire计数无需修改。尚未发新Turing/未有该批业务运行；协议冻结后复用已owned稳定source/target，原native-provenance prior receipts只允许准确匹配依赖，避免冷重编。目前candidate compile040254Z-2e151f7f在途（exec43456仅提交已终态0，真实远端PID3206329）；root仍唯一ledger writer。

### Release共存协议已冻结、增量编译推进

E/workload-release-comparison-plan-v1.json固定公开main对117c39，12份共同root/observer/workload/calibration文件逐字相同，生产overlay零。两网络模型各固定ABBAAB、每arm每profile3次，720逻辑样本/case，原2秒目录/TTY、120/240秒bulk、16/192MiB与所有重叠/完整payload/close/owner门保持。preflight双方各两个owned取消guard、low和split-high独立TCP校准；校准红停止性能比较。高模型最坏观察窗口在执行前声明owned harness cap600秒（原通用300不足覆盖240+2×48+setup），不改业务门。失败/取消/观察缺失全保留、替代0；有限n经验分位与工程退化筛查，不称全局p99/能耗或完整N3。

编译control v1的旧tree绑定被生成器误替换，035604Z-a2e3b97d在assert前置1/23.839862秒退出，产品编译/运行/迁移零；原failure及identity完整闭账。v2 control35c9d2ce修正旧/新binding并作静态对账，原native verifier未改。baseline035824Z-400ee7e2 **0/220.928664秒**、19files全核闭账；18first-party依赖通过原prior verifier复用，只CLI/新test重编，较上一轮冷编630.7秒短，CLI SHA完全相同。旧first-business两ELF先只读存盘/核hash；owned稳定source只切共同测试fixture。当前candidate040254Z-2e151f7f正在同法编译；两产物过后一次冻结runtime binding，再执行原preflight与固定配对，计时不编译。尚无本协议业务结果，不重开旧sole-peer批次。

### 当前唯一ledger writer：共存串行coordinator（请先核真实状态）

**E/ops/run-workload-release-pair-v2.py，exec45051，PID71819** 正运行，持专用local flock。root此间只读E/source-refs，不能并发stage/close；如coordinator终态才恢复writer。E/workload-release-comparison-progress-v2.json与results-v2.json是唯一执行索引。当前按baseline preflight040811Z-091f51be→candidate preflight041049Z-1d454085→固定A B B A A B，每business block low/high两fresh；替代0。不盲重提、观察超时不当终止。

两个Release compile已闭：baseline035824Z-400ee7e2 0/220.928664s，candidate040254Z-2e151f7f **0/212.873642s**；双方18first-party依赖按原verifier prior复用，仅CLI/新test重编，CLI SHA仍各与首业务旧ELF完全一致。新source基线f6c239ae/e22263d6、候选adbc7d6a/5dccbfd2（tree/snapshot，准确40位见E/workload-release-{baseline,candidate}-source-v1.json）。runtime control **9532c63b**按已闭compile SHA冻结，原owner/caller门及source-only保持。

原未执行runtime协议v1把旧冷TCP平均吞吐误作唯一模型准入；首次物理case前已逐字留存并以 **E/workload-release-comparison-plan-v2.json** 明确纠正，沿既有MODEL-DECISION分层准入，并增加每arm独立UDP case，preflight共5个（2owned guard+lowTCP+highTCP参照+highUDP）。不改原TCP断言/区间/退出结果；若仅旧冷TCP平均吞吐门红，仍原Rust101/原controller1，不冒称其绿；必须完整TCP双payload/68echo/server/300ms RTT/实际几何和独立UDP人口/pacer/容量全验才允许应用测量。队列/burst/datagrams与旧校准逐字相同；capacity仅capture可见性变pub(super)、正文同，双方仍做fresh校准，不用旧数据代新。

baseline preflight040811Z-091f51be已原件收齐/identity/独立data审计/ledger闭：两个真实取消guard与lowTCP、highUDP通过，highTCP原红，平均35.541/35.918Mbit，RTT300229μs、完整70操作/双payload及几何过；UDP两个方向steady48.30912Mbit、每方向145833/145833发送/无pacer skip，全部丢包分母保留。原return1不改；专属proof给**分层模型准入true**，非产品/完整N3接受。candidate preflight041049Z-1d454085在途，以实际progress为准。之后每case原raw人口、超时、阻塞、取消/清理必须全留，缺人口不能补零或只挑成功算p99。

### 实时关键红与下一批源码准备（2026-11-13T04:33:54.786857+00:00）

Release business首baseline041330Z-0ae9a3ec low/high均过。首candidate041753Z-362039e0 low过，**high原Rust101**：720原逻辑记录为600ok/2timeout/118blocked；bulk terminal index1与directory index2各超2秒，其后该phase118 terminal记录阻塞，16/192MiB实际完整payload照实保留，不能抹掉原红或把blocked零耗时当低延迟。后一candidate042344Z-6df2674e两profile原绿不抵销首红。原始/独立metadata全核，所有owner干净；coordinator仍按原固定顺序继续，不增替代轮。root曾在未读首candidate最终proof时错误更新“首组候选通过”，已向用户明确更正。新候选**未接受**。

失败发生在bulk初段；客户端借用probe在三个phase最大lag1857/2366/3266μs、相邻tick最大20.8/20.9/21.9ms，不支持客户端Tokio整体2秒停顿这一解释，**不证明resident/传输/整机原因**。终端断开/重连也未由该数据独立证明。先闭完整固定人口，再设计单一诊断/修复对照，不因后续绿忽略红。E/ops/summarize-workload-release-v2.py已加完整分母、失败/缺失拒绝误判、有限n nearest-rank分位及完整文件独立计量；两个业务超时不能把真实完整文件误记未完成。stdlib合成边界守卫通过，非产品运行。

新test-only树 **khor-net-sole-cache-v1 / experiment/net-sole-cache-v1** 基于干净117c39，三新测试文件staged（未提交、未format/编译/运行）。E/sole-peer-cache-physical-plan-v1.json在新物理case前冻结4case×3fresh：unusable第二腿hint（明确null不可解析、非字面删除字段）、stale Direct第二腿、X历史Direct、真实M暂停Y→X实际失败→恢复M-Y后新显式X读取；只用现有公开DeviceDoc/store与pause API，真实Direct/Relay两腿/隔离/唯一marker，hint前后保持，无prod hook或适配transport。恢复阶段从unpause开始同一总5秒，含middle-ready与最后读取，X PID不变；初始失败不得重试重放写。该批**只准备源码**，不能冒充NET-16已过；真正缺字段、多middle、递归/未信任等其余矩阵仍待。N3 coordinator运行期间不在Turing编译/测试/动cache，不动性能源I/B/C。

### Release双模型ABBAAB完整闭证：候选被拒绝（2026-11-13T04:45:50.939826+00:00）

[WORKLOAD-RELEASE](WORKLOAD-RELEASE.md)及E/workload-release-comparison-verification-v2.json绑定全部8run/12业务场景/8640记录/12完整file。coordinator **exec45051/PID71819已终态0**，progress/results均completed，ledger无pending，root恢复唯一writer。原fixed A B B A A B全部保留/额外0；不得再重开相同候选批次挑绿。baseline low/high各3/3；candidate low3/3，high**1/3**。第二红043619Z-e3822070为idle_after terminal index5超2秒及其后114blocked；连同首红，全high候选1925ok/3timeout/232blocked，完整文件三次均成功。low分段工程筛查过，high失败；median高file19.3299→22.4087Mbit/s不抵销交互退化，idle_before TTY计划至完成经验p95 0.303→8.229s。**117c39不具备完整网络接受，勿晋升canonical/main。**

下一优先定位上述Release高BDP退化，先独立诊断，不先改重试/期限/样本或凭一次绿收口。test-onlysole-cache源码准备保留待测。独立诊断树 **khor-net-workload-qlog-v1 / experiment/net-workload-qlog-v1** 基于117c39，普通qlog feature默认关；拟通过上游iroh/Noq qlog能力捕获whole high phase实际恢复/拥塞（新原红还含idle_after，不能只盯bulk启动）。目前仅源码准备：khor-net/Cargo.toml增加qlog feature、transport builder受cfg调用qlog_from_env，测试传每resident隔离QLOGDIR与epoch壁钟/单调锚点；无vendor算法改动/新重试或timer。E/workload-release-qlog-diagnostic-plan-v1.json预先限定三fresh诊断，不能代普通性能；须先remote fmt/lock/deps/trace体积准备再冻结source/producer，当前尚无该诊断产品运行。原1GiB expanded/512MiB压缩collector及40GiB/owner门不降。
