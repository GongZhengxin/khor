# feat/net 当前状态与恢复入口

2026-11-09 · owner=root，串行，无子智能体。

## 当前授权与位置

活动目标 **“完成计划”**，已授权按PLAN实施验证，取代最初仅规划状态。速度/体验优先；个人可信设备网。未合main、未发布、未采购/部署中继。本机source-only，无Cargo/Node/native重型工作；Linux用私库GitHub job，Mac沿macserver任务。

- 产品：`/Volumes/disk0/Khor/worktrees/current/khor-net`，branch `feat/net`。实现检查点9b80cb5d，计划/测试30335781、cbcc8abe；最新HEAD与暂存/未暂存内容以现场为准。
- 基线：main/rc8 `6485cb83d3b1f0bb93d4633825dc956623d78af9`。`khor-net-baseline` detached该提交；只改paths fixture、netlab.sh与dev.py，生产源码原样。
- 私库控制：`/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`，当前d7d1af1。控制main不动；产品feat/net尚未推origin。
- 证据根：`/Volumes/disk0/Khor/evidence/net/`。收据source.json、environment.json、measurement-mode.json及原日志绑定准确版本。
- 权威文档：[PLAN](PLAN.md)、[REQUIREMENTS](REQUIREMENTS.md)、[ACCEPTANCE](ACCEPTANCE.md)、[DESIGN](DESIGN.md)、[RUNS](RUNS.md)、[阶段报告](REPORT.md)、[下一设计检查点](NEXT-DESIGN.md)。初始过程保留HISTORY，非当前状态。
- GUI/治理任务独立；不改其worktree、现装App、原用户会话。图谱是不完整增量图，低风险/零flow不是无影响证明；先图工具，缺失后定向读，不擅自建立CodeGraph索引。

## 最高优先级：p2/p3恢复回归

正式3次顺序对照已结束。p6r旧3红、新3绿（候选p95 110–117ms，接近固定Relay参照）；但候选p2为13.0/16.4/6.9秒，p3为755ms/9.9秒/15.3秒并1次请求超时。旧版p2/p3三轮均绿。**当前N1未完成，先修此回归，不推进N2实现，不调宽门槛。** 完整数据见REPORT。

当前排查假设：raw ACK往返未扣对端ACK-delay，可能误读活动路质量/丢掉备用；另外，黑洞后速度优先可选同速Relay，历史RTT仍快的黑洞Backup可能未被resend_off_slow_backups重排，流数据继续等旧路。尚未定因，不把Direct硬优先加回来遮盖恢复问题。

## 最新代码与证据边界

1. 统一全承载速度排序、bind、改善门槛、迁移确认；hop发现/清理门槛统一。首版63pass，node hop9/wires10通过。
2. ACK样本计数/近2秒最多8观测中位数、有界STREAM活跃备用刷新：p7首次及三次顺序重复均绿，但闲置19.64–19.73秒已近20秒，尚缺相位/负载裕量。正式p2/p3重复红即来自这一版。
3. 共享quality/age/count输入：2项行为回归63pass/2fail已取红；随后PathState只读缓存/Path.measurement、PathFact与清理接线，**42511021381 SUCCESS（66pass/1ignored）**、**42511084014 SUCCESS（noq431/iroh路径22）**。读取不探测，缓存更新不通知paths_stream。质量/时间/样本量同一代表副本；age是actor观测时刻，非noq实际ACK接收时刻，CPU延迟读仍有缺口。
4. 最新工作区增加ACK-delay校正样本：RttEstimator.latest_sample保存原平滑算法已用的校正输入；原raw latest/PTO/拥塞数学不变，有可扣/不可扣ACK-delay测试。该变更在上述66/431/22快照之后，尚未验证。

## 先查询这些既有任务，不重复提交

| Run | 版本/任务 | 当前最后观察 |
| --- | --- | --- |
| 42509652259 | tree4549630f，API前完整matrix，control419b5f0 | in_progress；已有p2/p3回归，不算最终源码验收 |
| 42511301780 | treef8eba3f5，共享API、ACK校正前短矩阵，controld7d1af1 | in_progress；2并行，含f4/f4s/f4i |
| 42511866017 | tree05a5c757，最新ACK校正vendor，snapshot e3c172fa，controld7d1af1 | 新提交，先查/下载 |
| 42511895143 | 同tree05a5c757，最新ACK校正短矩阵，snapshot6e5479ac | 新提交，先查/下载 |

最近已结束/下载：baseline full42507224815（23pass/8fail，6测量红+2配对busy；另p2i报告红未assert）、baseline compare42509612220（每轮仅p6r红）、candidate compare42509632238（p6全绿，p2/p3回归）、动态重复42509803932全绿、共享API单元42511021381与42511084014全绿。42509569373因baseline漏同步线程开关主动取消，正式42509612220已补齐；不计产品失败。

## 下一步

1. 核对branch/HEAD/完整diff，保留当前代码；查询上表并下载收据/原日志，分类准备/编译/产品故障。新源码不能由旧在途结果代替。
2. 判断ACK校正是否改善p2/p3；仅一次绿不足，修复版本重跑正式3次。若仍红，构造noq同速黑洞Backup在飞流数据的确定性测试，结合实际路径/journal定位再修。
3. 继续N1防抖/CPU/转发、样本年龄/可信度与备用/相位开销；必要时补采样真实时刻，不把观测时刻冒充ACK发生时刻。
4. N1稳定后按PLAN推进N2期限/竞速/缓存/取消/随机退避与读写0-RTT，随后N3混合负载/能耗、N4故障域/中继方案、N5补丁退出/真实设备/最终Quality。REQUIREMENTS完整缺口保留。

当前所有阶段完成度：N0部分，N1实施且有产品红，N2–N5未完成。不得以单元绿、静态p6改善或若干阶段交付替代“完成计划”。

外盘缺失，在主仓只读 `git show feat/net:docs/net-optimization/STATE.md` 与PLAN恢复已提交内容，不覆盖外盘原未提交修改。
