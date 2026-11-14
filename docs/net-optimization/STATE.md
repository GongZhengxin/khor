# feat/net 当前状态与接续入口

更新于 2026-11-14T01:37:55.820890+00:00。用户已授权持续实施/远端验证，活动目标“好 完成完整目标”仍未完成。root串行，不派子智能体。完整[GOAL](GOAL.md)/[PLAN](PLAN.md) N0–N5范围保持，逐项缺口以[REQUIREMENTS](REQUIREMENTS.md)为准。未获main合并、正式发布、现装App替换或生产Relay采购部署授权。

此前STATE逐字归档[HISTORY-through-relative-margin-v2-unit](HISTORY-20261114-through-relative-margin-v2-unit.md)，SHA256 `c772dce4a855e181e700130c5805801f3be5e3451968af6f35ad38cef5264406`；其历史pending/PID不能用于重派。更早原件由该文链接继续保留。E=`/Volumes/disk0/Khor/evidence/net`；worktree均在`/Volumes/disk0/Khor/worktrees/current/`。

## 当前唯一Turing执行及下一步

唯一pending是 **turing-20261114T013456Z-1969d559**，net-node / relative_margin_preflight_candidate_v1，role=candidate；实际receipt/source-refs优先。control **429e1e87a686f8ea78fcde0cb666914203d50982**，protocol **2a72b1c8889a320fb50bafeb1434dfe6a1eaf6945774849ff710f7505a6d8a8a**。本轮只测相对正常切换收益门，未接受参数。

准确候选H2：worktree khor-net-relative-margin-v1，branch experiment/net-relative-margin-v1，HEAD **db21c2beeecb1d6b1823589267b2f98d34fa5e38** / tree **5cf730ab964c52fd613953251000ba2057e0f2bc** / snapshot **67b205f2d7e55eafb02d81b3a5be1f672669f948**。E/relative-path-margin-source-v2.json、source-audit-v2.json；相对DB7bac只carry.rs，choose_settling在已有活路时叠正常10%/既有settling20%收益门。初始/故障无活路选择、绝对5/10ms、60秒dwell、improves及peer-relay清理保留原版。PULL8，无新增timer/probe/连接/vendor。两新增政策测试和全部旧断言保持，反向源码重建逐字DB。

H2原生net-unit **011751Z-b0350bd0 0/37.973077秒**已闭，72pass/0fail/原1ignored，原全部名称/断言与H1相同；E/run/relative-path-margin-unit-verification-v2.json。普通Release producer **011935Z-60f3d677 0/554.262451秒**已闭，Rust8m23、actual CLI fresh=false，strict provenance/原profile/features[]/无qlog/无net-diagnostics及selected5存档核验完整。CLI **d11188633795731dd3189c787db3f3d99f83568419bbcf822a841e3118139cb4**，fulltest **b94a18c51041e9ba02152607139cc4fcfe2945d17dab62c9fa00c50c642b167a**。E/run/relative-path-margin-native-compile-verification-v2.json，producer control481c6c219a328da23e8148736be0ee0e2cb1c31e。cache从拒绝的PULL4恢复8并带carry.rs，迁移2文件；相对DB产品差仍只有carry.rs。原46来源工具元验证沿已核source hash准入，不复制Cargo fingerprint。

实际runtime已冻E/relative-margin-comparison-plan-v1.json / runtime-control-v1.json。两臂均完整同netlab entry及11共同workload/observer/calibration文件，原8/普通绝对margin为baseline（DB214246 producer），H2为candidate。两臂各5项新preflight，然后固定 **A/B/B/A/A/B** 六个high场景，720结果/场、共4320/6个192MiB文件；原2秒caller、240秒文件、400ms采样、owned600秒与所有model/CPU/磁盘/来源/清理门不改。已预先声明：任何business硬功能失败，当前场完整核验后停止后续；candidate失败拒绝候选，baseline失败停止参考准入；未跑项明确unexecuted，不作完整六场pooled结论。无替代/后台协调器/运行编译。

baseline preflight **013143Z-078f5b6e 1/166.500305秒**已闭，17files/identity/owned完整；两ownership guard、lowTCP、独立UDP过，原high冷TCP吞吐红保留，既有分层准入成立。E/relative-margin-comparison-progress-v1.json现1/8块。候选校准013456尚待核验，业务0。

继续顺序：先查source-refs和013456实际execution；收集后运行ops/verify-turing-record.py、verify-relative-margin-block-v1.py、record-relative-margin-block-v1.py。只有原分层准入/分析/清理成立且无pending，才用stage-launch-turing.py依序启动relative_margin_block_{1..6}_{baseline|candidate}_v1，source分别dual-replug-source-v2.json和relative-path-margin-source-v2.json。每块单独收集/核验/record，根智能体唯一ledger writer。summary用summarize-relative-margin-v1.py，统计自检已过；未完成cohort不得声称通过。

