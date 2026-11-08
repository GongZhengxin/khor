# feat/net 当前状态与恢复入口

2026-11-09 · owner=root，串行，无子智能体。

## 授权、位置与完整范围

活动目标“完成计划”已授权PLAN实施/验证，取代最初仅规划状态。速度/体验优先，个人可信设备网。未合main、未发布、未采购/部署中继。本机source-only；Linux用私库GitHub job，Mac沿macserver。其他GUI/治理worktree、现装App和用户原会话保持独立。

- 产品 `/Volumes/disk0/Khor/worktrees/current/khor-net`，`feat/net`；真实样本时间窗4fa3f1b3、p7相位6a45e488、journal数字回读6b3644f5、恢复/稳定性独立模式与路径诊断e480dc2e、窗口质量同步过期4ec6ab2e。更早ACK/Backup修复及p2协议仍保留。恢复先核git HEAD/diff，不能按摘要覆盖。
- 旧生产基线main/rc8 `6485cb83d3b1f0bb93d4633825dc956623d78af9`。`khor-net-baseline` detached该提交，只改fixture与远端测试脚本，生产源码原样。
- 控制 `/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`，最新fdd1b3f；此前0e5788c补核心单元，fdd1b3f新增独立netlab-recovery（三轮p2/p3）、netlab-stability（p1/p1j/p1s/p5单轮），full也显式写mode8并行。两个main不动，产品feature尚未推origin。
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
9. 样本时间窗：42518379039为434Noq绿、iroh22pass/3fail，确定性证明初始估计变化/延迟读stats/旧包ACK被误判新鲜。4fa3f1b3修复：Noq发布采样包发送和ACK处理时刻，Reading窗口以真实处理时刻计龄，确认要求包在确认起点后发出；正常2个观测tick、故障离开1个不变，静默恢复也排除旧包。42519038936 **SUCCESS 434Noq/26iroh**（tree5b137c71，snapshot997e58ac，control781a907，证据ack-time-green-42519038936）。其中额外1项是静默覆盖；不能说4项都在旧源取红。此后网络短矩阵见第11项，不用先前动态结果覆盖4fa源码。
10. 221b98fc的独立转发三轮42517915206 **SUCCESS 9/9**（hop-isolated-42517915206）：f4应答后0.1/0.1/0.6秒回carried；f4i回来后24.8/19.0/31.2秒通过m且无回退；f4s三轮绿。该源早于时间窗修复。
11. 时间窗修复版6a45e488短矩阵42519803507 **SUCCESS 9/9**（ack-time-smoke-42519803507）：p2 1.0秒/max45.7ms；p3最坏609ms、零错/退役；f4应答后0.9秒、f4s1.0秒回carried，无回退；f4i9.3秒回m；p6四项绿。2场景并行，仅一次，不能替代正式三轮或长稳定性。
12. 42519750741第一方格式通过，核心66pass/1ignored、hop9、wires10；journal27pass/1fail暴露新诊断的真实兼容红：serde内部tag缓冲不支持直接读u128。6b3644f5保留PathFact算术类型，按JSON的u64毫秒数字读取并扩展到真实Entry持久化回读；尚待42521515341验证。42520842668仅新assert格式失败，patch已与当前源反向校验一致，未跑单元。Netlab的journal助手直接读原始JSON Value，不经过Entry解码，这两类日志链路不同，原始实验数据不因该兼容错误消失。
13. 时间窗版三轮相位42519778236 **26pass/1fail**（9/0、9/0、8/1）：21个p7观察全部绿，交互2.26–2.60秒、闲置4.39–18.56秒；p3最坏703.7/573.3/458.6ms均零错/退役。p2为2.1/3.0/16.2秒，末轮超10秒；末轮max63.4ms、零错/退役，Alpha LAN@16s/Beta LAN@1s。目标保留，不把选路超限说成16秒请求卡顿。证据ack-time-phases-42519778236。
14. 42521515341仅六方向诊断函数签名格式失败，未跑单元；patch已检查/应用。不能当作u128修正仍失败或已通过。新的p2测量末尾始终输出两端路径质量事件，p1失败及双路Direct准备失败也输出；只改测试诊断，不改选路/网络/门槛。source-only轻量产品9项、control1项测试通过，Rust须远端复验。
15. e480dc2e第一方 **42524575725 SUCCESS：核心66pass/1ignored、hop9、wires10、journal28**，含实际Entry持久化回读；证据journal-number-node-42524575725。独立恢复 **42524600966 SUCCESS 6/6**：p2 750.2/727.6/753.1ms，max32.65/31.53/31.88ms；p3最坏640.6/705.8/578.4ms，零错/退役，证据recovery-diagnostic-42524600966。此前16.2秒超限不删除；新模式不同VM，不能用三绿宣布消除偶发。
16. 窗口过期的派生值不一致：42525350514确定性红（434Noq绿、iroh26pass/1fail），窗口只剩1ms样本但质量仍300ms。4ec6ab2e复用record的中位数计算，过期缩窗时重算，清空则保留最后已知值；时间/ACK计数/确认历史不刷新，不新增timer/探测调用。**42526508917 SUCCESS 434Noq/27iroh**，证据quality-expiry-green-42526508917。尚未证明这是p2/p7实际超限定因，真实网络验证见活动表。

