# feat/net 当前状态与接续入口

更新于 2026-11-14T02:46:50.630610+00:00。用户已授权持续实施/远端验证，活动目标“好 完成完整目标”仍未完成。root串行，不派子智能体。完整[GOAL](GOAL.md)/[PLAN](PLAN.md) N0–N5范围保持，逐项缺口以[REQUIREMENTS](REQUIREMENTS.md)为准。未获main合并、正式发布、现装App替换或生产Relay采购部署授权。

此前STATE逐字归档[HISTORY-through-relative-margin-v2-unit](HISTORY-20261114-through-relative-margin-v2-unit.md)，SHA256 `c772dce4a855e181e700130c5805801f3be5e3451968af6f35ad38cef5264406`；其历史pending/PID不能用于重派。更早原件由该文链接继续保留。E=`/Volumes/disk0/Khor/evidence/net`；worktree均在`/Volumes/disk0/Khor/worktrees/current/`。

## 当前唯一Turing执行及下一步

唯一pending **turing-20261114T024533Z-50822f42**，net-node / h2_lifecycle_case_1_v2，role diagnostic。control **30f054bebf4409cda005807368af850a967a67e4**，protocol **c16843dba23e6ccca3956ab0f3f936ab22794ecad0a81a854d385f2b218258f7**；先查receipt/execution，禁止重派。

前版023714Z-08b04294已collect/identity闭，1/5.698953秒，**native0/compile0/cases[]**。采集的三个文件合同校准全过，但KHOR_NET_DIAGNOSTICS在原逐场provenance前设置，环境哈希不匹配，正确拦截；不是产品红或有效性能样本。E/该run/lifecycle-preparation-verification-v1.json、原完整日志与v1计划/control保留。v1三场未执行，不跑其2/3。

新v2只改诊断控制入口：全部原provenance断言在精确producer环境执行；仅其后owned.run期间开启NETLAB_DIAGNOSTICS=0、KHOR_NET_DIAGNOSTICS=lifecycle，finally恢复原环境。没有删字段或改来源判定、Rust/fixture/参数。E/h2-lifecycle-shadow-{plan,control}-v2.json，固定3场，无native替代试次。复用H2 db21c2beeecb1d6b1823589267b2f98d34fa5e38/tree5cf730ab964c52fd613953251000ba2057e0f2bc/snapshot67b205f2d7e55eafb02d81b3a5be1f672669f948及011935 producer普通Release原ELF，不编译。

外部只读observer在原400ms循环持有私有serve.log fd，原owner全结束后再drain，保留Scratch unlink尾部；每场原unlink后写/换inode/外部symlink三校准，最多8份×16MiB。已有普通binary生命周期钩子，无feature诊断subscriber/qlog，开关另有只读tuple/短暂weak升级；因此日志扰动必须声明，不作性能/能耗验收。

每场原high wh2 **720结果/192MiB/2秒caller/240秒文件/owned600秒**保持，复用同源013456模型准入，原coldTCP红保留；实际geometry/CPU/原生来源/owner清理/全部人口仍逐场验证。先闭首场来源、采集和清理，再顺序派v2场2/3，前驱上一receipt；原功能红保留，只在调度未知、清理或采集不安全时停止，无重试至绿。

下一步：查024533 actual execution；完成后collect-network-ssh.py及verify-turing-record.py。新data-only verifier复用scripts/netlab-workload-wires.py解析概念，报告全部720人口、实际360目录/768切片的缺失/额外/重试，按真实ALPN/wire/stream/offset配对。不得减两个p95或跨未同步进程clock；server从accept开始，local write完成不是delivery/ACK，残差不得独断网络根因。root唯一ledger writer，无后台协调器。

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
