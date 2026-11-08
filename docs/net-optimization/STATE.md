# feat/net 当前状态与恢复入口

2026-11-09 · owner=root，串行，无子智能体。

## 授权、位置与完整范围

活动目标“完成计划”已授权PLAN实施/验证，取代最初仅规划状态。速度/体验优先，个人可信设备网。未合main、未发布、未采购/部署中继。本机source-only；Linux用私库GitHub job，Mac沿macserver。其他GUI/治理worktree、现装App和用户原会话保持独立。

- 产品 `/Volumes/disk0/Khor/worktrees/current/khor-net`，`feat/net`；ACK/共享质量基础cba1ea80；后续Noq重排/测试/诊断检查点以git log获取最新HEAD，先核实际diff，不能按摘要覆盖。
- 旧生产基线main/rc8 `6485cb83d3b1f0bb93d4633825dc956623d78af9`。`khor-net-baseline` detached该提交，只改fixture与远端测试脚本，生产源码原样。
- 控制 `/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`，当前c6a20c5；两个main不动，产品feature尚未推origin。
- 证据 `/Volumes/disk0/Khor/evidence/net/`；source/environment/mode与原日志是准确版本证据。
- 权威索引：PLAN、REQUIREMENTS、ACCEPTANCE、DESIGN、RUNS、REPORT、NEXT-DESIGN。历史不是当前状态。图谱是部分增量图；先图工具，缺失后定向读，不擅自建立CodeGraph索引。

N0部分；N1仍有产品红；N2–N5未完成。完整范围保留，不能以单元或p6绿替代整计划。

## 已完成的修复及适用证据

1. 全承载速度排序/统一bind/门槛/迁移确认，hop发现与清理门槛统一。首版63pass，node hop9/wires10通过。
2. ACK样本计数、近2秒最多8观测中位数、有界STREAM活跃备用刷新。p7原45秒不回→首次绿；3次交互2.53–2.75秒、闲置19.64–19.73秒（接近20秒，尚缺相位/负载裕量）。但这一版正式p2/p3重复恢复回归，不能收口。
3. 共享quality/age/count输入：先63pass/2fail取红，再66pass/1ignored（42511021381）、431Noq/22iroh（42511084014）绿。缓存读取不探测/不唤醒paths_stream；质量与年龄来自同一副本。age仍是actor观测时刻，非实际ACK接收时刻。
4. ACK-delay校正：latest_sample与原平滑器用同一校正输入，原raw latest/PTO/拥塞数学不改。42511866017为432/22全绿；短矩阵42511895143为8pass/1fail，仅f4i初始未通过m前置红，p2 973ms、p3最坏696ms且零错/退役、f4/f4s/p6均绿。仅一次，不替代正式重复。
5. 同速黑洞在飞数据：42512359677为432pass/1fail，旧快RTT的黑洞Backup在移路时不释放数据。当前Noq补丁9/iroh30将原不应答判据移到PathStats::unanswered_for，迁移时坏Backup也重排；健康同速不重排，MTU排除/原丢包与拥塞计量/QUIC去重保留，不重放应用请求。**42513546687 SUCCESS：433Noq/22iroh全绿**，tree6465d105，controlc6a20c5，证据blackhole-backup-green-42513546687。新增用例含真实虚拟PTO与流交付，非只改RTT模拟故障。

42513039282是首轮候选编译错误（path_stats引用/可变借用）；已改先收集backup ID再读状态。独立分类，不能记产品红。

## 当前必须先查询的活动任务

| Run | 任务 / 源码 | 最后观察 |
| --- | --- | --- |
| 42513580645 | netlab-dynamic，tree6465d105，snapshote67ddf7b，controlc6a20c5 | in_progress；p2/p3/p7单场景顺序3轮 |
| 42513843436 | netlab-smoke，tree6e61d014，snapshot66dac4d7，controlc6a20c5 | in_progress；同生产代码，另加f4失败journal尾部诊断；2场景并行 |
| 42514304126 | net-unit，treecb25e125，snapshotbc56f16f，controlc6a20c5 | 仅f4诊断matches格式失败，未跑单元；formatter patch已应用，接下来复验 |

没有其他旧在途任务。恢复先查这三个，观察超时不重启。新增文件先stage，snapshot排除untracked。

## 仍需解决的证据与下一步

- **恢复**：正式旧候选42509632238的p2为13.0/16.4/6.9秒、p3为755ms/9.9秒/15.3秒（末轮1超时）；旧版3轮均绿。先用当前动态3轮证明修复，目标10秒/1秒与零错误/退役不放宽。
- **转发**：共享API短矩阵f4i先started正确但后又selected relay两次、回hop58.6秒；ACK校正短矩阵f4i在最初180秒就未通过m。都未解决，不能统称首事件误记。新诊断打印[netlab-path-diagnostic]两端最后50个hop/dial/settled/path/retired，先区分没探索、实际收益不足、跨连接清理或再次切换，不把Direct硬优先加回来。
- f4物理配置：LEG为每设备20ms；relay设备44ms，完整Relay约128ms而非44ms；两跳carried至少约80ms另有封装调度。25%门槛可能临界，必须读真实候选质量，不能只凭注释“hop20ms”要求类型胜出。保持原失败与门槛，未改拓扑。
- **稳定性**：旧采样/API前完整candidate42509652259已21pass/14fail（35项1968秒），p1/p1j/p1s抖动、转发/NAT/p2/p5等红；baseline full为23pass/8fail（31项，含2配对busy未进入测量）。最终候选仍需同条件长回归，不能把所有失败归为CPU，也不能用旧源结果证明新源。
- 当前确认样本只按actor观测计数；真实ACK发送/接收时间、延迟读stats、积压ACK、置信/波动仍缺。若稳定性仍红，优先验证确认是否误用故障前发出的包及far_faster捷径，别仅靠加大固定阈值压住。
- N1达标后按PLAN推进N2期限/竞速/缓存/取消/随机退避/0-RTT、N3混合负载/能耗、N4故障域/中继方案、N5补丁退出/真实设备/最终Quality。REQUIREMENTS逐项审计保留。

本轮新增f4诊断没有网络行为变化；第一方格式的远端patch已应用，正在复验；最终准确源码检查仍需完成。详细运行收据/失败分类在RUNS，阶段报告不代表最终完成。

外盘缺失，主仓只读git show feat/net:docs/net-optimization/STATE.md与PLAN恢复已提交计划，不覆盖原未提交内容。
