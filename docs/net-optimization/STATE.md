# feat/net 当前状态与恢复入口

更新：2026-11-10 19:27 UTC（新加坡11月11日凌晨）。本页记录当前状态，完整此前段落逐字保存在[历史](HISTORY-20261111-after-scope55.md)，其中链接到[上一历史](HISTORY-20261111-before-current-state-compaction.md)及更早记录。失败、旧源、fixture与审计不覆盖、不删。历史中的运行句柄不当当前。

## 授权与工作位置

完整**N0–N5尚未完成，用户已要求持续实施**；不得因分项绿、compact、状态问句或旧goal服务paused停工。root串行，无子智能体。速度/体验优先，个人可信设备网；peer relay、IPv4/IPv6随时变址为硬范围。没有main合并、发布、采购、生产部署或改已装App/config/store/resident的授权。

- 产品：`/Volumes/disk0/Khor/worktrees/current/khor-net`，`feat/net`；先核HEAD/status/diff，不按摘要覆盖WIP。
- 控制：`/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`，固定`68cadb57a0ed72d40a4ca9a843e86b9c6797bf34`；两处既有untracked `__pycache__`保留。
- N1隔离：`/Volumes/disk0/Khor/worktrees/current/khor-net-recovery-deadlines`，detached `2ef6bee5a5aaf841af066f4edb5c894769d4539d`，clean，未接受。Iroh42–44在该树预留、未合主，不覆盖它。
- 证据：`/Volumes/disk0/Khor/evidence/net`，ops在其`ops/`；`source-refs.json`单写者，实际receipt/ref/tree/control优先。
- 完整范围：[PLAN](PLAN.md)、[REQUIREMENTS](REQUIREMENTS.md)、[ACCEPTANCE](ACCEPTANCE.md)、[NEXT-DESIGN](NEXT-DESIGN.md)、[REPORT](REPORT.md)、[VENDOR-AUDIT](VENDOR-AUDIT.md)、[审阅](REVIEW-20261111.md)。

## 最新已验证分项：923baaeb

**`923baaeb6a6e3b5d7bfe49e17ff28330ae89d29b`**，tree **`e8893d457afe920ad6852f84fb280780f233a111`**，snapshot **`37d77b42a8ea001002ed3fec8a2d09436a656d74`**。

- Linux `turing-20261110T191324Z-828ae604`：全12组/hop55/wires16/budget30/resume1；315.098秒、21files、group空、identity及全部实际summaries核。
- Mac `42883511676`：同canonical全12组/hop55/wires16/budget26/resume1；exact artifact **13794773913**、20files、11control blobs核，目录`mac-42883511676-hop-epoch-packet-age-v1`。已终态success，重型已结束。
- 新窄范围：MISSED只对原routing-wait epoch有效。Wires唯一持有Arc<()>，操作/cache仅Weak；原forgive_all替换token；open_hop首await前捕获，MISSED同锁校验活token且只清同epoch。无变化仍按原30/60/…/600秒退避。每(me,to)仍一项，不按换网增长keys，不留Node活；不得让读取者clone strong来延长旧epoch。
- 另闭合incoming packet年龄：使用当前selected IP/Relay真实Noq `last_received`与原Patience；Unknown/Custom/缺timestamp不猜。真实握手/echo后显式now/future快照年龄对照通过，旧计数假新鲜红保留。**这是年龄模型，不是实际OS/iPhone暂停证明**；睡眠时钟/网络路径世代仍待N4。
- hop-miss与packet-age两个freeze仅scope true/field false。旧98b997de为hop53/2红（网络前/后NO_ROUTE均挡新网、无变化控制过）；旧7bb0e664为hop55绿/wires15/1红（旧包被计数当新）。格式31ca/713f/bd43全部原样保留，无行为结论。
- 相对489477的图谱detect8files/24entities/22gaps、flows0，build落后；定向diff/真实tests为依据。egress出口状态变化也调用forgive_all，已核它不是每个ACK失效。只关闭这两个scope，LOST/peer与middle逻辑地址context、所有入站admission、N1物理时限/N3/N4/N5未闭合。

