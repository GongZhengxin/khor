# 网络验证运行账本

2026-11-09。证据根 `/Volumes/disk0/Khor/evidence/net/`；每个目录的 source.json / environment.json 才是准确源码和控制器回执。失败按产品、准备、格式或执行器分类。历史源码通过不覆盖后续修改。

| Run | 任务 / 源码 | 结果与适用边界 | 证据子目录 |
| --- | --- | --- | --- |
| 42505117705 | 旧策略 + 速度回归，net-unit | 产品红，60 pass / 1 fail / 1 ignored；90ms Direct 胜过5ms Carried | baseline-unit-42505117705 |
| 42505672250 | 首版候选 net-unit，tree b66d0343 | 61 pass / 1 ignored；不含后续所有 bind 注册 | candidate-unit-42505672250 |
| 42506109052 | 首版最终 net-unit，tree d0fdb004 | 62 pass / 1 ignored，含统一 bind 与 settling | final-unit-42506109052 |
| 42505323213 | 初次 baseline smoke | p6r 产品红；p6d 反向 wire 未建立，是准备红；p2/p3 绿 | baseline-smoke-42505323213 |
| 42506330051 | 修正 fixture baseline smoke，tree e73903a3 | p6d p95 34.02ms绿；p6r p95 369.78ms、Direct311/309ms产品红；p2 3.1s、p3最坏846.1ms绿 | baseline-smoke-42506330051 |
| 42506345592 | 同 fixture candidate smoke，tree ee158fdb | 4 pass；p6d p95 64.96ms；p6r p95 152.12ms、Relay84/88ms；p2 1.2s、p3最坏546.6ms | candidate-smoke-42506345592 |
| 42506805985 | hop 看守新增回归，net-unit | 产品红：relay30ms/hop22ms，固定10ms看守错误释放有价值hop | hop-unit-red-42506805985 |
| 42507205671 | hop 候选 net-unit | 仅格式失败，patch已应用 | hop-unit-42507205671 |
| 42507239186 | 同轮 net-node | 仅格式失败，patch已应用 | hop-node-format-42507239186 |
| 42507643289 | hop/p7候选 net-unit | 仅 p7 assert 格式失败，patch已应用 | hop-unit-42507643289 |
| 42507629732 | 同轮 net-node | 仅p7 assert格式失败，patch已应用 | hop-node-format-42507629732 |
| 42507391687 | p7动态，tree 9ffe76e4 | 产品红：交互与闲置两场景均45s未回Direct；0请求错误；尚未含新采样修复 | dynamic-red-42507391687 |
| 42508095178 | net-node，tree 067d62b5 | SUCCESS，hop9 / wires10全部通过；不含随后vendor采样改动 | hop-node-42508095178 |
| 42508069745 | net-vendor，tree 067d62b5 | 执行器准备失败：excluded依赖不能从根workspace跑dev-dependencies；控制器已改为独立manifest + committed lock | vendor-unit-42508069745 |
| 42507224815 | 完整旧策略baseline，tree e73903a3 | FAILURE：23 pass / 8 fail；6个测量断言红、2个配对阶段busy，另p2i报告red但未assert | baseline-full-42507224815 |

新采样候选：同tree `064925806d1c3962f715942d7f637a4b6257783f`（含全部新增文档，无排除untracked），base bc6cded2，control `16474c5`：

- net-unit **42509122554**，snapshot `3c88d2f67ff691431427cbd3153b76d67d1abf66`。
- netlab-dynamic **42509155432**，snapshot `41fdf5a93ef5fc717145abd87c6b218adce2b130`。
- net-vendor **42509174050**，snapshot `301ecbca4134be116d877ceac658f70545bc92c1`。
- unit SUCCESS：63 pass / 1 ignored，证据fresh-unit-42509122554。dynamic SUCCESS：p7a 2.6355秒、p7i 18.1609秒、零请求错误，证据fresh-dynamic-42509155432。vendor SUCCESS：noq431/iroh路径21通过，证据fresh-vendor-42509174050。source-only脚本轻量测试16项通过。

p6网络配置中的 relay40ms 是单腿配置；完整 A→relay→B 往返约80ms，实际84/88ms。已修正新日志和协议标签，未改变拓扑或200ms目标。前述A/B仅一次，四场景并行、不同托管VM；只能作为首轮证据，不能声称统计显著或替代重复/固定好路径参照。


固定好路径参照/重复与完整候选（control419b5f0）：

- baseline compare **42509612220**，tree `f95f6e54876971b5e8555896196b85fa457a3bca`，snapshot `e1088b13e0391c85998c92fa5338bb533cc58aab`。fixture/脚本与候选一致；生产代码6485cb83。
- candidate compare **42509632238**，tree `4549630fc7df95885de570640c764ca1d60e2735`，snapshot `776925e4e8ecaf7caf561dd38a4a7843c80a8c98`。
- candidate full **42509652259**，同tree4549630f，snapshot `0c2e962f57ed106c4944be399f02f288f1334825`。
- **42509569373 CANCELLED**：提交后发现baseline漏同步NETLAB_TEST_THREADS开关，已主动取消；准备输入错误，不纳入A/B，正式任务42509612220已补齐。