42513039282是首轮候选编译错误（path_stats引用/可变借用）；已改先收集backup ID再读状态。独立分类，不能记产品红。

## 当前必须先查询的活动任务

| Run | 任务 / 源码 | 最后观察 |
| --- | --- | --- |
| 42527295249 | netlab-dynamic，4ec6ab2e/tree6f9a77e3，snapshotdbcf6d94，controlfdd1b3f | in_progress；过期修复后p2/p3+7相位，单场景顺序3轮27项 |
| 42527316922 | netlab-smoke，同tree6f9a77e3，snapshote600c180，controlfdd1b3f | in_progress；过期修复后9项、2场景并行 |

只查询上述两项，不重复提交旧任务。新增文件先stage，snapshot排除untracked。实验源已锚定到本地`refs/khor/net-evidence/<run>`，不依赖七天artifact保留；数量及清单以证据根source-refs.json为准。快照含source/子树，读取例：`git show refs/khor/net-evidence/42512359677:source/vendor/noq-proto/src/connection/mod.rs`。校验`git rev-parse REF:source`必须等于收据source_tree；不能把parentless快照直接覆盖工作树。

42517477095只报p2格式，42517864599和42518102550只报journal字符串格式，均未跑相应单元。patch已检查/应用；第二次与第三次是同一格式问题，不能算三项新鲜性产品红。证据分别speed-v2-unit、path-measurement-node、ack-time-format对应run目录。

## 仍需解决的证据与下一步