此前已接受窄范围与准确红绿源均在历史和freeze：

- 48947721：同源Linux95f8da6b/Mac42878663761全12/hop52，exact13792758066；unused prepared carrier无owner释放、pending期间保留/结束释放、真实incoming-only首腿建立新Tunnel及carried TLS/echo、closed负控。
- 801dff9e：同源Linuxaa81601a/Mac42873590614全12/hop48，exact13790762470；initial Drop与normal watcher撤回含新有限pending消费者，保留两个无消费者释放控制。
- 153263c2：同源Linux8adb26d3/Mac42869844599全12/hop44，exact13790105312；首次错误身份旧IP拒绝后新peer relay在原2秒caller内Sessions一次，NO_ROUTE无业务请求、live wire拒绝hint撤回。vendor1e1264ef Noq439/path50/home11绿。
- 主vendor总54（Iroh1–41/45、Noq1–12），42–44隔离未合主；45只在无live连接且旧来源时间<=首次失败边界撤旧IP，新来源/Relay/Custom保留，不是完整地址epoch或永久blacklist。923相对此vendor代码未再变，无无因重复vendor运行。
- 更早sole-peer Recovery准入、ordinary与cold/resumed读取并行、pending lease/cancelled watcher successor、live胜旧cached排序、取消/响应owner/STOP/contact/actor conditional carried-close等按各freeze保留，不能升级成全部入口/代际安全。

## 当前唯一重型任务与下一步

**`turing-20261110T192533Z-0aeceb60`**，PID2771745，19:25:55 UTC启动，task **netlab-peer-mobility-samples**，revision `main55_verified_unit_peer_relay_replug_checkpoint_v1`，role baseline。**准确复用上述923同canonical**，control68cad；receipt见证据根该run。Mac job已完成后才启动，计时期间不派Mac重型。不要重复提交。

这是main55新checkpoint，**不包含隔离42–44**。按原`peer_relay_replug_v1`真实middle IPv4 replug、20ms腿、原两次10秒hold、124个业务结果及原四秒warm request选择合同。full samples诊断与旧隔离profile一致；但source/诊断字段不同，不能作已校准paired因果结论。可用性124/124不等于选路四秒通过，更不是p99/真手机。

1. 先只读核execution/task tail；重型profile无重叠。终态且group空才collect，不重收既有report。
2. 核identity和实际fmt/compile/scenario。`ops/calibrate-peer-mobility.py FOLDER`核身份/物理地址/hold/124结果，之后产品脚本`scripts/netlab-peer-selection.py FOLDER`独立原四秒合同；诊断可用`netlab-peer-state.py`、`netlab-peer-decision.py`及ops/peer-outer-timeline.py。先读各接口，不猜参数。
3. 若红，保留全部失败，按本准确源生命周期/路径证据定位，再决定是否整合或替代隔离42–44；不直接把旧隔离结论当main当前表现，不盲重跑/放宽期限。
4. 同时继续完整范围：incoming live对两cached竞争、NoRelay/ToNoRelay未知第二腿、MISSED的peer/middle候选/地址context、LOST owner与失效、无pending入站登记竞争、静默黑洞下新carrier接管、accepted写jobs/late losers。下一设计见NEXT-DESIGN，不把提议写成实施。

## 全计划仍欠的完成条件