原main参考工程门仍为high bulk目录p95<=592518.1μs、PTY<=391538.2μs，完整文件median>=16.791371Mbit/s；与当前baseline相对10%+20ms/90%门分别报告，均非用户已批准的普适SLO。完整低/高、多负载、公平及成本仍待。

## 已闭修复与仍然失败的方案

**N2旧拨号超时覆盖新认证联系已窄修复，并过完整Linux标准回归。** 准确96e999dae8312d86a38c08095ba58c3e1c1274e1/tree41df16ad51dc3955a8a6fdf3a494432d9f1a3dc0/snapshot1cc2cb5312430ffc4ba55c10ecd7f07ef1dd7a10，worktree khor-net-dial-contact-repair-v1。产品d15c2d1d仅link.rs/wires.rs，96e仅相同取消安全fixture。全局contact守Asleep，每协议握手代次守普通/early真实失败和等待者缓存；slots→presence→down同步提交，实际失败history/counters保留，普通流量不推进握手代次。三个失败出口均guard，无新timer/packets/tasks，不改原期限、PULL8或写/0RTT边界。

同fixture原版5066bfb9固定14：002222Z-fb33f5b6 101/196.613641秒，12pass/原2red；修复003059Z-d829e18f 0/196.561372秒，14/14，8marker、其他peer/无联系/新Tunnel与原6Asleep负控制全保，handler全join/cap-hit0。E/run/dial-contact-concurrent-{original,repair}-verification-v2.json。完整标准 **010353Z-7e1681fd 0/472.042894秒**，**21files/identity/owned**闭，原12组 **286pass/0fail/原1ignored**，包括IPC/取消/响应资源组；E/run/dial-contact-repair-standard-node-verification-v1.json。原T0两产品红和首修复串行fixture取消panic、格式准备失败全部保留，见[DIAL-TIMEOUT-CONTACT](DIAL-TIMEOUT-CONTACT.md)。仅这些准确源合同接受，不代全部N2或最终集成。

**H1全局收益门已拒绝。** f4b495e7/tree54404310，011408Z-8eea3f7a 101/46.143293秒：71pass/1fail/原1ignored，原path::tests::the_relay_beats_the_hop_only_on_measurement发现200ms Relay会清195ms hop。E/run/relative-path-margin-unit-rejection-v1.json；未改原断言，Release0，准备control539fbf6a未执行。E/relative-path-margin-design-v2.json先冻后才做H2，恢复清理资格、只限定正常迁移；H1所有原件保留。

**PULL4已拒绝，六场计划未完成。** 原8首场004921为720ok；4臂005307为676ok/2timeout/42blocked，005746为667ok/2timeout/51blocked；三文件完整、来源/原生/CPU/清理核验闭。三场goodput23.925/13.988/10.925Mbit/s。两个4臂硬失败后root等在途完整自然结束，取消尚未发起的4/5/6（8/8/4）。该早停是数据后的程序变更，原v3未预先声明，已在E/pull-window4-rejection-decision-v1.json明确；不作完整六场因果/pooled性能接受。原plan/control/全部坏样本保留，progress-v3=stopped-candidate-rejected。详[PULL-WINDOW-EXPERIMENT](PULL-WINDOW-EXPERIMENT.md)。PULL4不带入最终候选。

**原main↔F3完整同期N3对照是RED。** 六ABBAAB块/双profile共8640/8640读、12文件完整；high bulk目录p95 520471→963174μs、PTY337762→695394μs，均过原回退界限，吞吐18.657→24.413Mbit/s不抵销交互回退。E/current-mixed-comparison-verification-v1.json，详[CURRENT-MIXED-COMPARISON](CURRENT-MIXED-COMPARISON.md)。

路径线索E/high-mixed-path-churn-observation-v1.json逐核9场：原main三场selected-change均0，当前8三场2/4/2，另8首场0，4两场6/26。实际path-stream事件不是决定输入/业务包路由；缺统一wall↔workload时基，相关性不作因果。H2是待证假设，不提前称根因或最优参数。

## Mac 独立验证及磁盘入口

Linux已过的96e同源Mac标准net-node已唯一提交 **43540500760**（request7e5a0e2484914cb99b7f1c46d886642a，snapshotb516bfd2b818b81506d8a3a388803709a6315ceb，source tree41df16ad，builder feat/net68cadb57）。该builder的标准12命令块逐字Linux5d control，SHA25625cc8686fb1e5877bc0c80a2cd8b92de7c8a30eb130cb8ce83dbdfc8a259b90a。network-mac/job132279693750在原run-build门 **free<50GiB** 被拒，编译/测试0；不是产品红。原log E/dial-contact-repair-mac-attempt1.log，dev-evidence14091886017 SHA f178febcc3818a9f0c39568319fe59a275d6f0c008df1c808419db301724390c（2741B，尚未本地取ZIP）。不原样重派，先恢复磁盘reserve。

