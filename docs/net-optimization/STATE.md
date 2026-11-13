# feat/net 当前状态与接续入口

更新于 2026-11-13T09:34:46.572056+00:00。完整目标仍是[GOAL](GOAL.md)、[PLAN](PLAN.md)的N0–N5，**未完成**；逐项缺口见[REQUIREMENTS](REQUIREMENTS.md)。root唯一writer，不派子智能体。

此前完整STATE逐字存入[历史](HISTORY-20261113-through-wake-controls-and-mac-preparation.md)，SHA256 `d4e996331af2a19b62ee65dc69af0e68e170ef8408009520ff97e89151c60cf2`。历史中“在途/无pending”只描述原时点；不得重启已闭批次。本页是当前入口。

## 当前两台执行器

1. **Turing W原控制v2：073647Z-7e574645已终态0 / 4752.282312s**，control **c2c2d706799a26585febf3d135d43ca039a7e327**。固定8顺序p1s×6→p3→p5，全部原门通过、替代0。29files/source/control/hash/identity/owned cleanup核验完、ledger已闭，证据E/turing-20261113T073647Z-7e574645/copy-wake-original-controls-verification-v2.json。六轮600s短尖峰控制、黑洞接管、十二次短停顿控制均完整执行。**仅这8个控制通过，不是N3接受**；下一步是原unlogged Release业务性能对照，尚未派发。
2. **Mac补丁38移除的library检查43396826114已success、收集并核验**，request75ad0d5579ed4949a2960844a4ba5e4d，control **f27a6c9777d0f270fa986842a3e819f4b3fb3a65** / builder-ref feat/net-tuple-exit-mac-lib-v3。tree2d702718ff8f664cc1e2fa5db2352e2d8e91ee8d、actual snapshot0f11a5f7ebf2dc42a662a4d1d79697443ca42aee。移除前43394584580与移除后均 **198pass、0fail、原ignored1、0filter**，199项名称及结果完全相同。artifact14023303488（228172bytes/SHA29bf14c4…）ZIP/CRC、源/控制/metadata/guard均核，证据E/public-mac-tuple-exit-disabled-v3/mac-library-disabled-verification-v3.json。**库对照绿，不等于补丁可安全退出**，真实NAT重绑定仍待。

W原控制、Mac库两臂及W普通Release producer均已闭。Turing目前在途 **093228Z-62e9e4e5 / PID3968961 / child3969212**，control **8452189a017dbb400e6e77cec56c4b56c99dcb5e**，原公开基线的5项fresh模型/观察器准入，尚未业务计时。exec43311仅提交已终态0，不重派。准确receipt为E/turing-20261113T093228Z-62e9e4e5/receipt.json；cohort进度E/copy-wake-release-comparison-progress-v1.json，root手工串行、无独立coordinator。新对照协议见[COPY-WAKE-RELEASE](COPY-WAKE-RELEASE.md)，三臂固定B C W / C W B / W B C（18业务场景），已于业务前冻结。当前没有本批性能结论。先收集核验并关闭当前准入，再依次parent/wake准入；全部满足分层准入才进入固定业务顺序。

## 现场、授权与准确源码

E=/Volumes/disk0/Khor/evidence/net；工作树均在/Volumes/disk0/Khor/worktrees/current/。本机source-only，仅Git、stdlib Python源/数据/ZIP/hash与SSH/GH编排，不运行本机Rust/Cargo/rustfmt/Node/Bun/Xcode或产品native。重型只Turing/macserver/可信GH。用户main、已安装App及其他任务Lab23961不动，无main合并、发布/标签/TestFlight或生产Relay采购部署授权。用户已授权公开colehank/khor-build，不按旧私库额度阻塞。