| 包 | 未闭合 |
| --- | --- |
| N0 | 最终体验SLO/容差/采样量与前瞻paired校准、足量p99，失败/超时不删分布 |
| N1 | 原物理恢复/quiet成本/全矩阵；隔离2ef bf6105dd虽124/124仍5bad，X5505.782/Y3496.330ms；quality确认窗重启及cold start红保留。main55正在新checkpoint，不称通过 |
| N2 | 上述更多admission、所有地址/network contexts、全部入口ownership；当前仅各自实测scope |
| N3 | Custom1024frames/256KiB公平性，SSH/hop混合、shared/separate 1/3/5，交互/CPU/字节/唤醒与手机能耗 |
| N4 | 实际两腿Relay/唯一middle/断腿/多middle，独立v4/v6/同时变址、Wi-Fi↔蜂窝与peer/middle移动，手机自动恢复和拓扑验真 |
| N5 | 原53+45、隔离42–44逐项审计/上游退出，Noq1/5缺口，最终Quality/native/proxy/多设备/手机 |

独立原失败保持未解释：pair core transfer Busy854aa6ce、原resume32 timeout、N1物理四秒/quiet/cold-start。后来的分项绿不自动解释或关闭。

## 真机、构建与证据边界

MacLab与Turing同LAN；构建macserver为另一台，异网来自用户报告、尚非独立NAT证据。iPhone localhost旧现场Wi-Fi→蜂窝→Wi-Fi自动恢复失败，反复关开App某次才经Turing恢复；未证明OS force-quit。截图IMG_5608蜂窝5GA下hinton Direct29ms、多peer不通，只证首腿可见，不证第二腿、当时源码或精确故障时长。

**旧手机采样已结束，候选未安装，现在不做phone RPC/sampler、不要求亮屏/切网。** 原79样本47ok/22busy/9paused/1error不作候选SLO。REMOTE-DEVELOPMENT明确便携Mac网络executor不是native检查，开发机不生成正式iOS签名包，iPhone用可信GitHub TestFlight；没有为测试修改该边界或进行发布。当前网络unit绿不等于App产物，后续原生/可信Quality/分发仍独立。

本机source-only：不跑Cargo/Rust/Node/Xcode构建/测试/formatter/下载执行。Git/Python标准库元数据核可用。Turing只走既有UID1110/flock/user+netns/build admission，Rust1.95/jobs16/nice10、50GiB起40停/90min；无sudo/host网络变更。Mac原单cache/jobs2/50–40GiB/100GiB guard不改。无重叠Turing重型；普通unit可Mac+Linux并行，计时netlab不与Mac重型重叠。SSH不转发agent/凭据，无子智能体、无CUA。

图谱先行，仅导航；旧HEAD、flows0、tests_for0不是安全/覆盖证明。避免宽读crates/net/src/endpoint.rs顶部编译relay bearer token。collector仅终态/group空，verify只核identity，必须读真实日志。Mac只下载精确artifact ID，核11 control blobs。source ledger单写者，stage时不并发写；exec session先poll再依赖。

最新只读refs审计`source-refs-audit-20261110T191453Z.json`：457 refs/tree、64control labels、363显式receipt存在，94legacy缺字段保留，ledger SHA前后不变，当时仅828 pending；不是runtime/artifact全审。source-cache已保留未变文件mtime，不编造缓存瓶颈；保持jobs16/原测试观察窗。

证据路径脚本错误已纠正：scope55 freeze相对路径从control cwd查找失败，未改代码/测试；Mac ledger有效，两个freeze已用绝对产品根核hash并scope true，`scope55-freeze-path-recovery.json`保留。物理提交已存在，不重复。

临时early serial/queued817/peer543 baseline worktree已clean非force清理，pins及所有源/run保留；early9b680476 pin为refs/khor/net-baselines/early-serial-calibrated-v2，不按历史重建/再删。

## 估时口径

16:20 UTC告知下一可评审候选2–4工程小时（原锚18:20–20:20），完整N0–N5含真机/换网/能耗3–5工程日量级。18:23已交代本批48窄scope双OS通过，19:25又闭合55/16；在原预计区间内。原13:00 UTC的4–6小时只指下一批，已解释新增缺陷导致延后与口径不清；不得每次compact/状态问句无声重置起点。完整计划未完成，继续执行，不承诺已解决手机现场。
