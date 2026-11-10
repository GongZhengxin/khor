# feat/net 当前状态与恢复入口

更新：2026-11-10 07:56 UTC。完整N0–N5仍未完成，用户授权继续。速度/体验优先，可信个人设备网；无main合并、发布、采购/生产部署授权。root串行、无子智能体。长期[PLAN](PLAN.md)/[REQUIREMENTS](REQUIREMENTS.md)/[ACCEPTANCE](ACCEPTANCE.md)不缩减。上一状态逐字在[历史](HISTORY-20261110-before-natural-recovery-epoch.md)，保留更早历史链。不要因旧goal服务paused停止。

## 先核真实句柄

- 主 `/Volumes/disk0/Khor/worktrees/current/khor-net`，feat/net。生产行为HEAD **ba28fc03**（之后文档/注释）；主53vendor、Iroh41。Mac42770379226+Linux4bcfad82同source全部net-node已绿，见下。不把doc-only HEAD误当已测试二进制源。
- 隔离 `/Volumes/disk0/Khor/worktrees/current/khor-net-recovery-deadlines` detached **4ec3ec08**，source tree5cb081e1/snapshot0555b9f0；无IPC v2、Iroh42未合主。f0a5db0e含Noq3诊断fields，1a6371d7含actualPING两模型；4ec只引入相同旧顺序helper与3个独立回归，**尚未实现Iroh43修复**。旧50tests完整SHA5c4da4b4不动。
- 控制 `/Volumes/disk0/Khor/worktrees/current/khor-build-net` feat/net HEAD **68cadb57a0ed72d40a4ca9a843e86b9c6797bf34**已推。net-node仅在原remoteQUIC32之后/canonical之前加resume合同；3个source-only stub控制测试实际绿。
- Turing目前无已知heavy。9bf已真实52/1单元红并采证；43候选1cb/tree237/snapshot4584的9cf已441/53/9绿，f2b832dc原物理124/124但请求11bad/GoX7728/Y3470红，全部已terminal/sourcecontrolbundle/原group空核。接下来Iroh44先旧行为helper独立状态红，再同whole绿；具体设计见NEXT-DESIGN/ACCEPTANCE 07:56。不盲刷物理、不放宽4秒门。
- Mac无已知heavy在跑。macserver SSH早已恢复成功；A=macserver/B=Turing天然异网，P=iPhone。源提交经本机GitHub SSH，worker仍原API bundle/guard，无个人凭据给worker。
- 用户已确认：**iPhone在手边，可切Wi-Fi/蜂窝；已装khor并已与Macserver/Turing配对**。两次可用性/安装配对问题均已答，不重复问。真机具体采样窗口尚未开始；先核实际版本/cohort/可观测API并准备短操作，不假称手机已通过或要求改生产store。

## 当前关键因果：自然恢复先清故障，漏建采样epoch

- **新真实红7bf72701**：f0a5db0e/tree1cfb126a/snapshot92afa1e5/control ef0b，原124/124、请求门绿，decision/state为X3658.183ms/Y7404.281ms红；预算仅X3预约/1episode绿。source/control/bundle16artifacts/228.574秒/原group空核。
- Y judge=1778813.572639，第一合法post-judge sample sent=.574443（晚约1.8ms），RTT485.855ms；随后263.720ms。read_paths先mark_silent恢复可达并移除Silence，再recovery_probes迭代剩余Silence，故没有RecoveryCheck。+1.4秒后pendingPING=false、inflight0、cwnd约28k、globalOther=false，直至+5.4秒自然keepalive得83.794ms才开始2秒Moving，+7.404秒Go。**此Y不是观测到的cwnd阻塞**；旧2c9d X缺样本仍为另一个未证分支。
- 已在099c3c6b先写NEXT-DESIGN/ACCEPTANCE：新增actor真正调用的recovery_tick，旧顺序保持；独立新fixture验证首个合法但慢的自然RTT恢复可达时仍保留有限epoch，随后post-epoch快样本按原25%规则更新quality。无故障/closed/旧packet负向及旧50不改。当前4ec基线待实际红。
- **红后拟最小修复**：recovery episode登记先于mark_silent清除故障；不把第一次ACK当优路、不放宽sent>judged/gain/两fresh/Moving2秒/四秒门，不增3预约/250ms/2秒预算、永久timer或强owner。selected+available/已有Moving仍合并，充分自然样本仍停额外PING。先单元真实红→同whole新fixture绿，再原物理两门至少3次、quiet/黑洞/成本；不能预记修复了7.4或旧5.8。
- 新fixture `vendor/iroh/src/socket/remote_map/remote_state/recovery_epoch_tests.rs`，module tests_recovery_epoch明确被原`remote_state::tests`前缀filter涵盖。3case；预计旧50+新3。natural-recovery-epoch-red-source.json/fixture-proof.json冻新whole与原whole5c4da4b4。