三次顺序动态 **42509803932**：tree `9075794453893d6bb17a54303cf358fc01c8d8e1`（clean30335781），snapshot `3d5b3955c61cd83e99f23a06424d9b661aca0278`，control `9d915b7`；SUCCESS，三轮全部零错误：p7a 2.5261 / 2.7413 / 2.7497秒；p7i 19.6758 / 19.6386 / 19.7252秒。证据fresh-dynamic-repeat-42509803932。fixture新建Lab/居民，每轮同一VM，不能把重复当作独立机器统计；闲置已贴近20秒门槛，需相位/负载覆盖。


完整旧策略baseline42507224815：31项、8并行、1908.33秒，23 pass / 8 fail。

- 进入测量后断言红6项：p6r（p95 429.97ms，仍Direct306/320ms）、p1s（alpha3次/最多1次）、p5（150ms阶段beta打洞2次、600ms阶段alpha1次）、f4s（未回carried）、Plain relay NAT d2/d4未直连。
- a2与f4i未进入相应测量：配对 `document transfer state busy` 导致unwrap；这是准备阶段观察，不证明其NAT/路径行为失败。busy归因尚未确定，不能无证据称执行器故障或已修。
- p2i报告red（34.3秒）而Rust test ok，当前原场景仅观测；完整pass计数不表示它达标。
- p1/p1j/p4均600秒，p1移动alpha0/beta1，p1j0/0；relay b3是1200秒。b1/b2/b3/b4、c、uplink e与其他转发通过。
- 此完整matrix包含显式p1s CPU压力和其余并行场景，性能数字不与单线程repeat混用；它也没有后加p7及固定参照。候选完整42509652259尚在途，暂不归因回归。


quality输入一致性预期红 **42510628203**：tree `fa664f056b152393d7c5e8b42737aaffcf8fa86b`，snapshot `c2c4fbe831fab5a6eaae24816c2768879c667362`，control9d915b7。新增PathQuality数据与2项行为测试，helper仍读旧rtt；待结果。随后新增vendor缓存/API尚未包含在本run，也未验证。


正式3次对照已结束并全部下载：baseline42509612220每轮5pass/1fail，仅p6r旧策略红；candidate42509632238每轮5/1、4/2、5/1，p6全绿但p2两红（13.0/16.4秒）、p3两红（9.9/15.3秒、后者1超时）。见REPORT，不能把candidate工作流失败都归作预期旧策略红。

共享quality输入：42510628203已取得产品红63pass/2fail/1ignored，证据quality-input-red-42510628203。修复tree `fdc41531d7785fb22ce45b83ff0ef537ef52099b`：net-unit **42511021381 SUCCESS 66pass/1ignored**（snapshot97786ea0，shared-quality-unit-42511021381）；net-vendor **42511084014 SUCCESS noq431/iroh22**（snapshotad1f0fb5，shared-quality-vendor-42511084014），control9d915b7。包含PathMeasurement缓存/同副本age/count、PathFact接线和亚毫秒质量区分。

后续API短矩阵 **42511301780**：tree `f8eba3f5cb00780bc3b95d159fe5f421374ce409`，snapshot `083053a77e25ea76db83ef59218149bbccc09c29`，controld7d1af1；加入f4/f4s/f4i、2并行。它仍在ACK-delay校正前，待结果。

最新ACK-delay候选同tree `05a5c7575d72b524a081a28b48b62afc9c4824b1`、basecbcc8abe、controld7d1af1：vendor **42511866017**，snapshot `e3c172fa2112d40b87a9a343cee69e4e541bca2e`；短矩阵 **42511895143**，snapshot `6e5479ace12e63a8671c23a3fb2dd4ef8c2aac1d`。尚待验证。该候选保存与平滑器一致的校正样本，未改变raw latest/PTO/拥塞数学；不能凭代码推断已修p2/p3。


42511866017已SUCCESS：noq432、iroh22；证据ack-sample-vendor-42511866017。42511301780已FAILURE，8pass/1fail，证据shared-quality-smoke-42511301780：p2 2.6秒、p3 worst536.9ms（0错/退役）、f4/f4s与4项p6全绿；f4i first started正确，但恢复后2次selected relay、回hop58.6秒，违反无回退断言。

同速黑洞在飞数据预期红 **42512359677**：tree `d35655265579a8477072a5831c7135c0bd9f8da2`，basecba1ea80，snapshot `edd23810981b29bc57e292fbb40e03df19835cdd`，controld7d1af1。仅新增真实虚拟PTO/永久移除旧路的多路径流交付回归，未修改重发逻辑；待结果。


42512359677已产品红：432pass/1fail，same-speed blackhole test在旧路数据立即重排断言失败；证据blackhole-backup-red-42512359677。42513039282（tree1ad7b266、snapshot7c71583a）仅编译失败，因path_stats引用/借用；证据blackhole-backup-candidate-42513039282，已结构性修正。

