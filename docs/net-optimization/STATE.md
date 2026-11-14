# feat/net 当前状态与接续入口

更新于 2026-11-14T05:00:02.994460+00:00。用户已授权持续实施/远端验证，活动目标“好 完成完整目标”仍未完成。root串行，不派子智能体。完整[GOAL](GOAL.md)/[PLAN](PLAN.md) N0–N5范围保持，逐项缺口以[REQUIREMENTS](REQUIREMENTS.md)为准。未获main合并、正式发布、现装App替换或生产Relay采购部署授权。

此前STATE逐字归档[HISTORY-through-relative-margin-v2-unit](HISTORY-20261114-through-relative-margin-v2-unit.md)，SHA256 `c772dce4a855e181e700130c5805801f3be5e3451968af6f35ad38cef5264406`；其历史pending/PID不能用于重派。更早原件由该文链接继续保留。E=`/Volumes/disk0/Khor/evidence/net`；worktree均在`/Volumes/disk0/Khor/worktrees/current/`。

## 当前唯一Turing执行及下一步

唯一pending **turing-20261114T045906Z-abc0de50**，net-node / separate_pull_standard_node_v1，role candidate；control **5d3691f252f9dee0297fa3c494374be6119bca29**（原标准12组不改）。先查receipt/execution，按实际状态collect；禁止重复launch，原非零仍完整收集并保留，不把identity当行为通过。

worktree **khor-net-separate-pull-v1** / branch experiment/net-separate-pull-v1，准确formatted HEAD **dad4306132a51e1807da2058988e13f315b262ac** / tree **0db6508aa2bd2094b9dfcfee6c35715a0af20f17** / snapshot **0a97f374b33f6d7a3b5840b48c55fbf001cb7700**。E/separate-pull-source-v1.json，基于已接受I4＋N2并带纯测试IPv6夹具修正4d6341a1；产品原型1196137a和格式dad430，准确源保留全部既有产品修复，无H2/PULL4。

这是**单文件架构实验，默认原共享连接**。内部KHOR_EXPERIMENT_PULL_WIRE=separate或私有layout入口启用一条额外现有Main ALPN QUIC；沿用原Endpoint/有界dial/2秒caller预算/最多一次读retry，不扩写或0RTT。RAII完成/错误/取消关闭；原cached Main不替换；Weak side新增transfer_owned位，retire_peer同时关闭matching extra，其他peer不受影响；dial前后检查pause/rest。尚未采用、未发布、未跑性能；多文件caps/公平/完整mode-ON旧合同/真机成本仍缺。

**接下来先native组件门**：E/separate-pull-mode-on-contract-plan-v2.json固定原286＋7=293 pass/0 fail/原ignored1，原12命令、全部原test名逐一核对。新7个真实QUICmode-ON测试为drop extra/other-peer负控制＋peer pause/side cleanup/实际完整文件完成＋取消/读retry1/写不重放；旧mock默认shared，不称完整mode-ON覆盖。collect→verify-turing-record.py→verify-separate-pull-standard-node-v1.py；若非零先看准确compiler/test错误，不改原期限/旧断言或跳过合同，保留原红再做具体修复。

S formatter **044431Z-f8888b8a** 已0/45.951713秒，11files/identity/owner/source已核；原fmtcheck1及patch SHA7d3d82c247560c6c39f8a5c7d9d35dc7822923ad245a35fcce6b0bac32c68539保留，仅link.rs/isolation.rs格式，compile0/native0，已经准确apply/commit。此时不再重派formatter。

只有组件门通过后，再普通Release严格producer；V最近闭证producer043812可作准确prior，当前稳定candidate release cache source/tree1cbe5c0dfea1f838754fcf6d74145d5e4ef3c1dd。S相对V仅4个node文件（2runtime＋2test）改变，netlab/workload/vendor/validator源保持；必须完整source/ELF准入，不复制fingerprint/intermediate。本机source-only；重型编译/计时Turing串行，不在timing旁启动Cargo。

后续已预先设计但**未stage/运行**：E/separate-pull-lifecycle-design-v1.json固定shared/separate各1个真实high日志诊断（原720读/192MiB/全部门），只证明实际不同Main及独立Tunnel，复用原ServeLogShadow与3项采集校准；runtime flag只在provenance/CPU后owned.run期间设置、finally恢复。再填准确producer/controller冻结E/separate-pull-three-arm-design-v1.json：A原public main、B同prototype ELF shared、C同ELF separate，ABC/BCA/CAB九块各原low/high，共12960读/18file。C对A原相对10%+20ms/90%率及固定original-main绝对门都保持；B只作架构对照，不能为C降门。原hard/CPU/geometry/owner/futility stop与坏样本全部保留。尚无原型性能结论。

