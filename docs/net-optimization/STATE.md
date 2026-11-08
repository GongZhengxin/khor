# feat/net 当前状态与恢复入口

2026-11-09 · owner=root，串行，无子智能体。

## 授权、位置与完整范围

活动目标“完成计划”已授权PLAN实施/验证，取代最初仅规划状态。速度/体验优先，个人可信设备网。未合main、未发布、未采购/部署中继。本机source-only；Linux用私库GitHub job，Mac沿macserver。其他GUI/治理worktree、现装App和用户原会话保持独立。

- 产品 `/Volumes/disk0/Khor/worktrees/current/khor-net`，`feat/net`；共享质量/ACK校正cba1ea80，失败Backup重排c31494d8，p2 v2协议86ce3330/680dee60、fixture edf79074、path事件诊断221b98fc，真实样本时间窗修复4fa3f1b3。p7相位扩展先在7cfa922d冻结，再加fixture；恢复先核git HEAD/diff，不能按摘要覆盖。
- 旧生产基线main/rc8 `6485cb83d3b1f0bb93d4633825dc956623d78af9`。`khor-net-baseline` detached该提交，只改fixture与远端测试脚本，生产源码原样。
- 控制 `/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`，最新0e5788c；781a907新增netlab-hop顺序三轮和net-node的journal测试，0e5788c给net-node补khor-net核心单元。两个main不动，产品feature尚未推origin。
- 证据 `/Volumes/disk0/Khor/evidence/net/`；source/environment/mode与原日志是准确版本证据。
- 权威索引：PLAN、REQUIREMENTS、ACCEPTANCE、DESIGN、RUNS、REPORT、NEXT-DESIGN。历史不是当前状态。图谱是部分增量图；先图工具，缺失后定向读，不擅自建立CodeGraph索引。

N0部分；N1仍有产品红；N2–N5未完成。完整范围保留，不能以单元或p6绿替代整计划。

## 已完成的修复及适用证据

1. 全承载速度排序/统一bind/门槛/迁移确认，hop发现与清理门槛统一。首版63pass，node hop9/wires10通过。
2. ACK样本计数、近2秒最多8观测中位数、有界STREAM活跃备用刷新。p7原45秒不回→首次绿；3次交互2.53–2.75秒、闲置19.64–19.73秒（接近20秒，尚缺相位/负载裕量）。但这一版正式p2/p3重复恢复回归，不能收口。
3. 共享quality/age/count输入：先63pass/2fail取红，再66pass/1ignored（42511021381）、431Noq/22iroh（42511084014）绿。缓存读取不探测/不唤醒paths_stream；质量与年龄来自同一副本。age仍是actor观测时刻，非实际ACK接收时刻。
4. ACK-delay校正：latest_sample与原平滑器用同一校正输入，原raw latest/PTO/拥塞数学不改。42511866017为432/22全绿；短矩阵42511895143为8pass/1fail，仅f4i初始未通过m前置红，p2 973ms、p3最坏696ms且零错/退役、f4/f4s/p6均绿。仅一次，不替代正式重复。
5. 同速黑洞在飞数据：42512359677为432pass/1fail，旧快RTT的黑洞Backup在移路时不释放数据。当前Noq补丁9/iroh30将原不应答判据移到PathStats::unanswered_for，迁移时坏Backup也重排；健康同速不重排，MTU排除/原丢包与拥塞计量/QUIC去重保留，不重放应用请求。**42513546687 SUCCESS：433Noq/22iroh全绿**，tree6465d105，controlc6a20c5，证据blackhole-backup-green-42513546687。新增用例含真实虚拟PTO与流交付，非只改RTT模拟故障。
6. 当前生产代码的动态三轮42513580645：p3最坏530.5/732.0/740.9ms，均零请求错误/退役；p7交互2.4233/2.6044/2.4252秒、闲置19.5757/19.5113/18.7489秒，六项全绿。此run整体FAILURE：旧p2的LAN-only门两红一绿，不可改写工作流结论。
7. 第一方格式修正后42514729034 **SUCCESS 66pass/1ignored**，tree7ba78370，证据backup-source-unit-42514729034。其后p2 v2 fixture和f4_stall失败日志尚未由这次检查覆盖。
8. 最新同v2 fixture正式A/B已下载：baseline42516040900为4pass/2fail、4/2、3/3；candidate42516056972三轮**18项全绿**。候选p2快路收敛1.3/2.9/2.9秒、全部回显p95 38.6–47.4ms/max62.4–90.1ms，零错/退役；p3最坏431.1/539.7/463.8ms；p6r p95 113.4–128.9ms，旧版330.4–333.0ms。旧版p2末轮26.6秒超限；旧固定Relay参照虽回显110.9–113.9ms但三轮期末路径断言红，不能称整项绿。见REPORT/RUNS。
9. 样本时间窗：42518379039为434Noq绿、iroh22pass/3fail，确定性证明初始估计变化/延迟读stats/旧包ACK被误判新鲜。4fa3f1b3修复：Noq发布采样包发送和ACK处理时刻，Reading窗口以真实处理时刻计龄，确认要求包在确认起点后发出；正常2个观测tick、故障离开1个不变，静默恢复也排除旧包。42519038936 **SUCCESS 434Noq/26iroh**（tree5b137c71，snapshot997e58ac，control781a907，证据ack-time-green-42519038936）。其中额外1项是静默覆盖；不能说4项都在旧源取红。当前真实网络尚待验证，不用先前动态结果覆盖4fa源码。