- canonical khor-net / feat/net：runtime仍5064fe2a、iroh1.2/Noq1.3；本页文档提交不代表已合入候选runtime。
- I khor-net-product-integration-v1 / integrate/net-product-v1，clean **117c39e5432741198c924177fae1775ff292eada**、treeace4fd8223ed7fcd8d2b29477d9e651a2df53ff2；iroh1.4.1/Noq1.4.0，parent为冻结当时的公开main c3b098bdbf52a9b3b38e887ae74b90155292b544，apps/.github与该冻结main一致，非自动跟随新main。**因原N3 Release high退化拒绝最终晋升**。
- R khor-net-live-middle-retry-v1 / experiment/net-live-middle-retry-v1，clean **9081b94dce827cbd3fb9b9e7bb13633d762e3cd9**、tree **cf9d172bb92778ccfd97899ddea27bdf0e6d2f6e**。该tree准确等于被测snapshot a3cad264bc6772d68289b5f954c0966dc87627cd。仅hop.rs有runtime差异，另含三原physical fixture与两unit文件；接受窄恢复场景，未集成canonical/I。
- W khor-net-copy-wake-v1 / experiment/net-copy-wake-v1，HEAD117c39、四文件staged，frozen tree **f5faf2a2fef7fd64f09908fc53c73b24bb2eae4d** / snapshot **8780bf503d753b820a7236931357b2dcebfa42a6**。C共同fixture treeadbc7d6a5032cdc24db19cdbac4b9d1f7ec6415c上仅remote_state.rs runtime不同：取消off_direct guard，沿用已有wake/backoff/receive证据、质量/滞回/Noq/期限/信任。**消融，不是已接受默认设计；不含R**。
- M38 khor-net-tuple-exit-v1 / experiment/net-tuple-exit-v1，HEAD9081b94d、一文件staged，tree **2d702718ff8f664cc1e2fa5db2352e2d8e91ee8d**，预准备snapshotc5754ef1f3705c3ccfab3180a111bf058a0cca42；仅删patch38 reconcile helper及调用（53行方法，55行删除/1行增加），保持常规select_path_on(true)。无fmt/native/退出接受推断。Mac用新container0f11a5f7但相同tree。
- T khor-net-sole-cache-v1仍是原T3测试源，已闭tree8ff93012943b0b6c6b598142559b74e682e58829；I/R的旧对照、B/C/Q/A等准确树/闭合raw证据见历史与对应专题，不按摘要覆盖其staged内容。
- khor-build-net HEAD保持5d3691f252f9dee0297fa3c494374be6119bca29；仅新增Git对象/refs/control分支，未移动HEAD或默认main。Mac新增控制分支见上；公共main9b671817只读取维护脚本（与原worker-storage内容相同）。

## R：原单个存活peer relay恢复已红→绿

详[LIVE-MIDDLE-RETRY](LIVE-MIDDLE-RETRY.md)、[SOLE-PEER-CACHE](SOLE-PEER-CACHE.md)。T3/sp7 064222Z-57c213de原1/114.475143s：三前置全真，M在0.34–0.56秒读到Y，X仍5秒超时且无新hop；不是旧sp5/6前置失败。原红完整保留。

R prep065900、compile070105Z-8e82e611已闭（0/281.033342s、17files），whole hop92/92/0ignored；含真实NO_ROUTE→有限需求恢复、finite Pump仍退避、25ms caller到期、32并发只领一名额。原native verifier20unit/同producer CLI与test核。runtime **070832Z-6b87e77c 0/98.267049s**、18files/source/control/hash/identity/owner/ledger闭，原sp7固定3恢复3/3，总 **1013894/784788/878995us**；原同一5秒/真Direct首腿+Relay第二腿/marker/hint held/same PID/drop counters保持。CLI856ecd02bf638c2b08908a625e5579aac732d4369866df36eca5a6da0d101121，test72a8b6536c60eefd4adede0aa48aa549b07b93bcde6d64ecd64ec8d4b8b5afd3。

有限Ask/Forced且无target road、实际开放首腿，才在原总期限内重查；3秒最短间隔复用原HOP_WITHIN，后台/比较及无活首腿保持指数退避。TRIES2、current epoch/owner/策略/信任/单跳/写不重放保持。此3秒尚非能耗最优定案；全部NET16/手机/多middle仍未齐。

## W：控制进度与原资源中断

library063044Z-bf939c7b完整iroh **198/0/原ignored1**、15files/source/hash/metadata实际vendored Noq-proto核，非性能接受。native compile **071457Z-c140a37b 0/116.466771s**、17files/原verifier20unit、同producer netlab与CLI核。CLIbc47a17b2d3a424690478c42d8d556f66e7e21eee7ea02f1fdf48d24e07ff375，test8116cf8e939cf3a795266b2c6bfccb1e5209267a474794b8c3a0b5da0f72809f。

原v1 **071957Z-261fdeee 1/594.071116s**、15files：首p1s在supervisor524.866815s触历史owner513>512，中断前14业务均ok，无600s verdict/summary；原native-15/forced、原group_absent=false保持。控制器终态后只读postmortem PGID3664371及8个记录/proc项全空。原中断已闭，不作产品红/绿；另7未执行。