重排修复候选：**42513546687 net-vendor SUCCESS**，tree `6465d10558e05566d7aa621fcc911e432ee8019f`，snapshot `516e70a650e05ed2b40a085d42bec7a3d774a448`，basecba1ea80；control以environment为准。真实网络动态 **42513580645** 同tree6465d105，snapshot `e67ddf7b3a6d72b2545d1a17be014c0d8d6ffea9`，controlc6a20c5，p2/p3/p7单线程各3轮，在途。

42511895143已FAILURE 8pass/1fail，仅f4i前置未达，证据ack-sample-smoke-42511895143。ACK校正p2 973ms，p3最坏696ms、零错/退役，f4/f4s与p6全绿；仅一次，不能替代正式3次。

42509652259完整旧采样candidate已FAILURE：35项21pass/14fail，1968.35秒；证据candidate-full-42509652259。此源早于共享API/ACK校正/失败Backup重排。保留原p1/p1j/p1s/f4/f4s/f3/p2/p5等回归，不把旧源问题静默删掉。


42513546687已下载blackhole-backup-green-42513546687，**433Noq/22iroh全绿**，environment证实control `c6a20c5a733a0778985c3960bc6490820da60d41`。同速黑洞回归现在不仅重排计量通过，流数据在健康路交付也通过。

带f4失败诊断的短矩阵 **42513843436**：tree `6e61d014c82c25b24f2bf4cfaab7a4e5f92043b3`，snapshot `66dac4d7271a6cf336492d141253b8be299a2a32`，basecba1ea80，controlc6a20c5。在途。生产代码与tree6465d105相同，仅追加f4失败时最后50个路由journal到artifact；不改变拓扑/门槛/工作负载。


当前源接线/第一方格式 **42514304126 net-unit**：tree `cb25e12597100ca70f27a1c0cdfd0d06a94c5347`，snapshot `bc56f16f9f7b481ff80387d2acb3a3ca038737ce`，basecba1ea80，controlc6a20c5。在途。生产代码与433/22绿源相同，包含f4诊断与最新文档；若格式失败，保留formatter patch并应用后再验证。


42514304126仅格式失败：新f4诊断的matches宏需多行，未执行单元，不算产品红。证据backup-source-unit-42514304126/format.patch已检查并应用，无本机formatter调用；待新源复验。


格式修正41c8af78后的 **42514729034 net-unit**：clean tree `7ba783704556f3893cfb1bd1dc9fb6d1fc3d4b74`，snapshot `955b3e90dbefc3a7f8b4b8ade950a1ca793b0ea1`，controlc6a20c5，在途。42513843436短矩阵已FAILURE，待下载/分类诊断，不把观察超时当终止。

## 最新终态与p2协议纠错

以上“在途/待结果”为当时记录；以下是终态。所有证据已下载，原失败不删除。

- **42514729034 SUCCESS 66pass/1ignored**；证据backup-source-unit-42514729034。只覆盖当时源，未覆盖后加p2 v2与f4_stall诊断。
- **42513580645 FAILURE**：三轮分别3pass/1fail、3pass/1fail、4pass。p3最坏530.5/732.0/740.9ms，均零错误/退役；p7 active2.4233/2.6044/2.4252秒、idle19.5757/19.5113/18.7489秒。旧p2前两轮AlphaRelay/BetaLAN后约390/410次成功回显仍未bothLAN，第三轮1.4秒bothLAN；原门未测每次回显分布，不能回填体验通过。证据backup-dynamic-old-protocol-42513580645。
- **42513843436 FAILURE 7pass/2fail**：f4绿（腿应答后0.2秒回carried，无回退）；f4i本次绿（22.4秒回m，无回退）；f4s红（23.0秒carried、应答后14.0秒、最长Relay17.8秒、slowest echo10.1秒）。p2另一红为LAN-only类型门；p3最坏563.7ms零错/退役，p6四项绿（p95 64.59/148.30/87.07/115.82ms）。证据backup-hop-diagnostic-42513843436；没有path-diagnostic行，因为本轮f4i通过而当时f4s尚未挂失败诊断。后来fixture补到f4_stall，不改拓扑/门槛。

p2纠错协议在86ce3330/680dee60、任何v2实跑前冻结：原Relay未加延迟，必须LAN与速度优先冲突；改验明确快集合LAN/Relay并加逐次真实回显门，10秒/零错误/零退役保留。Link旧平滑RTT无年龄，不拿它冒充新鲜值设门。旧LAN-only数据全部保留，p3真实卡顿继续有效。

相同v2 fixture三轮顺序对照（controlc6a20c5）：

| Run | 生产代码 / source tree | snapshot | 状态 |
| --- | --- | --- | --- |
| 42516040900 | baseline6485cb83 / 02851ae57ad3df1c0a4e67db171f8148152fd82e | 6fc450c403b4d4022172a19f5ff0ed695f63781c | FAILURE，三轮4/2、4/2、3/3；证据speed-v2-baseline-42516040900 |
| 42516056972 | candidate680dee60 / e0f8ac95368e27248abe9db96296f949f1d85cbd | 1c86ed462d76d423c63e640e9ac1913de48f213d | SUCCESS，三轮6/0、18项全绿；证据speed-v2-candidate-42516056972 |

