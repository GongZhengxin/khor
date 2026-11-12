# feat/net 当前状态与接续入口

更新于 2026-11-12 18:05 UTC（本地2026-11-13）。用户“定好目标，开始”已恢复执行；完整目标 [GOAL](GOAL.md) 与 [PLAN](PLAN.md) N0–N5 **未完成**。
用户ToDesk级体验、iroh/Rust极致优化及原约束均已记入GOAL/PLAN v3。root串行，不派发子智能体。目标服务旧paused状态不覆盖本次明确恢复授权。

此前STATE逐字保存在 [上轮完整状态](HISTORY-20261113-iroh-recovery-and-stability-preparation.md)，其中继续链接所有更早历史、原失败、收据及冻结输入。
历史SHA256 `3e1fe2e38c7f7d90fa34166977390a8e2918564a7ab2370d01cf5c906f970753`。本页只描述当前事实和下一步；历史中“当前在途”按其时点理解，不能再次启动已关闭批次。

## 恢复后的当前执行

暂停现场已核：coordinator61277不存在，原candidate用户取消已完成收件/identity/group空，ledger无pending，Turing原独占锁可取。保留baseline完整5/0、28journal、22CPU与32目录请求原证据；按E/iroh14-policy-stability-resume-plan-v1.json（SHA a67d29b161db39ea368f55b916d2ca2c0f1251ef55c5aa917de9810705bb1a9e）只补一轮完整candidate。源码/控制/五原case/600秒/原门/资源均不改，原用户取消部分保留；不重跑baseline，不追加失败替代。随后执行mixed，再建立Release性能基线。当前在准备该补齐run，准确run以ledger revision iroh14_policy_stability_candidate_resume_v1恢复。

## 历史暂停（2026-11-13 00:57，本地）

以下描述历史暂停；本次18:05 UTC明确恢复已覆盖停止授权。coordinator61277已按准确命令身份停止；candidate长稳 turing-20261112T164807Z-a0e7a1d2 由其owned runner SIGTERM正常清理，原返回1/KeyboardInterrupt，原files收件、identity及group空已核验，ledger终态user_cancelled_partial_run，**不是产品回归或长稳接受**。baseline完整5/0与28journal/22CPU/32目录请求原证据保留；candidate部分观察不构成完整配对结果，未启动混版。当前没有本任务自动推进；恢复必须先核该取消收据并显式修订中断后的协议，不能盲目重启已消耗的fixed arm。

## 暂停前执行时点：两组长稳对照

- run **turing-20261112T155449Z-56a8ac18**，PID **2128254**，15:55:10 UTC启动。
- task `netlab-stability`，revision `iroh14_policy_stability_baseline_v1`。
- control **f862ab2defb48d21236c2bdcca736c9a3d108773**，baseline source **515e99f0d886de8035801801a10e7587167cf78b**。
- receipt及E/source-refs.json已存，启动exec已终态。最近只读仍running，p1已原600秒绿（两端0move/0retired/0punch、四journal完整），p1h/p1j/p1s也已各原600秒绿，p5在途（4/5原结果，未最终收件）；p1s含20次300ms spike，模拟交互最长4.7s。原门只验切换/退役，不能据green接受交互尾部，该4.7s须与候选同场景对照并继续定位；不要重复提交。
- 先baseline完整终态、收件、identity和行为核验，再candidate **一次**。两组各约50–60分钟，原90分钟守护不变；失败全留、零替代重试。
- 计时期间不插入自己的其他Turing编译、产品测试或共享source-cache写入。本地源码/文档/证据审计可继续。

**串行自动衔接已运行**：本机coordinator PID **61277**，exec session **48333**，脚本E/ops/run-policy-stability-pair-v1.py。**接续先读 E/iroh14-policy-stability-progress-v1.json，实际phase/run优先于本页静态时点**；完整结果固定写E/iroh14-policy-stability-results-v1.json。local flock防第二个coordinator。该进程活着时，它是root唯一ledger writer，模型只读，不能另行stage/launch/collect/specialized verify或启动其他Turing重型任务。
它等待现有baseline，顺序收件/identity/原stability审计、在准确原receipt/group空后只stage/launch一次candidate，再顺序收件/核验/输出描述性成本对照。任一控制/元数据错误停止并保存原receipt，不重复提交；原产品红可按冻结协议保留后测另一arm。十次只读SSH连续失败或超过100分钟未观察终态会停协调而不替代原run，远端90分钟守护不变。协调器只执行本批，不晋升、不改源码、不自动派发混版。源码/语法/合成比较准备记录E/iroh14-policy-stability-coordinator-preparation-v1.json；两份原proof始终为行为依据，不凭自动化宣称接受。