- **p2协议纠错**：原拓扑Relay与LAN都没有延迟注入，强制两端回LAN违背用户速度优先。旧13.0/16.4秒或90秒未回LAN不能单独证明实际请求慢，旧日志缺延迟分布不能回填新协议通过。ACCEPTANCE在v2首跑前冻结：至少一端实际走将变慢的second，10秒内两端进LAN/Relay快集合，包含收敛过程的全部回显p95<200ms、max<1秒、零错误/退役，再保持30次。新候选三轮已绿，旧结果全部保留。
- 当前时间窗版同协议又有p2一次16.2秒超限。优先读新诊断的真实质量及确认/迟滞输入，尤其一分钟内10ms绝对收益底线是否拒绝持续5–10ms改善；Alpha连接年龄69秒不是selected_at，不能只凭年龄定因。现未改阈值或确认。独立恢复3轮与独立稳定性结果要一起看，不能修恢复又放大抖动。
- **真实黑洞恢复**：旧候选42509632238的p3为755ms/9.9秒/15.3秒（末轮1超时），这是真实体验回归。当前三轮已均<0.75秒、零错/退役；继续用相同fixture的正式A/B和长回归验证，不用p2纠错消除p3原失败。
- **转发**：共享API短矩阵f4i正确started后两次selected relay、回hop58.6秒；ACK校正短矩阵f4i最初180秒未通过m。最新42513843436为7pass/2fail：f4绿；f4i本次绿（22.4秒回m、无回退）；f4s回carried23.0秒，较腿恢复应答晚14.0秒，最长回显10.1秒，红；另一红是旧p2类型门。全部日志已下载backup-hop-diagnostic-42513843436。先前诊断只挂f4i，恰逢其绿，没有实际diagnostic行；当前fixture补到f4_stall失败，也读x/y/m四对最后50个hop/dial/settled/path/retired。仍需真实质量/探索/清理证据，不把Direct硬优先加回来。
- f4物理配置：LEG为每设备20ms；relay设备44ms，完整Relay约128ms而非44ms；两跳carried至少约80ms另有封装调度。25%门槛可能临界，必须读真实候选质量，不能只凭注释“hop20ms”要求类型胜出。保持原失败与门槛，未改拓扑。
- **稳定性**：旧采样/API前完整candidate42509652259已21pass/14fail（35项1968秒），p1/p1j/p1s抖动、转发/NAT/p2/p5等红；baseline full为23pass/8fail（31项，含2配对busy未进入测量）。最终候选仍需同条件长回归，不能把所有失败归为CPU，也不能用旧源结果证明新源。
- ACK/Backup修复后、真实时间窗前的full42517959505已**FAILURE 23pass/12fail、35项1864.48秒**（ack-backup-full-42517959505）。4项准备未达：f4/f4s/f4i180秒未经m、p1j beta未Direct。8项进入测量后原断言红：f3未声明no_relay、d2/d4无Direct selected、p1移动5/3、p1s7/9、p2快集合13.5秒、p5短卡顿期间切换/打洞、固定Direct参照p95 1.56秒/max2.63秒。p2i15.7秒报告red但Rust未assert，不计为达标。d2/d4“没选Direct”尚不足证明打洞未打开，需审计可达性与类型门。full8并行与独立转发/性能比较不可混算。
- full的18行f4准备失败诊断只含x/y、只有Relay/拨号/settled，未记录hop或Carried，缺M腿/道路书事实，不能据此确定未探索或CPU根因。6b3644f5把三个失败出口都改为x/y/m六个方向，新的40项矩阵在途。下一步可补p1/p1s失败质量日志与独立稳定性/CPU场景，先区分全矩阵交叉负载与单场景，不直接扩滞回掩盖问题。
- 时间窗版full42521539154已**FAILURE 28pass/12fail、40项2001秒8并行**，证据ack-time-full-42521539154；早于窗口质量过期修复。p1移动5/11、p1s5/5；p1j、f4前置未达；f4s应答后0.4秒回carried，但30.5秒后又Relay且期末Relay；p3最坏1.1秒零错/退役；p6r p95 223.21ms；p7i20.054秒；f3/d2/d4/p5仍红。其余6个p7相位绿，p2本次绿，p2i报告red79.8秒但未assert。不能当作4ec修复后完整验证，也不能将这些全归CPU。独立稳定性结果见下段；先结合新保存的质量/样本/年龄判断，再改策略。
- 独立稳定性42524625685已FAILURE 2pass/2fail、1880.44秒：p1空闲600秒0/0移动，p5三组短停顿零Relay切换/零打洞；p1j在Direct准备前置失败，未测稳定性；p1s含自身忙CPU/20次300ms停顿，600秒移动14/2、零退役、最慢回显658.8ms。早于4ec过期修复，但说明抖动不能全归full交叉负载。p1s若干selected事件快照质量接近/反转，快照非决策瞬间，先复现确认流程，不直接调门槛。证据stability-isolated-42524625685。
- Reading时间窗缺口已确定性红→绿，21相位/三次p3绿，但p2又一红；长稳定性/CPU长尾仍需在途结果。ACK处理不是内核收包时刻，置信/波动未齐。p7各phase新建同拓扑，门10/20秒不改。新完整fixture40项，旧35项不覆盖新增相位。journal解码修正不改网络选择；继续定向必要修改，不能以短绿完成N1。
- N1达标后按PLAN推进N2期限/竞速/缓存/取消/随机退避/0-RTT、N3混合负载/能耗、N4故障域/中继方案、N5补丁退出/真实设备/最终Quality。REQUIREMENTS逐项审计保留。

本轮p2只纠正测试的体验定义、补实际回显测量；Entry兼容已红→绿，最新过期修复434/27绿但实际网络在途。f4最长回显含人为3/8秒断腿及超慢备用，不能套p3“已有快备用时<1秒”的目标；其原恢复后1.5秒/无回退门保持。详细收据/失败分类在RUNS，阶段报告不代表最终完成。

外盘缺失，主仓只读git show feat/net:docs/net-optimization/STATE.md与PLAN恢复已提交计划，不覆盖原未提交内容。