## N1已有证据边界

- Iroh42 8d91e1ec只让candidate confirmation复用recovery_copy_rank（低PTO优先，同PTO恢复copy粘附），current最高PTO规则不动。旧5a诊断Y1549ms取84ms后confirmation切另一copy、101ms代表使Moving撤销，Go7549；另一copy同刻PTO未知，不称同PTO事实。42尚未合主。
- 42 source894：4f2物理124/124、请求绿、X3262ms，Y始终同wire Custom/首stream14ms/首成功762ms；原decision缺Go仍红，补充continuity仅回溯探索绿。repeat2 **2c9d**为124/124但请求门及两端Go5825/5870ms红；X三API Ok后RTT49/1.1646秒不前进，未有实际tx计数；另一copyPTO2/unacked None亦近期收包，不能假设同PTO或直接猜cwnd。原失败完整保留。
- fullcounter v1 source1a/tree80a/snapshotd2，两轮6d83与ff132双端3585/3578与3285/3518ms绿，原124/124/两门/state/6预算2episodes绿；只得正常counter参照，不能倒推旧红已解，不盲刷同一诊断。原wholeNoq multipath、Iroh50tests、物理fixture保持。carried-ping-source-audit/fixture-identity/counters/reference留证。
- sourcef0a仅Noq3 PathStats扩bytes_in_flight/ping_pending/pending_other_frames（global pending不等于此path承载业务），两模型补API排队/tx前后状态；selector不读新fields、CC/PTO/发包/RTT数学不改。Turing505de602实际fmt/441/50/9绿，61.836秒/14artifacts/原group空核。提取器v2要求send_state测量三字段存在，旧版备份保留。上节7bf据此定位新的Y分支。
- Noq两个真实模型：健康backup确实PING发出/本路ACK/新RTT，黑洞API/tx不假刷新RTT；Mac427650+Linuxf1d19b07同1a完整441/50/9绿、13/14artifacts核。普通ping不等于PTO到期，不能为四秒绕过RFC9002拥塞边界；primary links在NEXT-DESIGN。
- **小补丁quiet匹配一对**：42候选da82e2b8 source894与41基线f9d50e90 source829，实际仅一Rust tie规则+KHOR记录不同，同control4eeee/原p1s600秒20×300ms+p5。两run全部32ok/22CPU/all live threads64/2hogs/source/control/bundle/11artifact/原group空核。候选0moves/671.7ms/6punch3362pkt1325754B；基线1relay@245s/881.6ms/18punch4632pkt1706984B。p5均绿。confirmation-copy-matched-quiet-comparison.json accepted=false：单对无重复统计/p99/手机能耗接受；初始diff checker漏KHOR文档的assert已修并留erratum，非产品红。
- Iroh41首物理ad627双端3816/3613绿、repeat34871为3747/7238红；Iroh40约5.4秒双红、Iroh39约4060/5799红、现代pre39 baseline约7674/7913红。原Noq12 quiet两candidate1/2move、旧第二at-most-one红；现代旧baseline零move/worst539/635ms。Iroh40 quiet71aa一move/1.3秒不算性能成功。ack-floor-controlled-stalls-comparison.json保留所有负面证据。
- 040407/042825等初连10.032秒超时未replug保留cold-start独立红，不混入恢复时序。N1仍未验收。

