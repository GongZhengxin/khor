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