42513039282是首轮候选编译错误（path_stats引用/可变借用）；已改先收集backup ID再读状态。独立分类，不能记产品红。

## 当前必须先查询的活动任务

| Run | 任务 / 源码 | 最后观察 |
| --- | --- | --- |
| 42517915206 | netlab-hop，clean221b98fc/tree87f4555d，snapshotfd10c974，control781a907 | in_progress；f4/f4s/f4i独立顺序三轮，含path质量缓存诊断，无后加Noq样本时刻 |
| 42517959505 | 完整netlab，同tree87f4555d，snapshot6c42bd86，control781a907 | in_progress；35场景原8并行，无后加Noq样本时刻 |

没有其他旧在途任务。恢复先查仍在途任务，观察超时不重启。新增文件先stage，snapshot排除untracked。43个实验源已锚定到本地`refs/khor/net-evidence/<run>`，不依赖七天artifact保留；清单证据根source-refs.json。快照含source/子树，读取例：`git show refs/khor/net-evidence/42512359677:source/vendor/noq-proto/src/connection/mod.rs`。校验`git rev-parse REF:source`必须等于收据source_tree；不能把parentless快照直接覆盖工作树。

42517477095只报p2格式，42517864599和42518102550只报journal字符串格式，均未跑相应单元。patch已检查/应用；第二次与第三次是同一格式问题，不能算三项新鲜性产品红。证据分别speed-v2-unit、path-measurement-node、ack-time-format对应run目录。

## 仍需解决的证据与下一步

- **p2协议纠错**：原拓扑Relay与LAN都没有延迟注入，强制两端回LAN违背用户速度优先。旧13.0/16.4秒或90秒未回LAN不能单独证明实际请求慢，旧日志缺延迟分布不能回填新协议通过。ACCEPTANCE在v2首跑前冻结：至少一端实际走将变慢的second，10秒内两端进LAN/Relay快集合，包含收敛过程的全部回显p95<200ms、max<1秒、零错误/退役，再保持30次。新候选三轮已绿，旧结果全部保留。
- **真实黑洞恢复**：旧候选42509632238的p3为755ms/9.9秒/15.3秒（末轮1超时），这是真实体验回归。当前三轮已均<0.75秒、零错/退役；继续用相同fixture的正式A/B和长回归验证，不用p2纠错消除p3原失败。
- **转发**：共享API短矩阵f4i正确started后两次selected relay、回hop58.6秒；ACK校正短矩阵f4i最初180秒未通过m。最新42513843436为7pass/2fail：f4绿；f4i本次绿（22.4秒回m、无回退）；f4s回carried23.0秒，较腿恢复应答晚14.0秒，最长回显10.1秒，红；另一红是旧p2类型门。全部日志已下载backup-hop-diagnostic-42513843436。先前诊断只挂f4i，恰逢其绿，没有实际diagnostic行；当前fixture补到f4_stall失败，也读x/y/m四对最后50个hop/dial/settled/path/retired。仍需真实质量/探索/清理证据，不把Direct硬优先加回来。
- f4物理配置：LEG为每设备20ms；relay设备44ms，完整Relay约128ms而非44ms；两跳carried至少约80ms另有封装调度。25%门槛可能临界，必须读真实候选质量，不能只凭注释“hop20ms”要求类型胜出。保持原失败与门槛，未改拓扑。
- **稳定性**：旧采样/API前完整candidate42509652259已21pass/14fail（35项1968秒），p1/p1j/p1s抖动、转发/NAT/p2/p5等红；baseline full为23pass/8fail（31项，含2配对busy未进入测量）。最终候选仍需同条件长回归，不能把所有失败归为CPU，也不能用旧源结果证明新源。
- Reading时间窗缺口已确定性红→绿，但实际稳定性/CPU长尾仍需新源验证。不能把ACK处理时刻说成内核收包时刻，置信/波动与完整CPU影响仍缺。p7新增交互0/2/4秒、闲置0/5/10/14秒相位，各phase新建同拓扑，门槛仍10/20秒；变化前复查bothRelay，不能把未打中变化记绿。动态任务每轮9项（p2/p3+7相位）、3轮27项；完整新fixture40项，旧35项run不能称覆盖新增相位。下一步提交准确源后跑net-node、动态相位和短转发回归。
- N1达标后按PLAN推进N2期限/竞速/缓存/取消/随机退避/0-RTT、N3混合负载/能耗、N4故障域/中继方案、N5补丁退出/真实设备/最终Quality。REQUIREMENTS逐项审计保留。

本轮p2只纠正测试的体验定义、补实际回显测量；221b98fc只在已有path事件附只读测量缓存，不新增timer、不改显示句，journal旧行兼容/新行回读回归仍须net-node实际执行。格式patch已应用，准确当前源最终检查未完成。详细运行收据/失败分类在RUNS，阶段报告不代表最终完成。

外盘缺失，主仓只读git show feat/net:docs/net-optimization/STATE.md与PLAN恢复已提交计划，不覆盖原未提交内容。