## N2已闭合的分项：真实暂停恢复、STOP、回复所有权

- **暂停恢复真实因果红绿**：Linux旧c2ec在原canonical主动pause→resume后731行立即Cancel被paused拒绝，先前主I/O/cancel已完成；最初误称first/precondition已在ledger/proof记erratum，保留真红。随后独立真实Node held-next-poll合同，Mac427683仅fmt失败（不计行为），精确patch4b7后 **Mac427690实际paused拒绝红**、wholecase6.25秒。原四whole fixtures冻结。
- **ba28候选同source双OS绿**：treecf0b29d4/snapshotf9efe7c2/control68cadb，Mac42770379226 +Turing4bcfad82全部12项net-node实际绿；新resume1.74/1.83秒、原canonical含pause/resume6.87/4.24秒、actual32STOP1.70/2.16秒、portable/IPC18/local32/process/remote32/budget20Mac24Linux均绿。分别11controlblobs/20与21artifacts、Linux原group空核；四whole fixtures与实际红逐byte同。resume-admission-red-green.json candidate_verified=true，scope限此恢复bug。
- 修复仅serve初始pause::poll初始化共享cache、incoming准入读node.paused该cache；accept热路径无磁盘读，不改半秒poll/持久job/预算/重试。之后f381仅纠正Node.paused及link注释。跨process文件通知仍原半秒poll，旧IO读覆盖新本地cache潜在generation竞态未另证明，不能偷带广重构。
- **actualSTOP同source双OS已闭合**：32Window实际encoded barrier/permit0、caller abort/join、32个Ok(Some(STOP))、零connection-close替代、同B stable wire、barrier仍持有时同wire Begin/Cancel成功。quic-explicit-stop-wire-proof.json更新ba28同cohort，首次Mac427613/a00证据逐字备份quic-explicit-stop-wire-before-cross-platform.json；旧c2ec没跑到STOP保留。不是WAN取消、accepted写jobs或手机能耗证明。
- 原remoteQUIC32 Mac427545真实5秒回收红→757保resp/frame至finalwrite、仅Resolve/Window/Record可取消，Mac427556+Linuxd964同source3f0全部net-node绿；whole两个原fixtures/red-green命令前缀核。quic-response-owner-32-red-green.json。Begin/Cancel/写job/Watch语义保持。
- caller短期427298真实红→427332原whole块绿；本机32旧427371真实红/whole945c→首次lease427389 SIGABRT保留→Box::pin后427399旧fixture绿。独立process427531约1006ms预算/真实Node join/child reap、坏cookie/额外字节无副作用、legacy halfclose绿。相关proof锚原sources，不拿新增module后whole hash倒填。

## N4当前真跨网与手机

- 当前已执行WAN cohort仍 **757/tree2d218/snapshot3f0/controlccd**：coordinator4cee410c97601afe/Mac42757732776，Mac↔Turing各30目录+30固定29B正文逐byte验证，120/120全部ok，两端fresh配对ID相符。MacbinarySHAcd9be1a8/Linux0e063201。实际公共Relay101.200.80.77:3340，不是Direct/peer/v6/mobile/p99/吞吐/手机能耗；也非与旧ff608匹配性能比较。
- worker_us最近秩p50/p95/max：Mac→Linux目录55382/58815/76133、正文61322/64630/66470；Linux→Mac目录59527/64622/65616、正文55671/58949/59727。caller另存proof。wan-current-directory-body-proof.json。
- 同cohort Mac3/3+Linux3/3 stop/EOF/kill真实Node join/worker0/owned空/锁释放；最初两observer IndentationError留原并readonly恢复不计pass，然后新六轮绿。两个business fresh identities与8lifetime copies直接验证清理、一次GH secret已删。manifest current3f0/旧ff608备份，误directory_v1 ledger label保留并actual_v2+erratum补正。
- iPhone可手动Wi-Fi/蜂窝且已装/已配A/B。准备实际版本/cohort与现有可观测API后，给短、明确采样窗口，记录全outcomes/首成功/选路/实际v4v6。尚无手机观测，不混称当前candidate已在手机执行。