两版paths.rs相同；baseline生产代码保持648不动。候选包含新增f4失败输出但compare不运行f4。候选p2 1.3/2.9/2.9秒、n35/41/41、p95 38.6–47.4ms/max62.4–90.1ms；p3最坏431.1/539.7/463.8ms，全部零错/退役。旧p2 3.1/3.1/26.6秒，末轮超限；p6r三轮慢Direct红，p95 330.4–333.0ms，新版113.4–128.9ms三绿。旧固定Relay参照p95 110.9–113.9ms但三轮期末held路径断言红，不能称整项绿；详细各项见REPORT。

源码持久化：43个实验快照已建本地不可变`refs/khor/net-evidence/<run>`，清单证据根source-refs.json。每个ref的`:source`子树校验匹配source_tree，避免七天artifact和未引用git对象过期后丢失脏树实验源。未推外部、未覆盖任何工作树；新run终态后仍须补环境/模式/结果证据。

## 诊断与真实样本时刻

- **42517477095** net-unit，tree50bb9361/snapshot55ffa0e6/controlc6a20c5，仅p2 fixture格式失败，未执行单元；patch已应用。证据speed-v2-unit-42517477095。
- **42517864599** net-node，clean221b98fc/tree87f4555d/snapshot50775a6e/control781a907，仅新journal兼容测试字符串格式失败，未执行单元；patch已应用。证据path-measurement-node-42517864599。
- **42518102550** net-vendor，treed62cbf30/snapshota9a761f4/control781a907，同一journal格式失败，未执行Noq/iroh单元，不能记预期新鲜性产品红；证据ack-time-format-42518102550。

221b98fc在既有path事件附缓存质量/age/count，不新增timer、不改UI显示句。control781a907将journal单元加入net-node，新增netlab-hop以原fixture单场景顺序三轮。

| Run | 任务 / source tree | snapshot | 状态 |
| --- | --- | --- | --- |
| 42517915206 | netlab-hop / 87f4555d149816be3fbc74801aac98a7357c74de | fd10c974e5e2c032f5cef62627675e747b1bb26b | SUCCESS，f4/f4s/f4i三轮9/9；hop-isolated-42517915206 |
| 42517959505 | full netlab / 87f4555d149816be3fbc74801aac98a7357c74de | 6c42bd86d9193953f9fde0821346e343e55067b6 | FAILURE，23pass/12fail，35项1864.48秒8并行；ack-backup-full-42517959505 |
| 42518379039 | net-vendor / 915fec4f91a79525b6d2fac33ee728048004be93 | d24a4cc9f51c7cb580b170bf2e2171c2801fbb80 | 产品红，Noq434绿、iroh22pass/3fail；ack-time-red-42518379039 |

后两类源不同：前两run生产代码仍为ACK校正+Backup修复、Reading用actor观测时刻。42518379039只向Noq暴露采样包发送/ACK处理时刻，保持原RTT/PTO/拥塞数学，新增Noq元数据一致性与iroh三项回归（无ACK初始估计变化、延迟读stats、积压旧包不能确认新往返）；尚未修改Reading消费逻辑。全部control781a907。恢复先取真实红，不能把格式错误或静态判断当作回归已经复现。

真实时间窗修复 **42519038936 SUCCESS**：tree `5b137c71dd44ea439111d41b724acc7fb4871d00`，snapshot `997e58aca525718f24617443b2fb109425c5760e`，control781a907，434Noq/26iroh全绿；证据ack-time-green-42519038936。保留3项旧源真实红，第四项是后加静默覆盖。实现4fa3f1b3，p7相位先7cfa922d冻结、后6a45e488补5个case，共7相位；不改10/20秒目标。

同clean6a45e488/tree `2c18140bb83f52ca7c6ff0508980977667b1dbfc`，control0e5788c（给net-node补核心khor-net单元）：

| Run | 任务 / snapshot | 结果 |
| --- | --- | --- |
| 42519750741 | net-node / 641d2e83ad9250e2a634a5bcca51c64b07ad0bbe | 格式/核心66pass1ignored/hop9/wires10绿，journal27pass1fail；新诊断Entry兼容产品红，ack-time-node-42519750741 |
| 42519778236 | netlab-dynamic / 64a72fa8fd51a021c2517c0b889e277bd64d174a | 在途，p2/p3+7个p7相位，单场景顺序三轮27项 |
| 42519803507 | netlab-smoke / 053f6d67243e454ee874cc1e35a71b8bda74c364 | SUCCESS 9/9，2场景并行380.40秒；ack-time-smoke-42519803507 |