执行前冻结输入：E/iroh14-policy-stability-execution-v1.json绑定双方源码、control、readiness与四份协议/审计/物理结果SHA。
协议v1 SHA **e89aaa0c657e0bb26e7dfd3f01312a417c9de0f87bd1bff41cc61407b0aa86c6**；v2 SHA **08c27588260363d228c415769d4e96fa01ec71217379e7292eeeae51ccc63b0c**。
两arm都用iroh1.4.1/Noq1.4.0。baseline为028原25%收益门与旧恢复完成顺序；candidate为3b绝对5/10ms收益门和ACK顺序修复的整体策略对照，不作单一因果归因。

| arm | overlay source / tree / snapshot | compile-only准入 |
| --- | --- | --- |
| baseline | 515e99f0d886de8035801801a10e7587167cf78b / f0005b7544d4e5d93eed5e3dfe19ac021960980b / 10bb1bd227abb145b3591d94dbb413f8b7d66f49 | 154359Z-45ffa1ec，0/77.939305s |
| candidate | adf39a4c1adbae1a7b35bf98e1a6e9815f7db1b7 / 716caddfe8c009c8055d5d3b839b654474130f5b / ac1b99c3fe292f00b449e233c372aa21c2ac7644 | 154632Z-eb0a7d58，0/77.974962s |

描述符E/iroh14-stability-{baseline,candidate}-source-v1.json；工作树khor-net-iroh-stability-{baseline,candidate}-v1，分支experiment/net-iroh-stability-{baseline,candidate}-v1。
两个compile-only均15原files、control e532f19a4bb8939a14a921eaf3d954ccdd77cf5e，format/locked netlab artifact/三锁1.4组/source diff空/hash/identity/group空，readiness proof与ledger已闭；runtime0。

两边相同fixture只新增p1h高RTT抖动wrapper和测量结束后peer/here原journal导出；去掉25插入行可逐字还原两父源。两树仅差三app策略文件与iroh恢复完成顺序文件，锁/scripts/其他源码相同；source audit SHA **8e8311b241ccc790c6cbd3f9a6dd20cf8b5600f2a79d3fae29db95a08e17d3c6**。
原p1/p1j/p1s/p5及新增p1h保持600秒窗口、p5每档四次150/300/600ms stall、CPU contention验证、单线程/单repeat/diagnostics0。p1h调用one_speed(65,5)，约130ms RTT。
原gate：两端各最多一次选择、零retired；p5短于500ms档零Relay切换/零punch。不缩观察窗口或改生产预算。
原journal KEEP300，约600行轮转；要求每份非空bootstrap journal最早保留事件不晚于窗口开始。28份raw导出核保留范围，不宣称上游事件从未丢失。
E/ops/verify-policy-stability-v1.py已备，准备记录E/iroh14-policy-stability-auditor-preparation-v1.json：旧真实四case/22 CPU记录及六种错误副本检查，零新增产品执行。核验器会保留原红，不用数据完整性检查替代Rust结果。
p1s/p5原helper忽略echo错误；单端接口包/字节不是全网成本或能耗；该批不接受普通延迟/p99/完整N1或手机。

## 最近修复：首个恢复ACK不再提前撤回有界质量确认

[IROH-RECOVERY-ORDER](IROH-RECOVERY-ORDER.md)为完整设计、红绿及物理协议。冻结生产候选：
source **3b734073b410ec4130b4b57def3bf096b10c3e7a**，tree **473ee06d1a836d56d9e39bd10a7a4016fdacf530**，snapshot **4a688a33bd3da7f095cd8880ed1fd9519fb5ce09**；树khor-net-iroh-recovery-ack-order-v1。
只把complete_read内三行改为先recovery_probes、再mark_silent。反转即恢复红源，测试段及其他文件逐字相同，source audit SHA **a814baae82dab7b97cc92c24b620c8ee08647104f5d6a155d7c20c75bc2d2f5d**。
没有新状态、timer、quota或新鲜性豁免，原三次/250ms/2秒质量确认与四秒物理门保持。实际被补上的episode可能增加探测，不能由预算不变宣称成本相同。
旧read_paths已经传latest_rtt；真实问题是首个post-judgment ACK可让mark_silent在RecoveryCheck建立前撤掉Silence。不是median参数错误，也不是全部首连/NAT/手机失败唯一根因。