### 纯IPv6固定功能矩阵已完成

详[IPV6-ONLY-MOBILITY](IPV6-ONLY-MOBILITY.md)，E/**ipv6-only-mobility-verification-v2.json** phase=completed：**15/15场、五类各3次、630/630真实读取成功**；双向首次最大1705510μs，原2秒门保持。全部内核old移除/new实际UDP源、无非loopback IPv4、IPv6 TCP、owned路由/原attempt人口、CPU/PSI/owner清理独立核验。强制Relay3场各首次约0.507–0.509秒；nft allow独立reflector39999＋block39998正负控制、实际readback与counter、两侧Hop::Relay门都保留。

准确产品CLI全程**443c6e010293b83e7c0e963b6eb96edb3897dd4c0d6ada0bca9f6fca02666a43**，产品/vendor变化0。前4夹具源f3a62316/4ea140与后11 literal-only修正4d6341a1/1cbe5c除nft字符串逐字一致；E/ipv6-only-functional-carryover-v2.json已核，只作功能carryover、不混算同ELF性能。初版E0308编译040023（native0）与v1第5 nft语法失败043417（业务0）全部原件保留；v1当时停止invalid preparation，设计v2先冻后修单literal，非删坏业务样本。

V source4d6341a1/snapshot4e610bc70eac4714893c614be30ed17072b30ee9；普通Release producer **043812Z-bfb5fd1c** 0/244.923981秒、23files/identity/严格来源已闭，实际source nft先在独立user/netns实装/读回0，CLI同字节、testSHA187f84874dd99a5e4532b13ffa63564afa49ef59154600e7390abe8b0bee5282。runtime-v2 control **afd4f2792e09f6e16c5829f5579c46863a8ab7ff** / protocol6e46b9a4cadc25d70bd8075c77ab200fb433d23541654a4d7d06cac41a098ad2，固定5..15全部闭证。最后15 **045718Z-60f01e1e** 双prefix首次1516369/1705510μs；全部run/结果详上述报告，不能重派矩阵。

边界：IID add/DAD/delete可能有地址重叠；Relay-only在配对前已隔离UDP，证明已建立Relay换址重连，不代初始Direct突然断UDP。未代NAT64/DNS64、冷新peer发布发现、睡眠/真实Wi-Fi↔蜂窝/peer relay移动/手机能耗/Relay容量与故障域。**完整N4/N3/N5未完成，活动总目标保持**。

### 准确接受源I4＋N2完整混合对照已闭：功能全绿、N3仍红

E/**contact-current-mixed-verification-v1.json**、progress-v1 phase=completed；plan/control-v1在native业务前冻结，control2cc1ad6578c83a4384335e825f2ba30db650439b，protocol13b66d93d03519417f0b20c956cbf697ad968f8044d502995c16727d81bc4f51，design SHA57d5827b8348b70a4cc9f81510d3c478a4cfb8030783fc80ada54ded79ac1afd。原ABBAAB六块×low/high、12case、**8640/8640ok、12完整file**、全部来源/CPU/geometry/owner已闭；无重试或替代、计时0compile/日志开关关闭。

baseline仍冻结原公开main runtime c3b098bd/treef6c239ae/dirty=true桥接快照e22263d6（035824普通Release ELFs），candidate准确**96e999da/tree41df16ad/snapshot1cc2cb53**（I4＋N2），无H2/PULL4。candidate producer **030058Z-e97c48c8** 0/570.415930秒、Cargo8m30、22files，CLI443c6e010293b83e7c0e963b6eb96edb3897dd4c0d6ada0bca9f6fca02666a43/full netlab01b6432ae381f633d98a4a1fb6af84b9a5e91fa1075f65c88f371ec14f53dfbb，features[]/原thin LTO、强制refresh且fresh=false、46原provenance合同source一致复用。完整source_frame13路径/其中native5已核，新增node单元fixture与netlab11不变区别明确，I4 browser lock保留。

两臂preflight **031149Z-be538813 / 031537Z-b18b6b98** 各17files：原2owner/lowTCP/UDP四绿，原high coldTCP红保持，按既有MODEL-DECISION分层准入，非全绿。业务6块如下（每块含原low/high各720）：

| 块 | arm/run | 原结果 |
| --- | --- | --- |
|1|baseline031906Z-28dfcb1b|0/279.047954秒，1440ok/2file|
|2|candidate032427Z-7237c5a7|0/239.000680秒，1440ok/2file|
|3|candidate032931Z-531d7086|0/242.931865秒，1440ok/2file|
|4|baseline033418Z-7450f864|0/258.920607秒，1440ok/2file|
|5|baseline033947Z-25c28bc7|0/258.780927秒，1440ok/2file|
|6|candidate034503Z-1460b442|0/247.020924秒，1440ok/2file|

所有low cell、idle两profile与文件率门均过。**high bulk目录计划→完成p95 baseline546493→candidate1142002μs、TTY351134→715597μs**，相对10%＋20ms与原固定绝对592518.1/391538.2μs门均红；high文件中位18.742678→23.288474Mbit/s不抵销交互回退。candidate三轮high目录628558/389794/1807685μs、TTY517555/302533/1357564μs，不能挑第二轮绿接受。

最后一块record得到固定360人口中目录52/TTY37条越绝对门（最多18允许）。这是**完整cohort最后才触发**，recorder明确保留rejection_at_complete_cohort且phase completed，让summarizer输出完整红；只修完整性呈现、不改任何阈值/样本/原结果或重跑。早停规则原样适用于还有未来块时，历史H2停规则未追溯修改。

源后journal：candidate前两high场分别2/0次selected变化，最后high4次；y有约334ms Direct质量/301ms Relay质量的选后快照，随后Relay变477/634ms又回Direct。**不是选择决定输入或业务包路由证明**。H2两正式candidate场无selected变化，其中第二场目录p95仍850180μs，故“只要不切就解决”已不成立，不能盲加H3收益常量或当场改定时器。E/shared-path-fault-scope-hypothesis-v1.json只是假设；E/high-mixed-rate-queue-observation-v1.json原qdisc有taildrops且与尾巴不单调，不作BBR因果或丢包率结论。

N2窄修复Linux286/Mac282及14合同继续接受，**N3完整屏仍拒绝，完整目标未完成**。下一轮应解决共享/分连接与真实拥塞/调度，而非再重跑相同默认cohort直到绿；IPv6-only夹具独立准备不覆盖N3。全部旧red/无效准备/未执行保留。root唯一ledger writer，无后台协调器。

### 普通H2无重编译诊断已完成，不作性能接受

持久说明见[LIFECYCLE-DIAGNOSIS](LIFECYCLE-DIAGNOSIS.md)。

**024533Z-50822f42 / 024937Z-96d5dcf3 / 025345Z-16ed1d5c**，control30f054bebf4409cda005807368af850a967a67e4，固定3场全部collect/identity/原owner/来源/采集/业务人口闭证：**2160ok、3384实际响应stream**（1080目录＋2304切片），各16文件、runtime无compile、无替代。E/h2-lifecycle-shadow-summary-v2.json，逐场h2-lifecycle-shadow-verification-v2.json；E/ops/verify-h2-lifecycle-shadow-v2.py为data-only，匹配真实ALPN/wire/stream/offset。

每场目录+文件共一Main，终端另Tunnel。bulk目录client elapsed p95 355501/353847/344306μs，server accept→local write p95 1929/1373/1476μs，seat增量p95 4/4/2μs，open p95 65/46/66μs；逐RPCduration残差占主要耗时（不是两个p95相减）。计划→完成p95 381737/419509/429886μs。文件20.8028/17.4581/23.6164Mbit/s只描述诊断。每场2256业务快照均显示同一Direct、观测点0变化；不能证明采样间无切换或实际包路由。

结论只限这三个有日志样本：已测应用排队/处理较短，**没有重现原无日志p95回退，未定位其根因，也未接受H2性能**。server从accept开始，local write不是delivery/ACK；未同步clock不互减，残差仍含未测传输/accept/delivery/application区间。旧runtime flag另有只读tuple诊断/短暂weak升级、外部持fd采集有扰动，禁止与无日志性能cohort混算或称能耗不变。

准备版 **023714Z-08b04294** 1/5.698953秒，native0/compile0/cases[]，三个文件采集校准全过；flag在原逐场provenance前设置导致环境hash正确拒绝。原件/lifecycle-preparation-verification-v1.json及v1三场未执行记录保持。v2仅在全部原provenance和资源门之后的owned.run开启运行期flag，finally恢复；未改Rust/fixture或来源断言。每场unlink后续写/换inode/外部symlink三校准、私有8×16MiB日志持fd/owner退后drain均过。

### 刚闭的H2正式对照：未接受，不再派5/6

E/relative-margin-comparison-plan-v1.json，control429e1e87a686f8ea78fcde0cb666914203d50982/protocol2a72b1c8889a320fb50bafeb1434dfe6a1eaf6945774849ff710f7505a6d8a8a。两臂preflight013143/013456已闭分层准入。业务前4场均完整原native/CPU/owner/人口核验：

| 块 | 来源 | run | 原结果 |
| --- | --- | --- | --- |
| 1 | 原8/绝对margin | 013842Z-71395996 | 720ok，文件完整，0/184.597149秒 |
| 2 | H2 | 014404Z-c413459e | 720ok，文件完整，0/192.597185秒 |
| 3 | H2 | 014844Z-9003c9f9 | 720ok，文件完整，0/192.818851秒 |
| 4 | 原8/绝对margin | 015521Z-4f83dbcd | 646ok/2timeout/72blocked，文件完整，1/198.869200秒 |

按提前声明的baseline硬失败停止，progress-v1=stopped-baseline-hard-functional-failure；第5 baseline和第6 candidate未执行，不重派/不替换/不作完整6场paired结论。E/relative-margin-stopped-summary-v1.json。

H2也不能单靠相对收益门关闭N3：两场已观察的240条bulk目录中28条计划→完成超过原main门592518.1μs，原计划360的nearest-rank p95最多允许18条超界，即使第三场全好也已不可能过。PTY两场0超391538.2μs，目录每场p95 531354/850180μs、文件21.465/21.488Mbit/s；不能用功能全成功抵销目录回退。H2未采用，E/relative-margin-next-diagnosis-decision-v1.json。

历史response-priority-decision-v1.json已有低/高各18场（共25920调用）：-1低延迟有目录收益，高延迟未稳定改善，故当时把0固定为后续预取中性参照，非最终策略。不要凭记忆重做同一个常量；本轮先定位当前普通H2的client exchange与server seat/read/dispatch/encode/write及剩余未测区间。

## 已闭修复与仍然失败的方案

**N2旧拨号超时覆盖新认证联系已窄修复，并过完整Linux标准回归。** 准确96e999dae8312d86a38c08095ba58c3e1c1274e1/tree41df16ad51dc3955a8a6fdf3a494432d9f1a3dc0/snapshot1cc2cb5312430ffc4ba55c10ecd7f07ef1dd7a10，worktree khor-net-dial-contact-repair-v1。产品d15c2d1d仅link.rs/wires.rs，96e仅相同取消安全fixture。全局contact守Asleep，每协议握手代次守普通/early真实失败和等待者缓存；slots→presence→down同步提交，实际失败history/counters保留，普通流量不推进握手代次。三个失败出口均guard，无新timer/packets/tasks，不改原期限、PULL8或写/0RTT边界。

同fixture原版5066bfb9固定14：002222Z-fb33f5b6 101/196.613641秒，12pass/原2red；修复003059Z-d829e18f 0/196.561372秒，14/14，8marker、其他peer/无联系/新Tunnel与原6Asleep负控制全保，handler全join/cap-hit0。E/run/dial-contact-concurrent-{original,repair}-verification-v2.json。完整标准 **010353Z-7e1681fd 0/472.042894秒**，**21files/identity/owned**闭，原12组 **286pass/0fail/原1ignored**，包括IPC/取消/响应资源组；E/run/dial-contact-repair-standard-node-verification-v1.json。原T0两产品红和首修复串行fixture取消panic、格式准备失败全部保留，见[DIAL-TIMEOUT-CONTACT](DIAL-TIMEOUT-CONTACT.md)。仅这些准确源合同接受，不代全部N2或最终集成。

**H1全局收益门已拒绝。** f4b495e7/tree54404310，011408Z-8eea3f7a 101/46.143293秒：71pass/1fail/原1ignored，原path::tests::the_relay_beats_the_hop_only_on_measurement发现200ms Relay会清195ms hop。E/run/relative-path-margin-unit-rejection-v1.json；未改原断言，Release0，准备control539fbf6a未执行。E/relative-path-margin-design-v2.json先冻后才做H2，恢复清理资格、只限定正常迁移；H1所有原件保留。

**PULL4已拒绝，六场计划未完成。** 原8首场004921为720ok；4臂005307为676ok/2timeout/42blocked，005746为667ok/2timeout/51blocked；三文件完整、来源/原生/CPU/清理核验闭。三场goodput23.925/13.988/10.925Mbit/s。两个4臂硬失败后root等在途完整自然结束，取消尚未发起的4/5/6（8/8/4）。该早停是数据后的程序变更，原v3未预先声明，已在E/pull-window4-rejection-decision-v1.json明确；不作完整六场因果/pooled性能接受。原plan/control/全部坏样本保留，progress-v3=stopped-candidate-rejected。详[PULL-WINDOW-EXPERIMENT](PULL-WINDOW-EXPERIMENT.md)。PULL4不带入最终候选。

**原main↔F3完整同期N3对照是RED。** 六ABBAAB块/双profile共8640/8640读、12文件完整；high bulk目录p95 520471→963174μs、PTY337762→695394μs，均过原回退界限，吞吐18.657→24.413Mbit/s不抵销交互回退。E/current-mixed-comparison-verification-v1.json，详[CURRENT-MIXED-COMPARISON](CURRENT-MIXED-COMPARISON.md)。

路径线索E/high-mixed-path-churn-observation-v1.json逐核9场：原main三场selected-change均0，当前8三场2/4/2，另8首场0，4两场6/26。实际path-stream事件不是决定输入/业务包路由；缺统一wall↔workload时基，相关性不作因果。H2是待证假设，不提前称根因或最优参数。

## Mac 独立验证及磁盘入口

Linux已过的96e同源Mac标准net-node已唯一提交 **43540500760**（request7e5a0e2484914cb99b7f1c46d886642a，snapshotb516bfd2b818b81506d8a3a388803709a6315ceb，source tree41df16ad，builder feat/net68cadb57）。该builder的标准12命令块逐字Linux5d control，SHA25625cc8686fb1e5877bc0c80a2cd8b92de7c8a30eb130cb8ce83dbdfc8a259b90a。network-mac/job132279693750在原run-build门 **free<50GiB** 被拒，编译/测试0；不是产品红。原log E/dial-contact-repair-mac-attempt1.log，dev-evidence14091886017 SHA f178febcc3818a9f0c39568319fe59a275d6f0c008df1c808419db301724390c（2741B，ZIP大小/SHA/CRC后续已核，见attempt1-verification）。不原样重派，先恢复磁盘reserve。

只读盘点 **43540770433** main9b671817成功，artifact14092315786 12912B SHA01a8f196bca3464555b9507cb9ccaedd30bb2830085a8fafaa6f1ac654518424/CRC已核，E/worker-storage-inventory-43540770433.{zip,json}。free49.66156GiB、target83.55098GiB；incremental37.38995GiB/347目录，82旧变体符合现有8版本/2小时规则；另1个superseded khor_node test约254.6MB。当前installed build_guard SHA **998f5723eb447a1634411397c18a524e5ed0d50bd6dbd81001a16626219f64eb**，50/40GiB及target100GiB门保持。

已修正维护判定并完成受保护回收：隔离builder worktree khor-build-start-reserve-v1 / fix/net-builder-start-reserve-v1，source bfe8b0f3b4a602fe159c6651e0d8815e58b9d639，含受控workflow的HEAD fa246be（具体40位见E/mac-start-reserve-reclamation-verification-v1.json）。main未合，guard本身保持998f。唯一产品外修正将ensure-budget的提前返回储备40改为已有启动门50；原shared flock/lsof/新8版本/2小时/symlink/inode保护及磁盘门不变。

可信GH **43542416062** 首轮prepare/storage成功；原脚本仅新增49.5GiB真实流程回归红（其余5过），修正版6/6，原4保护测试未改。三个artifact大小/SHA/CRC与candidate脚本SHA322899ec2722aa159d64ac1162e57fff0e354aaea65e5d159655cbd1392ed269、实际控制commit相互核验。原host操作仅回收1个superseded test＋82旧incremental，free **49.6639→67.2096GiB**，target **83.5510→66.0158GiB**，admission changed=false且SHA仍998f；E/mac-start-reserve-reclamation-verification-v1.json。无App/用户文件/活跃Lab或main变化。

SSH只读hostname曾成功，但之后原生前的Python维护测试上传在banner阶段超时，执行0；E/mac-start-reserve-ssh-preparation-failure-v1.json和原intent保留，已走可信GH完成上述修正/回收，不原样反复SSH。

环境恢复后的同source/control Mac标准net-node **43542867922** 已首轮success并完整闭证：requestcb0220712d284ad48e30f92ef00595a9，snapshotb19a46316daf8a698ef4f976b69930c3319558e9/source41df16ad/commit96e，job132287202971。E/dial-contact-repair-mac-verification-v1.json：原12组 **282pass/0fail/原1ignored**，14联系合同全过，与Linux原名称逐一核对，少4条有明确cfg(target_os=linux)且Linux已过，不补写成286 Mac。artifact14093330987 38214B SHA005de796571e0c6ae5101835afce68961b9c7a134a2cb56ae2e6f695754a573e/CRC、全部11控制Git blob、snapshot/source/bundle/host ARM64/rust1.95/jobs2、installed guard998f和所有原日志逐项核验。本地只数据操作；Rust--nocapture的换行结果复用已验parser，首单行parser准备失败未改原数据或重跑测试。原435405磁盘拒绝原件保持，不算产品红。

N2该窄修复现有准确同源Linux286/Mac282及14合同接受；仍非Release性能/最终集成/手机/完整N2或N5。此处没有App构建/安装/发布。

## 仍有效的I4/F3与边界

I4准确4517d4cb/treecd52c290（khor-net-product-integration-v4）完整Quality43523128168 FIRSTATTEMPT13/13及可信native Mac Lab43523194833首轮4jobs/原77case全部成功，真backend/source/contract/arm64 host/ZIP已核。E/net-product-quality-v4-verification-v1.json、gui-verification-v1.json（实际27组/附件9、Chromium159.0.8154.6）、public-native-lab-net-product-v4/lab-acquisition-verification-v2.json。playwright-core1.65.1/WebKit27.2只在原四场固定诊断后更新，未改原5/15秒断言；I3旧WebKit26.5两完整Lab红和四场2/4崩溃均保留。详[WEBKIT-LAB-DIAGNOSTIC](WEBKIT-LAB-DIAGNOSTIC.md)。I4不包含之后N2/H2修复，不是最终源或手机接受。

F3 silent/healthy原固定6场168/168读成功、silent0.873–0.954秒、healthy原TCP全部保持；双端V4/NAT及dual/前缀固定6场252/252、首双向0.507–1.674秒已闭。完整来源和原失败在归档及[RELAY-DEMAND](RELAY-DEMAND.md)、[BOTH-ENDPOINT-MOBILITY](BOTH-ENDPOINT-MOBILITY.md)，不代IPv6-only/真机或全N4。

canonical feat/net产品运行仍5064fe2a（iroh1.2/Noq1.3）；本分支后续HEAD主要持久文档，尚未把已升级iroh1.4.1/Noq1.4.0的实验/集成树合到canonical。最终只带实证通过修改，保留I4浏览器锁、N2修复与main既有成果。custom历史60项为1已上游替代、2Noq移除红保留、1 iroh38实际NAT移除红保留、56待退出核查，不能说56均必要。

仍缺：N1完整多middle/置信/成本；N2全部取消/预算/写入及包级0RTT/NET16负向；N3共享vs分连接、多负载公平/1-3-5成本及原回退；N4独立/同时IPv6-only、发布hint/新连接、手机换网/睡眠/容量故障域及拓扑方案；N5逐custom退出、最终准确源完整检查、真实iPhone/能耗及最终设计报告。全部详细原要求留REQUIREMENTS，不据局部绿缩减。iPhone仍旧安装版，早前“就绪”已过期，真实候选可用后需新确认。

## 执行约束

本机source-only，不运行本地Cargo/Rustfmt/Node/Bun/Xcode/产品；Git、stdlib Python源码/数据/ZIP/hash与远端编排允许。重型仅Turing/macserver/可信GH，其他任务Lab23961和main不动。Turing UID1110/shared flock、jobs16/nice10、Rust1.95、磁盘50/40GiB、私有user/netns、计时前CPU>=80%idle/pressure<=5/无同UIDCargo；编译与计时串行。Mac共享worker jobs2，原50/40GiB、target100GiB不降。root唯一ledger writer，无后台协调器。

先读本STATE与PLAN，查实际worktree/HEAD/diff及E/source-refs.json；不要按历史pending/PID重派。准确候选图谱无覆盖（0不是低风险结论），已先核main图谱再定向读实际源；不自行建立索引。当前已有单元通过不代Release业务、Mac或真机；不停止完整目标，不标goal完成。