最新短矩阵：p2快路1.0秒、p95 33.7ms/max45.7ms、零错/退役；p3最坏609ms零错/退役；p6d18.68/p6r106.05/ref-d29.22/ref-r106.27ms p95全绿；f4腿应答后0.9秒/f4s1.0秒回carried且无回退，f4i9.3秒回m。仅一次，不能替代本源码三轮或长稳定性。f4/f4s的3/8秒断腿期间没有快路，最长回显3.7/8.8秒是记录值，不套p3快备用目标。

42519750741失败原因：新What::Path嵌PathFact含u128，serde内部tag缓冲不支持直接deserialize_u128；已取真实产品红，不算格式失败。Netlab::LabNode::journal直接读原始JSON Value，所以既有实验路径日志仍完整，正式journal Entry回读另外需要修。6b3644f5保留算术类型，读取JSON的u64毫秒数字再扩展，并验证实际持久化Entry及显示句；不改网络选择。

首次该修正 **42520842668**：tree `a7b1a498994cb2cef7544f44ebf75f73b987ba8f`，snapshot `775b8e61ca09161bc24460b868ee55f56625c71e`，control0e5788c；仅新assert格式失败，未执行单元。证据journal-number-node-42520842668；patch与当前源反向校验通过，已按其格式落盘。

35项full42517959505终态分类：

- 准备未达4项：f4/f4s/f4i在180秒内未经m，p1j beta没到Direct，未进入相应测量。
- 原断言红8项：f3没声明no_relay；Plain NAT d2/d4未选Direct（尚不能推断直连路径未打开）；p1移动5/3、p1s7/9；p2快集合13.5秒、max424ms、零错误/退役；p5短卡顿切换/打洞；固定Direct参照p95 1.5628秒、max2.6313秒。
- p2i报告red15.7秒而Rust未assert，不能计达标。p3最坏656ms零错/退役、p6自动两向及Relay固定参照、p7两项均绿。
- 18行f4准备失败诊断仅x/y的Relay/拨号/settled，无hop/Carried记录，缺M腿和道路书事实，不足以定因。6b3644f5把三个f4失败出口都扩到x/y/m六方向；完整8并行压力不能与独立9/9转发混为一条性能结论。

当前clean6b3644f5/tree `75ca1f10d591b5cc29f0189775847461475a0a4c`，control0e5788c：

| Run | 任务 / snapshot | 状态 |
| --- | --- | --- |
| 42521515341 | net-node / 853f56ed862f13915c14b6f924ac7f14f8e1e95d | 在途；格式与核心/hop/wires/journal，含Entry数字修正 |
| 42521539154 | full netlab / fc1903c89d37f711a93eb52877f3f14bd58ab7be | 在途；40项8并行，真实时间窗+新增相位+六方向失败诊断 |

恢复时先取三项活动任务（另有42519778236），不因轮询超时重启。各run实际source/environment和原日志为准；不能把这次full35项或前一版短矩阵当作未来源码完成证明。


## 2026-11-09 窗口过期修复与独立稳定性

以上三项等待记录已结束：42519778236为26pass/1fail，p2末轮16.2秒超10秒，所有21个p7相位绿（交互2.26–2.60秒、闲置4.39–18.56秒），p3最坏703.7/573.3/458.6ms，零错/退役；证据ack-time-phases-42519778236。42521515341仅hop_diagnostics签名格式失败，未执行单元（journal-number-node-42521515341），其formatter patch已应用。42521539154为28pass/12fail，40项8并行2001秒（ack-time-full-42521539154）。

full40红项：f3没声明no_relay；f4在180秒内未通过m；f4s腿应答后0.4秒回carried、但30.5秒后又Relay且期末Relay；d2/d4未选Direct；p1移动5/11、p1j准备未Direct、p1s移动5/5；p3最坏1.1秒且零错/退役；p5切换/打洞断言；p6r p95 223.21ms；p7i20.0543秒。其余6个p7相位绿，p2本次绿。p2i报告red79.8秒但Rust未assert，不能计达标。完整交叉负载不能全归CPU，亦不能拿它代替独立稳定性。

控制均为fdd1b3f80de655e8430c9ee00362da1109f06fd2：

| Run | 任务 / source tree | snapshot | 结果 / 证据目录 |
| --- | --- | --- | --- |
| 42524575725 | net-node / c4c0324c69ca3d55c3d031d1b9cfb2f8ddc552da | 2a7cb33abfb4b4a016ff217d600b7f4514a5e811 | SUCCESS，核心66pass/1ignored、hop9、wires10、journal28，journal-number-node-42524575725 |
| 42524600966 | netlab-recovery / 同c4c0324c | cdabfe166a39f07958b22cbe71755f8c5c9d3b42 | SUCCESS 6/6，recovery-diagnostic-42524600966 |
| 42524625685 | netlab-stability / 同c4c0324c | 093b405cb04e0483ea818c6cfa4172b928e20515 | FAILURE 2pass/2fail、1880.44秒，stability-isolated-42524625685 |
| 42525350514 | net-vendor / 7af2b013aef23546977fc07e545b63827b0b4e16 | 851e0be25cc924c3677b312013ac270143c47b61 | 预期产品红：Noq434绿、iroh26pass/1fail，quality-expiry-red-42525350514 |
| 42526508917 | net-vendor / f182946f57f13110a16433e1a1bdaf093e3aef2c | d709594aa36daf3313f5d65bf0e2f3cf4e70de0d | SUCCESS Noq434/iroh27，quality-expiry-green-42526508917 |
| 42527295249 | netlab-dynamic / 6f9a77e399e25e1c3b57eb8d52136fcd04e3d92a | dbcf6d949d8de4be03ce247d1b2bc29e3c59de2c | 在途，过期修复后，p2/p3+七个p7相位3轮27项 |
| 42527316922 | netlab-smoke / 同6f9a77e3 | e600c180ba602727678ed55dd1bd554a341fe599 | 在途，过期修复后9项2场景并行 |

