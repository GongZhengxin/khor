# feat/net 当前状态与接续入口

更新于 2026-11-13T03:52:37.515590+00:00。活动目标仍是[GOAL](GOAL.md)和[PLAN](PLAN.md)完整N0–N5，**未完成**。上轮是实际进展：准确产品源完整Quality13/13及真实唯一middle固定3次均已闭证，不是整体接受。root串行，不派发子智能体。

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

串行coordinator E/ops/run-sole-peer-release-pair-v1.py **exec72887已终态0**，progress/result均completed，当前无Turing在途/无pending ledger；root恢复唯一writer，旧61277不恢复。原UID1110/flock/jobs16/nice10/50GiB启动/40GiB停止/90分钟守护保持。相同已闭Release或Debug候选不追加挑绿。

## 当前Mac原生候选：最新集成源，不启动本地App

公开builder的完整Lab **43355810513**，request **ad847c8fb3874591bf786afd5c347a24**，source **117c39**/tree **ace4fd**，snapshot **4140e11e727ee0d43e859a6c1dd8d4659d793744**，native_contract **71e64768d0b001dcf6f8c45deaab341e845b226ae3d58443264bdb3e083ac4d7**，port23963；control main **9b671817e34846df9e14c213a562ab88309cb45a**。source/request已写E/public-native-lab-net-product-v1/request.json，API核workflow_dispatch/准确control，原生/前端成功，整体检查存在下述失败，尚未接受。

本地仅source快照/上传和artifact数据获取进程，**exec96116已终态0**，脚本E/ops/acquire-product-native-lab-v1.py。它调用原builders.remote，callback只记盘，不建manager/不启动下载的native。前端/原生artifact均已获取，acquisition-result.json明确failure；callback未启动本地产品。不重复提交build。builder.log、acquired-*.json、acquisition-result/error.json都在该目录。原native/frontend/retire三job成功；checks在WebKit桌面settings导航data-nav-me元素原15秒稳定性等待超时。62个已落盘浏览器case无errors，原完整检查log与35,419,759字节ZIP/SHA/CRC保存在failed-checks.zip，未隐去失败。无UI源码/断言/期限修改；仅对原失败job131659939275发起**一次**同源受控重跑，API请求范围仅原失败job；attempt2最新job列表也出现新ID的native/frontend成功job，其实际是否重建待logs/payload核实，不预称未重建。见checks-failed-job-only-retry-v1.json；attempt2 checks job131672404119仍在原完整App/WebKit步运行；后续核它，不另造build。旧3b Lab不能代替本集成源。

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

在已闭首业务两配对树中只加入共同netlab_workload_release测试target及当前workload.rs与8个辅助文件；旧原夹具/receipt保持。已核公开main实际拥有wire_counts、pull_path、open_on、close_on API，**不采纳摘要误报“无wire_counts”**，双方共同workload业务体、observer及wire计数无需修改。尚未发新Turing/未有该批业务运行；协议冻结后复用已owned稳定source/target，原native-provenance prior receipts只允许准确匹配依赖，避免冷重编。当前无Turing pending。