## 全范围仍要完成

| 包 | 余项 |
| --- | --- |
| N0 | 最终SLO/容差/样本量/p99充分性、同测量baseline/candidate/固定好路/全失败/负向校准；补充state checker冻hash及18数据正负控制保留，旧selection/decision/物理门不改，回溯绿不充prospective |
| N1 | 自然epoch实际红绿→原两门同source≥3次、旧2c X/独立cold-start、原quiet/黑洞/成本；冷暖多候选/多middle双向、201/208/244/242慢方向、原hop/recovery/dynamic/stalls/full矩阵与162/182红 |
| N2 | accepted写job所有权、两middle总3秒、输者/晚候选、地址epoch/重启/jitter/quiet epoch/close-vs-offer、写不重放/legacy；真实WAN取消 |
| N3 | Custom1024帧/256KiB外层队列、SSH/hop混合公平/共享vs分连接、1/3/5设备CPU/字节/唤醒/手机能耗；Noq704请求模型不能替代 |
| N4 | U12 v4/v6独立/同时/切族、peer移动、Mac/Turing/iPhone冷暖不对称、登记/出口/整机故障域；布局容量运维数据，不采购部署 |
| N5 | 主53/隔离42及后续43逐项vendor coverage/上游等价/退出、Noq1/5缺口、最终源可信Quality/nativeApp/代理/多机/手机能耗；证据前不升级删补丁 |

## 执行硬约束

本机source-only，禁Cargo/Rust/Node/Xcode产品build/test/formatter或下载执行物；Git/Python stdlib元数据/数据checker/纯stub控制合同允许。Turing owned `/tmp/khor-netlab-feat-net-1110` UID1110、单flock槽/Rust1.95/jobs16/nice10、50GiB准入40停/90min，仅fresh user/netns，无sudo/宿主网络改动，计时不夹编译/其他heavy。Mac走既有run-build/build_guard单槽/jobs2/50–40GiB/100GiB target，resident不脱离guard，不改installed policy/ensure-budget/已装App/生产store。

SSH BatchMode/ForwardAgent=no/StrictHostKeyChecking=yes；无个人key/PAT/agent/签名/模型凭据给workers，不重设HOME/CODEX_HOME。WAN能力只私管道/一次secret，禁止普通artifact/日志/聊天票据。超时先查receipt/GH/proc/PGID，已有bundle/run先recover不重发。stage/launch显式完整source/control、前驱terminal采证/原group空/锁核；exec session不是结束，commit hook完成后才snapshot；ledger禁止并发writer。

图谱先，缺vendor覆盖才定向读；无.codegraph不自建，无子智能体授权。旧baseline/control WIP保留不reset；终态+owner/hash核后只清本次资源。verify工具只核identity，必须分fmt/compile/fixture/executor/product行为。证据根`/Volumes/disk0/Khor/evidence/net`，source-refs每次pin、最近全审329refs/39ops，后续新增逐条pin，稳定后刷新全量。恢复先核上方句柄，持续完整目标，不因分项绿或旧goal paused停下。

07:56新事实：43排序修复已在同whole新fixture真实52/1红→53绿，Noq441/home9均绿（proof vendor_candidate_verified=true），但first physical f2b832dc失败，candidate_verified仍false。X85→112稀疏质量造成Moving清除、停止补测，new44只分开有限采样/连续迁移确认，原radio总成本仍要验。phone-readonly-preflight-20261110.json保留Turing0.1.15混合cohort：首次devices本地document transfer busy，重试读取成功；对已确认手机localhost只读journal0minute一次360ms/valid空JSON成功，CLI总耗时非网络RTT；未开始受控换网/能耗。macserver只找到KhorDeveloperLab模拟器、标准路径无已装CLI/App，不能拿MacLab生产设备条目当macserver同身份。用户手机测试暂缓，不影响服务器目标。