独立恢复p2收敛750.2/727.6/753.1ms，max32.65/31.53/31.88ms；p3最坏640.6/705.8/578.4ms，零错/退役。先前16.2秒失败保留，不以不同VM三绿宣布消除偶发。

独立稳定性单场景顺序、每项一次，保留p1s自身忙CPU与20次300ms停顿：p1空闲600秒两端移动0/0、零退役；p5在150/300/600ms各4次停顿，均零Relay切换/零打洞，最慢回显266.9/469.3/973.7ms。p1j准备失败（alpha未Direct，日志见Relay约23–25ms、Direct约21–22ms；尚未进入600秒测量），不能把它记为600秒抖动红。p1s确实测量红：600秒移动14/2、零退役、最慢回显658.8ms。说明抖动不能全部用full交叉负载解释。部分selected事件快照质量已接近/反转，不能当作底层决策瞬间输入；需确定性复现确认流程，先不盲调门槛。

窗口过期红：近2秒窗口只剩1ms样本，派生quality却仍300ms。4ec6ab2e在窗口缩小时复用record的中位数计算，空窗保留最后已知值；样本时间、ACK计数、确认记录不刷新，不新增timer/探测。红绿源仅后者修正派生值，实际网络因果仍待新两项结果。实际Entry持久化回读已由42524575725验证，兼容修复不再等待旧格式任务。

实验源现已锚定57个本地不可变refs，清单source-refs.json；已校验每个snapshot:source与source_tree一致。活动源已锚定但仍须补终态environment/mode/结果。


42527316922终态 **SUCCESS 9/9**，373.32秒、2场景并行；证据quality-expiry-smoke-42527316922，准确source/environment见该目录。p2 1.0秒、n34/post30、p95 36.70/max40.55ms，零错/退役；p3最坏642.6ms，零错/退役；f4/f4s腿应答后0.3/0.4秒回carried且无回退，f4i19.6秒回m；p6d23.57/p6r123.02/ref-d24.11/ref-r106.38ms p95。早于下一项确认候选，不替代防抖验证。

新增确认回归取红 **42528150054 net-vendor**：base645363cd、treeb82736a93788817c0f8a2d8397ae4d5c6d352ebc、snapshotd760ea482545bf86350ebd555722dbe14c532ada、controlfdd1b3f；在途。仅新增两条行为回归，生产逻辑仍为4ec：决定前两个包不能触发快速迁移，等待快候选时应同时探测当前路。拟最小候选与恢复边界见NEXT-DESIGN，尚未修改生产逻辑。


42528150054终态预期产品红（decision-proof-red-42528150054）：Noq434全绿，iroh27pass/2fail，恰为新两条回归；一条旧值Go，另一条旧Hold不探测from。不是编译/格式红。候选只改确认取证起点与等待时双路复测，快分支仍不强制等慢当前路，不改黑洞leave/收益/固定确认时长。

确认候选source tree `86619b975d12358a8ee99aefee4407857efdbe6b`，base645363cd dirty，controlfdd1b3f：

| Run | 任务 | snapshot | 状态 |
| --- | --- | --- | --- |
| 42528714010 | net-vendor | b82b415a8bb99fcc905cc1799141c7d7f9de0677 | 在途，434Noq/29iroh目标 |
| 42528746575 | netlab-stability | bf2c40d5cd2c5fff34e30bc81038156300cc2cc5 | 在途，单场景长稳4项，原准备/忙CPU保留 |
| 42528783380 | netlab-hop | 97263e13d067b978bb73b8be3aa2b7921151140e | 在途，转发3场景×3轮 |
| 42528841695 | netlab-dynamic | 05d5a15898b11633f6356cd60d2761ac6f813137 | 在途，p2/p3+7相位×3轮 |

全部source已锚定；清单现62个refs，不依赖snapshot远端分支或artifact保留。确认期间可能比旧快分支多PING，受原4秒确认/30秒冷却边界约束，尚无后台开销结论。