v2仍原600s/p5每stall4drops/Rust全部门，supervisor1200s只容纳长场景、outer90min不改。历史记录cap8192、同时匹配owner仍512；纯数据校准2000历史+1活通过、513活拒绝、8193历史拒绝。前六轮实际peak_live10/11、history667–697，全部原门通过；p3实际230次黑洞后读、最慢536833us、0失败；p5十二次短停顿、未切Relay/未打洞。六轮p1s实际切换0/0/0/0/0/1、无退役。计划E/copy-wake-original-controls-plan-v2.json。旧健康spike 3/6与原Release红保留，不覆盖旧结果。

本批额外逐项审计372次完成的目录读均ok（p1s各20，p3含预热240，p5共12）。原helper echo()实际执行ls_of(beta,/tmp)，旧result里的keystroke措辞不能当PTY实测；p1s/p5原门只约束稳定性，额外读审计属于事后范围核查，不新造SLO或PTY验收。原N3真实PTY+文件的unlogged Release检查仍必需。

## Mac库对照与构建通道

协议E/tuple-refresh-exit-mac-library-plan-v3.json；源码R与M38各完整iroh库一次，test_threads1、同原ignored、0filter，Noq算法未改故不重复其whole suite；Linux真正NAT退出仍待。

- v1 **43390512217**在GitHub source bundle大JSON读入时JSONDecodeError，编译/库执行0；artifact14020995265（1800bytes/SHA01b2eb7f…），ZIP/原日志已留E/public-mac-tuple-exit-baseline-v1。
- v2 **43392622400**真实Mac raw bundle已验证，但既有guard拒绝<50GiB，编译/库执行0；artifact14020718395（2776bytes/SHA69c7652a…），已核E/public-mac-tuple-exit-baseline-v2。
- 修复使用GitHub支持的raw blob media type、bounded reads、原64MiB cap、Git blob SHA1+bundle SHA256不变。实际22110825bytes源包校验与六个纯数据完整/截断/超限/错误hash控制均过。Mac baseline后续复用同d3a6379快照/54b072d blob，不重复上传或编译假装新源。
- v3 **43394584580 success**，真实Mac **198pass/0fail/原ignored1/0filter**，metadata iroh1.4.1/Noq+proto1.4.0及实际vendored路径、源码treecf9、controlf27a6c9、ZIP/artifact14022571285（229653bytes/SHAcea062709…）均核，E/public-mac-tuple-exit-baseline-v3/mac-library-baseline-verification-v3.json。
- 使用仓库已有prune-test-programs，只在free<50时于build.lock下清无占用旧测试exe，留每目标最新2、排除lib/链接，lsof活文件保留。原实际guard **50GiB准入/40GiB停止/target100GiB**（旧35/20文档对此机已过时），前后SHA998f5723…完全相同；清23个旧测试、约2GB，库编译8.80s/执行45.80s。没有改guard或产品/App/store。
- disabled v3 **43396826114 success**，完整iroh库198/0/原ignored1/0filter，编译7.73s、运行45.58s；与baseline的199项名称及结果逐项一致。metadata实际iroh/noq-proto来自vendor，noq1.4.0来自registry（不是误称三者均vendor）；原guard SHA998f5723…保持。artifact14023303488与全部16files内容已核，source tree2d7/controlf27准确。只接受库对照，NAT oracle未执行。
- 失效/新源的receipt目录已列；baseline v2/v3均**复用ceeb30968b274bb0b8da0f5a41ebf520原refs**，不是新请求所有：全部使用结束留证后只能由原owner、核SHA再退休。disabled请求75ad…有自己refs；本批均已闭，但此处尚未删除远端refs。

SSH macserver本轮仍8秒banner超时，但GH通道实证可用，不当整体阻塞。M38退出边界见[TUPLE-REFRESH-EXIT](TUPLE-REFRESH-EXIT.md)，计划E/tuple-refresh-exit-plan-v1.json：原r0/r1 fixture SHAac977269…不改，原124读和30暖远端stream选中新tuple/零mismatch的oracle scripts/netlab-nat-selection.py不放宽；拟当前源ABBAAB各3真NAT，先完善实际producer/control协议。**库绿不能判补丁可删。**

## 原N3退化与仍需完成的完整范围