| 准入/固定批次 | 原结果 |
| --- | --- |
| 红150712Z-549ba3dd，source70a6f7d9 | 101/200.479139s；Noq467过，iroh原197过/原ignored1；只有新增first recovery ACK测试红，实际131ms/应85ms；node test compile绿 |
| 绿151200Z-d504e81e | 0/180.387027s；Noq467、iroh198含新回归过/原ignored1，node test compile/锁/源码diff保持 |
| 同源12app 151523Z-db3999ab | 0/379.316808s；276过/原ignored1，原七取消合同保持；不是完整Node/Quality |
| 物理R1 152244Z-802bb00c | 0/200.580265s；124全ok/final carried true/原warm及decision过；X/Y Go3649477/3640302us |
| 物理R2 152638Z-8914d68c | 0/214.366541s；同124全ok及两门过；Go3532073/3798379us |
| 物理R3 153157Z-8dea5447 | 0/152.258487s；同124全ok及两门过；Go3766136/3757942us |

以上原文件SHA/source/control/identity/group空、对应proof与ledger均终态。**物理三轮批次已关闭，不追加同源试验。**
结果E/iroh14-recovery-ack-order-physical-results-v1.json SHA **f1a4c7ef3eb61470fb5f5c377265c224ecdebde91b460d1a80eb41296136e5d8**，fixed3/additional0/screen_passed=true/**accepted=false/promotion=false**。
372逻辑请求全ok，六个Go3.532–3.798s；不是首连失败率、p99或任意移动场景接受。
E/ops/audit-recovery-episode-cost-v1.py仅复算已留日志；每episode最多三reservation。R2全启动20episode/56reservation是10PING请求+46merge，换网后两Custom episode各2PING+1merge；不要把reservation全算额外发包。

## 最新稳定升级及custom退出

官网latest在本轮16:00 UTC再次核验仍为 [iroh1.4.1](https://github.com/n0-computer/iroh/releases/tag/v1.4.1)。官网BLE/Tor为experimental custom transport，不替代khor的两腿选择/信任/自动接管，不新增本轮承载范围。
独立升级基线02807c79e04a92c46fb2e6c14d2ecf107b98336c（khor-net-iroh-upgrade-v1）为iroh1.4.1/Noq1.4.0；C3b在其上修策略。
Canonical feat/net仍原运行时 **5064fe2a**、iroh1.2/Noq1.3；后续docs提交不冒充测试源码，没有集成/发布升级。
E/iroh-upgrade-patch-ledger-v4.json：60历史项=1上游替代库验证、2移除对照红而保留、57待退出核查；57不表示全都必要。42–44原隔离编号未合主。

- iroh22手工IPv6 trim已用上游relay_tls_server_name替代，库合同过；原IPv6环境可提前返回，非WAN/手机接受。
- Noq5移除091835Z-cb319e57：四原合同三过一红，合法ACK-only最大号已回收时误增RTT计数5→6；基线同合同绿，保留。
- Noq2移除093340Z-deb1f7de：始终reporting both时，握手响应前早数据错误发送ObservedAddr TX1/应0；基线绿，保留。
- E/iroh14-custom-native-comparison-20261113-v1.json新增静态核对4/6/15/16/27：1.4.0仍按may_send_data归集ACK/连接timer ping所有路径；1.4.1仍有Relay close/queue/route无界等待、forced major通知不公开。没有新删除或行为测试；不能由该静态检查宣称这些补丁对所有部署都必要。
- iroh21容量64覆盖可评估上游Lagged+paths快照，当前roads::follow的mine/Openings未在Lagged对账；需真实burst消费滞后合同。Path::ping是该历史条目另一行为，分开退出。E/iroh14-event-buffer-exit-audit-v1.json。
- iOS callback链存在，同interface地址-only变化不独立穿过status+接口类型去重；iroh另有监控，不能静态归因现场故障。E/iroh14-ios-notification-static-audit-v1.json；真机仍待。

028已闭证据：Linux084724全vendor467/197及Mac43186999295同人口、085339固定12app273、090144真实流36/独立74Fetch RPC、095017实际CLI取消两合同、100747诊断12静默/12准备/12STREAM阳性、102059三实际接口场景372逻辑请求、111507 IPv6重叠248、111859实际QUIC NAT端口124（30/30远端warm stream新tuple）。准确范围/收据见[IROH-UPGRADE](IROH-UPGRADE.md)与历史，不相加重叠人口，不挪用于3b最终验收。
完整Node旧dc738与新028：同1387人口、1274过/同38失败/75原ignored，无new-only；均非完整Quality接受。Git2.25.1/tmux3.0a前置只是部分原因，账户runtime等未闭，不能一概豁免。

## 已关闭的诊断和普通性能批次

所有旧红、非唯一根因和不可晋升结论仍保留；以下均不得原样追加挑绿，完整ID/proof在历史与对应设计文档。

- 028 peer首连105138红、110840可达但四秒门约9.3秒红；旧dc738110236约3.47秒绿。首连错误不是注入换网后失败，未定位。
- 被动pending诊断08ecab50固定三轮闭；确认copy候选80固定三轮为首连红/恢复红/一绿；均不晋升。[IROH-CONFIRM-COPY](IROH-CONFIRM-COPY.md)。
- hop准备诊断794固定三轮均可达但decision红；只采日志，无唯一首连根因。
- 绝对收益门源67固定三轮：一绿/两final carried false；保留失败，不接受单改收益门。[IROH-HYSTERESIS](IROH-HYSTERESIS.md)。
- 同app/旧依赖源16e943d8固定三轮均过；小样本及非匹配顺序不证明新版唯一回归。两次旧compile准备含错用1.4 metadata红，均留证。
- 原生TRACE源bc3064af固定两轮：一绿/一恢复5.41、7.40秒红；捕获迟恢复，未捕获持续无carried。日志缺唯一Noq drive id，不能合并相同stream号猜连接。Noq issue889大型并发wake报告非本3节点根因证据。
- 028 vs旧dc738普通固定六block全部闭：8640交互/12file完整，有文件收益也有交互代价和新681ms p95/1.10s最大、scheduled1.89s尾巴；[IROH-WORKLOAD-RESULTS](IROH-WORKLOAD-RESULTS.md)，不代表3b性能。
- 原prefetch18场景、rolling24场景与所有原240秒文件超时均保留；rolling12block/17280交互全ok、21文件完成/3超时，不接受全局最优或完整N3。四源码/四真实RPC/Job stop控制合同已闭，生产server/worker/host-child仍待。[PIPELINE-COMPARISON](PIPELINE-COMPARISON.md)。
- 历史严重6aa及block9终端1.2–1.56秒/scheduled8–12秒未闭；后来三轮未重现不抹掉原失败。旧idle16–17秒不满足极致体验。原quiet v1准备后仍有STREAM的十二红和v2诊断准备修复均保留，[IDLE-ACTION](IDLE-ACTION.md)。

## 环境、所有权和下一项准备

- PRODUCT `/Volumes/disk0/Khor/worktrees/current/khor-net`，branch feat/net，创建基线6485cb83。
- CONTROL `/Volumes/disk0/Khor/worktrees/current/khor-build-net`，HEAD **5d3691f252f9dee0297fa3c494374be6119bca29**；新controls只建Git对象/ref，不移动HEAD，既有__pycache__不覆盖。
- EVIDENCE **E=/Volumes/disk0/Khor/evidence/net**。root是source-refs.json唯一串行原子writer。stage/launch/collect/verify须等真实exec会话终态；unknown handle先读receipt，不再提交。
- 本机source-only：不跑Rust/Cargo/rustfmt/Node/Bun/Xcode或产品。重型Turing/macserver+私库colehank/khor-build；本地Git、stdlib Python源码/元数据和bash -n可用。
- Turing原UID1110/flock/private user-netns/Rust1.95/jobs16/nice10、50GiB启动/40停止/90分钟守护保持；不sudo/改宿主网络。
- Mac50GiB准入保持；43189070840空间门红后43205919189在付款/额度阶段失败、零Mac操作。最新GitHub只读仍该run，无盲目dispatch；16:00后SSH再试仍banner timeout255。异步额度/常规SSH问题未获答复，不降门/清未知缓存。
- MacLab与Turing同LAN，macserver独立；localhost旧0.2.0 iPhone以前反复关开恢复不算候选自动接管。没有新Lab/手机窗口或候选安装接受。
- 无本任务main合并、发布、生产Relay采购/部署、已装App替换授权。中央GUI授权不套用。
- 图谱缺失/partial/stale或不覆盖vendor时，0节点不是安全证明；先用已有图谱，再定向源读，不自建CodeGraph索引。

混版协议和control已在本地准备：[IROH-MIXED](IROH-MIXED.md)。E/iroh14-mixed-version-plan-v1.json / mixed-control-v1.json，control **05419f73cadb7787ee5c4356004b79f672a2c1c7**（仅原d07eec56 net-node分支）。旧dc738/new3b准确CLI native来源→不可写副本→真实sync_mixed唯一正标记/两方向row+pin、独立两host/child观测与退出。原fixture/helper/native工具两源逐字相同，Python AST/bash syntax已查；**零新远端编译/产品执行/互通接受**。两长稳全部终态收件核验后再执行一次，不抢Turing计时。旧owned run110236的bundle/package须核SHA；缺OLD_KHOR直接绿不接受，短唯一TMPDIR/只碰私有tmux，保留homes以免兜底reap隐藏close失败。
E/turing-build-tools-inventory-20261112-v1.json确认Git2.25.1/tmux3.0a/libevent.pc缺失，只有只读库存；未安装/编译新工具，长测中不可用准备名义插入重型执行。
Mac Finder trash原AppleEvent−1712（32/33）重复确认已用尽，保留失败；权限/响应根因未证，不盲目同源重跑。

## 完整剩余范围与完成定义

- **N0**：最终分场景UX门、充分尾部样本/p99、同条件完整前后对照、观测成本及成本前沿。
- **N1**：当前长稳两arm；严重/quiet/recovery尾部、固定好路径参考、跨连接质量/旧样本、启动/配对失败、动态延迟突变、后台流量/唤醒。
- **N2**：同一实际连接的独立/同时IPv4/IPv6地址/前缀/NAT变化，换网/休眠/黑洞/LOST；owner/cancel/late/revoke/carrier失效；身份/总期限/写入和0RTT不重放。
- **N3**：完整前台文件/预览与TTY尾部，公平/饥饿/背压、共享与分连接实测、真实server/worker/host-child取消、1/3/5设备CPU/字节/唤醒/能耗。
- **N4**：Direct/公共Relay/peer relay/SSH双腿，唯一middle自动救通（第二腿可Relay/旧信息），多个/移动middle、独立双栈变化，北京/香港整机/出口/寻址故障域、自家Relay约束、双家/地域/容量/运维设计。采购部署另授权。
- **N5**：60历史补丁逐项上游等价/保留退出，Noq1及各物理成本缺口、feature/integration、最终准确源码Quality/完整Node/native/proxy、Finder、混版与真实手机自动换网/peer relay/能耗。

完成须设计理由、状态/职责边界、持续验收矩阵和同条件报告均绑定准确版本。单次绿、三物理screen绿或库升级不等于完成。
此前2–4天、4–8小时为低置信估计且偏乐观，不因compact重置；目前没有可信整体完成日期。当前两组长稳约两小时只覆盖一项，不承诺整计划两小时完成。


## 最新排期估计（2026-11-13 00:47，本地时间）

用户再次问完整完成还需多久。按当前六块未闭工作，最新用于排期的低置信估计为：**完整计划还需5–10天持续开发与验证**；具备Lab试用条件的候选目标2–3天；当前固定两arm长稳剩约1–1.5小时。不是确认交付日期，不把当前测试批次结束当完整计划完成，也不从compact重新起算此前消耗。
估计以前提Mac构建通道尽快恢复、真机验收可配合为准；最不确定的是首连/唯一middle自动接管、CPU忙及传文件时长尾、新旧互通/最终整合，以及57项剩余custom的退出核查。此前估计偏乐观，后续根据明确门的结果修订，不用单次green代替完成。