42528714010终态 **SUCCESS 434Noq/29iroh**，证据decision-proof-green-42528714010。新增两回归先红后绿；原快路不等慢路/原黑洞离开测试仍绿。源码已提交6eabe635；提交前核当前vendor/Noq/策略文件与绿snapshot:source无diff。新确认可能多一个取证阶段，实际p1s/f4/p7目标与探测开销仍须网络证据；余三项新源任务和42527295249旧确认/过期修复相位任务均未结束。


## 过期修复后的正式三轮：黑洞仍有真实长卡顿

42527295249已 **FAILURE 25pass/2fail**，三轮9/0、8/1、8/1（362.79/371.43/367.23秒），证据quality-expiry-phases-42527295249，tree6f9a77e399e25e1c3b57eb8d52136fcd04e3d92a、snapshotdbcf6d949d8de4be03ce247d1b2bc29e3c59de2c、controlfdd1b3f。单场景顺序、三轮27项。

- p2收敛1.5/4.0/1.1秒，max58.15/62.52/50.23ms，零错/退役；全部通过。
- p7共21项全绿，交互2.26–3.10秒、闲置2.51–18.41秒；闲置phase0为18.356/18.408/18.358秒，仍有20秒裕量问题。
- p3最坏612.4ms/1.3秒/15.3秒。末轮还有10.3秒请求，1次15秒超时，零退役；alpha Relay@1s、beta Second@1s。既已选备用仍卡顿，不能只用路径类型收敛证明请求恢复，不能当执行器问题删除。

在此源之后的6eabe635只修快速确认，三个实际网络任务仍在途，不能借其未出结果覆盖旧失败。当前新增 **42529470019 net-vendor**：basee45cac03、tree65bdf14629fee79159010ae273c7582179c53d58、snapshot50542c58de1cdbb5f4e9d8758c090f8280071895、controlfdd1b3f。仅复用原真实虚拟计时黑洞fixture增加“先改status、后达到不应答”的回归，保留原先故障后移路及健康同速覆盖；未改生产Noq。预期检验只在status改变时检查是否漏掉后续PTO达到的不应答，尚未取得结果，不预认定为这次netlab的全部原因。


42529470019终态预期产品红（late-backup-red-42529470019）：**434pass/1fail**，仅新增先移路用例失败在“a failed backup must release data even if the move came first”。没有编译/格式错误；Noq失败后脚本未运行iroh，不能记iroh新红或新绿。

候选保持原判据，只在Backup既有LossDetection事件重新置位检查，由poll_transmit统一重排；不引入独立timer/逐次发送扫描。回归额外检查<300ms不重排，并在相同虚拟时序验证<1秒释放及健康路完整流交付。p3仅在失败后补两端路径事件，不加测量请求。

以下同tree `6cbec16a56889023db95e92f6a4dc657aa6a6805`、basee45cac03 dirty、controlfdd1b3f：

| Run | 任务 | snapshot | 状态 |
| --- | --- | --- | --- |
| 42529931673 | net-vendor | fb8718047f9584b6dcd3f239cea7ce1aef5bca02 | 在途，目标435/29 |
| 42530014989 | netlab-recovery | 544da02c21aff9aaad98a94bcdc3b0fdbbb72e2a | 在途，p2/p3三轮6项 |
| 42530060891 | netlab-smoke | 39c8968ba8ccc7e49c6df6e5e2d31fdedf0b53ca | 在途，9项2场景并行 |

6eabe635三项旧Noq网络任务仍独立在途，不把它们当作这份新Noq代码验证；它们用于分离确认修正对稳定性/转发/相位的影响。


42529931673终态 **SUCCESS 435Noq/29iroh**，证据late-backup-green-42529931673；Noq新增后移路故障用例先红后绿，另外<300ms不重排、<1秒原PTO释放及完整流交付均通过，既有434与iroh29全绿。代码已提交ff399658，提交前核全部vendor/策略/p3文件与绿snapshot:source一致。当前66份实验源ref已验证，终态收据已刷新，余5份网络任务仍在途；不能用单元代替实际p3/f4。


## 2026-11-09 确认版终态与准备v2

下列前3项同tree86619b975d12358a8ee99aefee4407857efdbe6b、controlfdd1b3f，早于ff399658：

- **42528841695 SUCCESS 27/27**，decision-proof-phases-42528841695。p2 976.7/752.8ms/3.0秒，max32.29/32.11/31.55ms；p3 461.2/613.8/493.3ms，零错/退役。21个p7全绿，交互2.38–2.79秒、闲置5.62–19.67秒；phase0为19.6105/19.5826/19.6735秒，裕量小。每轮359.64/355.58/359.18秒。
- **42528746575 FAILURE 1pass/3fail**，1311.33秒，decision-proof-stability-42528746575。p1空闲600秒0/0；p1j、p1s均Direct准备失败，未测各自600秒。p5零Relay切换，150ms组打洞0/1、300ms组2/3触发原短停顿红；600ms观察组打洞2/2、最慢回显1.1秒。不能称p1s防抖已经改善。
- **42528783380 FAILURE 8/9**，decision-proof-hop-42528783380。三轮3/0、3/0、2/1，554.53/485.29/514.06秒。f4应答后1.0/0.6/2.2秒，末轮超1.5秒；均无回退。f4s 0.5/1.0/0.8秒全绿，f4i22.7/29.8/18.8秒全绿。