[WORKLOAD-RELEASE](WORKLOAD-RELEASE.md)：公开冻结main c3b098对I117c39，相同Release共同fixture、A B B A A B、low40ms/8Mbit及high300ms/50Mbit、12完整file/8640记录。baseline low/high各3/3，candidate low3/3、high1/3（1925ok/3timeout/232blocked），吞吐19.3299→22.4087Mbit不抵销交互失败，原所有红保留。双方BBRv3/PULL_AHEAD8/priority0。分层模型准入原cold TCP红也保留，未造TCP绿。

[WORKLOAD-QLOG](WORKLOAD-QLOG.md)/[WORKLOAD-NATIVE-ACK](WORKLOAD-NATIVE-ACK.md)各固定3（各2160ok）未复现原超时，不是修复/普通性能接受；全已闭，不加同源诊断挑绿。qlog观察到一段Relay回复两个传输观察点相隔1.308s；ACK病例有单Direct copy2PTO/另一copy较新收包且move仍Hold，各自限其source/观察，不把所有尾延迟合成单根因。

后续必须保持：N1速度优先与稳定/共享样本/成本；N2总期限、取消/资源、读写/0RTT不重放；NET16多middle/缺真字段/全部策略与移动；N3原unlogged同源Release前后对照、同步/预览/SSH/hop公平与分连接比较、1/3/5设备CPU/线程/字节/唤醒/手机能耗；N4双方及middle IPv4/NAT/IPv6隐私IID/前缀独立和同时变化、Wi-Fi↔蜂窝/锁屏、寻址/Relay登记与故障域/容量布局；N5全部custom退出依据、最终同一准确source完整Quality/Mac/手机接受。原长稳p1h包+33.970%/字节+37.908%及旧所有红保留，不能宣称闲时最优。

原I Quality43348869616首次13/13、Mac Lab43355810513 attempt2（原WebKit失败保留/唯一失败job重跑77case绿）及ZIP/source/probe证据仍只适用I；只证明两懂Versions版本的旧/新互通011900Z-19b49119。物理iPhone仍旧0.2，尚无候选手机/能耗接受。移动只读审计见[MOBILE-ADDRESS-AUDIT](MOBILE-ADDRESS-AUDIT.md)，准确netwatch0.19.3有iOS AF_ROUTE通知，静态App去重不单独证明漏事件；连续route chatter的250ms debounce只列待测风险，不冒充根因。

上游最后registry核2026-11-13T05:25UTC仍iroh1.4.1/Noq+proto1.4.0（最终交付前再核）；custom ledger60项为1上游替代有证、2移除红保留、57退出仍待，不是57均必要。patch38当前上游对照只读E/iroh-patch38-source-exit-review-v1.json不能替代新源物理退出。最终设计/职责/持续回归/报告与准确最终代码仍需收敛，无main合并或发布授权。

## Release准备中断与继续（本轮）

原准备 **091727Z-71804040 1/51.961675s** 已收集13files/identity/owner干净/ledger闭。原B/C/ACK各5个selected ELF+dep-info已逐字保存，C canonical khor由同一个原selected proof中相同hash的deps ELF取回，不以文件名推定。准备在原native_provenance模拟单元37项处停：32过、2fail、3error；**Rust编译与业务执行均0**，不是产品红/绿。E/turing-file-clock-calibration-v1.json与原断言表明Linux文件ctime较time.time_ns滞后一个tick。v2仅对模拟unit的prepare返回前增加最长1秒的同文件系统ctime barrier，原dict/时间戳/断言/校验器/产品timer不改；并按失败migration凭据回到准确旧源核验再继续W。证据E/091727目录的copy-wake-release-preparation-failure-v1.json（目录完整名以run ID为准）。原失败保留，不通过删单元或放宽门来编译。

W普通Release **092115Z-acbcbe95 0/596.523376s** 已收集20files/identity/source/control/hash/owner核验并ledger闭，准确source仍treef5faf2a2、snapshot8780bf50。原37项模拟unit在有界ctime barrier下37/37，原校验器/断言不改；真实cargo编译9m04s。默认features，无qlog/net-diagnostics；opt3、strip、thinLTO、codegen1保持。test SHA80f7252a42fe4067d61f01775c3982fd2f0d1f63100621703e85756ac8176622、同producer实际CLI SHA5a46c7750880e46e946039ca1af12fcd5a0cb30a5b9eac81e1e878db1fd7d61f。E/该run/copy-wake-release-compile-verification-v1.json，runtime执行0，仅producer接受。新runtime控制保留原内核adoption witness，准确source与全部selected文件还原后仍用原native_provenance，计时前2秒CPU实际取样门已冻结。
