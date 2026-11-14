# feat/net 当前状态与接续入口

更新于 2026-11-14T03:13:13.428229+00:00。用户已授权持续实施/远端验证，活动目标“好 完成完整目标”仍未完成。root串行，不派子智能体。完整[GOAL](GOAL.md)/[PLAN](PLAN.md) N0–N5范围保持，逐项缺口以[REQUIREMENTS](REQUIREMENTS.md)为准。未获main合并、正式发布、现装App替换或生产Relay采购部署授权。

此前STATE逐字归档[HISTORY-through-relative-margin-v2-unit](HISTORY-20261114-through-relative-margin-v2-unit.md)，SHA256 `c772dce4a855e181e700130c5805801f3be5e3451968af6f35ad38cef5264406`；其历史pending/PID不能用于重派。更早原件由该文链接继续保留。E=`/Volumes/disk0/Khor/evidence/net`；worktree均在`/Volumes/disk0/Khor/worktrees/current/`。

## 当前唯一Turing执行及下一步

唯一pending **turing-20261114T032931Z-531d7086**，net-node / contact_current_mixed_block_3_candidate_v1，role candidate；control **2cc1ad6578c83a4384335e825f2ba30db650439b**，protocol **13b66d93d03519417f0b20c956cbf697ad968f8044d502995c16727d81bc4f51**。先核actual receipt/execution，禁止重派。原始baseline descriptor E/contact-current-mixed-baseline-source-v1.json（c3b098bd/treef6c239ae/snapshote22263d6，dirty=true是既有已闭bridge快照，不能新造为clean）。

**030058Z-e97c48c8普通Release producer已闭成功**，0/570.415930秒，Cargo原8m30、22文件/identity/原owner已核；准确已过Linux286/Mac282的 **96e999da/tree41df16ad/snapshot1cc2cb53**（I4＋N2联系代次修复）。E/该run/contact-current-release-compile-verification-v1.json；CLI **443c6e010293b83e7c0e963b6eb96edb3897dd4c0d6ada0bca9f6fca02666a43**，full netlab **01b6432ae381f633d98a4a1fb6af84b9a5e91fa1075f65c88f371ec14f53dfbb**。features[]/原Release thin LTO、无qlog/诊断feature，CLI强制refresh且实际fresh=false、46原来源合同源码一致复用，未执行任何业务测试。原H2/PULL4未采用；I4浏览器锁保留，N2 link/wires是相对I4仅有产品增量。

producer完整tree13路径逆重建精确H2 tree5cf730ab，source_frame全部13逐项核验，其中native5（carry恢复I4、link/wires、2个单元fixture），netlab11文件不变。raw migration的fixtures_unchanged在该控制模板中指netlab；新增2个node单元fixture在plan/proof显式列出，不能称所有测试文件都未变。E/contact-current-release-{source,compile-plan,compile-control}-v1.json，control950e7607及全部原source/provenance保留。首次本地旧descriptor断言与随后不存在baseline descriptor的stage前失败均未创建远端/native执行；后者E/contact-current-mixed-stage-preparation-v1.json，不算性能场次或替代试次。

新验收的设计/实际绑定已在全部计时前冻结：**E/contact-current-mixed-design-v1.json**（SHA57d5827b8348b70a4cc9f81510d3c478a4cfb8030783fc80ada54ded79ac1afd），**contact-current-mixed-plan-v1.json / control-v1.json**，E/ops/freeze-contact-current-mixed-v1.py。冻结原public-main runtime baseline035824的已闭ELF，candidate030058准确普通Release；ABBAAB六块×low/high=12case/8640，原caller2秒、16/192MiB、bulk120/240秒及owned600保持。每arm5个原guards/calibration新执行，coldTCP原红按既有MODEL-DECISION分层，不能写全绿。计时无编译，无日志开关；两份producer初始selected5 archive按原hash保留，只有已闭新candidate可创建初始档。

原相对10%+20ms/文件90%及原始基线各cell固定绝对界限同时保留（high目录bulk592518.1μs/TTY391538.2μs、文件中位16.791371Mbit/s下限）。新协议保留有效baseline产品失败并继续固定cohort，其失败cell相对全成功门不能通过；candidate任一完整块硬失败拒绝。固定360任一cell已有>18条越固定p95门，或三文件已有两条低于固定中位门，可数学不可挽回早拒；未知调度/来源/分析/清理则停，无替代。H2旧协议不追溯修改。

baseline预检 **031149Z-be538813** 已按collect/identity/行为/record闭证，1/164.474783秒、17文件：原5项中2owner/lowTCP/UDP四绿，原high coldTCP红，分层模型准入、全部owner退出且分析完整；progress已1块/collecting，所有原red保留。candidate预检031537已唯一派发，前驱031149。

candidate预检 **031537Z-b18b6b98** 已collect/identity/行为/record，1/164.635421秒，原同5项四绿＋high coldTCP原红，分层准入且全部owner退出，17文件；progress现2块/collecting。**business1 baseline031906** 已唯一派发，原low/high两profile计时，前驱031537。

business1 baseline **031906Z-28dfcb1b** 已闭0/279.047954秒、14文件，low/high各720ok且2文件完整。low目录bulk计划→完成p95 273795μs/TTY83153μs/文件6.9338Mbit/s；high605119/758473μs/15.3005Mbit/s。当前原始baseline本身较早参考波动，CPU准入实际99.17%idle、PSI0；不变更原门或以此为candidate开绿灯。原限速queue downstream drop9742/delay drop0，E/high-mixed-rate-queue-observation-v1.json连同以前10场计数只作线索，0随机loss不等于0taildrop，不能推算未经分母/GSO核对的丢包率。progress3块/collecting。business2 candidate032427已唯一派，前驱031906。

business2 candidate **032427Z-7237c5a7** 已collect/identity/行为/record闭，0/239.000680秒、14文件，low/high各720ok及2完整file。low目录bulk计划→完成p95 234969μs/TTY82376μs/文件7.2083Mbit/s；high628558/517555μs/24.2642Mbit/s。首次candidate两p95高于固定绝对门，但累计不可挽回门尚未触发，progress4块/collecting；原y_journal含Relay→Direct两次selected事件，选后snapshot质量见原log，非精确选择输入/业务route。business3 candidate032931已唯一派，前驱032427。

下一步：032931完成后collect-network-ssh.py、verify-turing-record.py、**E/ops/verify-contact-current-mixed-block-v1.py**、**record-contact-current-mixed-block-v1.py**按顺序闭证记录。progress E/contact-current-mixed-progress-v1.json，是新cohort唯一顺序入口。若phase仍collecting且无未知，按progress唯一派business4 baseline，source E/contact-current-mixed-baseline-source-v1.json，revision contact_current_mixed_block_4_baseline_v1，同control，前驱032931 receipt；再依次business5 baseline/6 candidate，revision contact_current_mixed_block_{index}_{arm}_v1，每块完整low/high；每次collect/verify/record才下一派。**summarize-contact-current-mixed-v1.py**只在完整cohort给相对＋固定绝对门，停止则不冒充完整对照。root唯一ledger writer，无后台协调器，不和编译并发。

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