只读盘点 **43540770433** main9b671817成功，artifact14092315786 12912B SHA01a8f196bca3464555b9507cb9ccaedd30bb2830085a8fafaa6f1ac654518424/CRC已核，E/worker-storage-inventory-43540770433.{zip,json}。free49.66156GiB、target83.55098GiB；incremental37.38995GiB/347目录，82旧变体符合现有8版本/2小时规则；另1个superseded khor_node test约254.6MB。当前installed build_guard SHA **998f5723eb447a1634411397c18a524e5ed0d50bd6dbd81001a16626219f64eb**，50/40GiB及target100GiB门保持。

发现维护逻辑不一致：main scripts/worker-storage.py的ensure-budget在target<90GiB且free>=40GiB就提前返回，但启动需50GiB。当前尚未改/执行清理。下一步以原共享build.lock、lsof活跃保护、每crate最新8份及2小时保留规则回收符合条件的旧缓存；不降磁盘门、不碰用户数据/活跃Lab或原生下载缓存。可先查本轮macserver只读SSH重试OSsession82935（仅hostname，20秒限），若不通继续可信GH通道。已有worker-storage.yml仅inventory/prune-test-programs可选，ensure-budget需受控入口；先完成可核验修正/原断言，再运行和取实际free/guard/hash报告。恢复后才能重派同源Mac Node。此处没有App构建/安装/发布。

## 仍有效的I4/F3与边界

I4准确4517d4cb/treecd52c290（khor-net-product-integration-v4）完整Quality43523128168 FIRSTATTEMPT13/13及可信native Mac Lab43523194833首轮4jobs/原77case全部成功，真backend/source/contract/arm64 host/ZIP已核。E/net-product-quality-v4-verification-v1.json、gui-verification-v1.json（实际27组/附件9、Chromium159.0.8154.6）、public-native-lab-net-product-v4/lab-acquisition-verification-v2.json。playwright-core1.65.1/WebKit27.2只在原四场固定诊断后更新，未改原5/15秒断言；I3旧WebKit26.5两完整Lab红和四场2/4崩溃均保留。详[WEBKIT-LAB-DIAGNOSTIC](WEBKIT-LAB-DIAGNOSTIC.md)。I4不包含之后N2/H2修复，不是最终源或手机接受。

F3 silent/healthy原固定6场168/168读成功、silent0.873–0.954秒、healthy原TCP全部保持；双端V4/NAT及dual/前缀固定6场252/252、首双向0.507–1.674秒已闭。完整来源和原失败在归档及[RELAY-DEMAND](RELAY-DEMAND.md)、[BOTH-ENDPOINT-MOBILITY](BOTH-ENDPOINT-MOBILITY.md)，不代IPv6-only/真机或全N4。

canonical feat/net产品运行仍5064fe2a（iroh1.2/Noq1.3）；本分支后续HEAD主要持久文档，尚未把已升级iroh1.4.1/Noq1.4.0的实验/集成树合到canonical。最终只带实证通过修改，保留I4浏览器锁、N2修复与main既有成果。custom历史60项为1已上游替代、2Noq移除红保留、1 iroh38实际NAT移除红保留、56待退出核查，不能说56均必要。

仍缺：N1完整多middle/置信/成本；N2全部取消/预算/写入及包级0RTT/NET16负向；N3共享vs分连接、多负载公平/1-3-5成本及原回退；N4独立/同时IPv6-only、发布hint/新连接、手机换网/睡眠/容量故障域及拓扑方案；N5逐custom退出、最终准确源完整检查、真实iPhone/能耗及最终设计报告。全部详细原要求留REQUIREMENTS，不据局部绿缩减。iPhone仍旧安装版，早前“就绪”已过期，真实候选可用后需新确认。

## 执行约束

本机source-only，不运行本地Cargo/Rustfmt/Node/Bun/Xcode/产品；Git、stdlib Python源码/数据/ZIP/hash与远端编排允许。重型仅Turing/macserver/可信GH，其他任务Lab23961和main不动。Turing UID1110/shared flock、jobs16/nice10、Rust1.95、磁盘50/40GiB、私有user/netns、计时前CPU>=80%idle/pressure<=5/无同UIDCargo；编译与计时串行。Mac共享worker jobs2，原50/40GiB、target100GiB不降。root唯一ledger writer，无后台协调器。

先读本STATE与PLAN，查实际worktree/HEAD/diff及E/source-refs.json；不要按历史pending/PID重派。准确候选图谱无覆盖（0不是低风险结论），已先核main图谱再定向读实际源；不自行建立索引。当前已有单元通过不代Release业务、Mac或真机；不停止完整目标，不标goal完成。