以下ff399658源码tree6cbec16a/controlfdd：

- **42530014989 SUCCESS 6/6**，late-backup-recovery-42530014989。p2 2.9/3.1/1.3秒，max36.43/34.95/36.66ms；p3 591.9/428.7/602.2ms，零错/退役；每轮172.20/167.52/165.26秒。
- **42530060891 FAILURE 8/9**，305.58秒2并行，late-backup-smoke-42530060891。f4i准备阶段LabNode.with的Node::open_as报document transfer state busy，并未进入网络恢复测量。p2/p3、p6四项、f4/f4s绿；p3 594.5ms零错/退役，p6d35.16/p6r112.88/ref-d25.24/ref-r110.97ms p95。f4应答后0.2秒；f4s selected记录早于最后一腿heard，饱和差值0而非证明真实恢复耗时为零，均无回退。独立状态门测试明确Busy是实际非阻塞保护；当前不改生产存储，也不删除失败。

稳定性准备v2先309c1bbc冻结、后f8292b59实现。设备Relay100ms往返只用于one_speed准备，清除后再原30秒settle/600秒测量；p1s忙CPU/20次300ms、原最多一次移动/零退役均保留。只复制paths.rs/hop.rs/dev.py/netlab.sh至baseline；已逐文件相同，并核其全部diff只在这四文件，baseline生产仍648。

| Run | 任务 / tree | snapshot | 状态 |
| --- | --- | --- | --- |
| 42530871452 | net-unit / e4912fda2dcb2caf8aed2324495d48c8e1cff424 | b372912e515416278e81fe4b97e9a250b6e6d3e5 | SUCCESS 66pass/1ignored，stability-bootstrap-unit-42530871452 |
| 42531361022 | candidate netlab-stability / 同e4912fda | 591b73995f7e88fd6f629bedde75e99a046a1b46 | 在途，ff生产+准备v2 |
| 42531408799 | baseline netlab-stability / 0e9bf325d47377ac4c7b09b8d8ef68cd53b2e62c | 1b3f7a1bbadee364f746f3c9e8df449509e6f87d | 在途，648生产+相同fixture |
| 42531477192 | netlab-hop / 同e4912fda | a432378e2dbd779482bdffad75e5851a7c51b002 | 在途，新Noq版三轮9项 |
| 42531529275 | net-vendor / 80b4f2a6f891a9f9d96874ed206007cb8149523f | a17654f76db3e8eb80bdb616d31a8918d15a47c9 | 在途，Watch单次慢质量不能立即打洞的回归取红，生产未改 |

均controlfdd1b3f。Watch当前在首次慢quality直接打洞，未等既有2秒探测窗；新回归仅提出行为要求，未将静态分析记为复现成功。下一候选须保留真实故障1秒通道/30秒节流、要求决定后新往返，避免仅延长固定等待而让旧样本/CPU暂停通过。


42531529275终态预期产品红：Noq435绿，iroh29pass/1fail，单次150ms质量同时probe/punch为true；证据probe-before-punch-red-42531529275。

Watch候选 **42532303734 SUCCESS 435Noq/33iroh**：basef8292b59 dirty、tree63ddfeff21b634bbb79e0cf104f07a6117ca8b6e、snapshot5da5fad0b37e8a93d3c34f26e7f0e64ace4ab0ab、controlfdd，证据probe-before-punch-green-42532303734。代码519ad4aa；提交前核vendor/策略/fixture与绿快照一致。普通质量持续原2秒+两个开始后包的独立观测才打洞；原探测预算不变，前台样本仍可确认。极慢RTT超2秒不等放不进预算的往返；原故障1秒/打洞30秒节流保留。3个附加覆盖为新绿，不能声称都已旧源取红。

control **6af0ac328457a8c3aea9157e8b2ffcd65ae7e5c2**已push feat/net，新增netlab-stalls（原p1s/p5完整时长各1次/1线程），不替代完整A/B/full。产品9项、control准备1项Python及network.sh bash-n通过。当前3个Watch网络run同tree `5f40be2e96dae66a8d57cb82afe1e447014c194b`、base519ad4aa dirty（后续接口/文档变化），control6af：

| Run | 任务 | snapshot | 状态 |
| --- | --- | --- | --- |
| 42532770351 | netlab-stalls | b23eef27de00b6214e09e82416f764f108044a6f | 在途；实际prepare成功、Linux network运行、Mac跳过 |
| 42532849927 | netlab-dynamic | dc80527796731ec472955f5c593b5069f2e82d59 | 在途，27项三轮 |
| 42532877270 | netlab-hop | 43c7f6ee25e5dd488c1d3fdbb262dfbb06a0c711 | 在途，9项三轮 |

源码ref清单75份已验证，所有终态收据已刷新；其余3个旧Watch（新Noq）网络任务继续独立保留，不互相替代源码结论。
