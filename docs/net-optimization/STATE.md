# feat/net 当前状态与恢复入口

更新时间：2026-11-09 · 所有者 root，串行。

## 当前授权与位置

用户活动目标 **“完成计划”**，已授权按 [PLAN](PLAN.md) 实施与验证；此前仅规划状态已替代。速度/体验优先，个人可信设备网络。**未合 main、未发布、未采购或部署中继。**

- 产品：`/Volumes/disk0/Khor/worktrees/current/khor-net`，`feat/net`，产品实现检查点 `9b80cb5d`，测试/计划检查点 `30335781`；实际HEAD及后续文档提交以git log为准，先核对diff。
- 创建基线 main/rc8：`6485cb83d3b1f0bb93d4633825dc956623d78af9`；旧生产代码对照树 `khor-net-baseline` detached该提交，只改fixture、netlab顺序执行脚本和远端入口。
- 私库控制器：`/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`。网络Linux job独立，Mac任务沿既有macserver入口；本机source-only，不做Cargo/Node/native重型工作。
- 证据：`/Volumes/disk0/Khor/evidence/net/`；[运行账本](RUNS.md)、[逐项完成审计](REQUIREMENTS.md)、[验收协议](ACCEPTANCE.md)、[设计](DESIGN.md)。初期过程移至 [历史](HISTORY-20261109-initial.md)。
- 其他GUI/治理任务独立，不修改它们的worktree、现装App或用户既有会话。

## 已确认事实

1. 旧策略速度回归红；首版全承载速度优先、统一bind和迁移确认的单元62通过、1 ignored。
2. 修正反向wire准备后的真实p6：旧策略快中继场景 p95 369.78ms，候选152.12ms并选Relay；反向快Direct两版均绿。仅一次A/B，尚不能完成N0/N1。
3. hop清理原固定10ms与选路25%/5ms矛盾，已统一改善规则，并允许慢Direct参与hop发现；节点hop9/wires10通过，对应tree067d62b5。
4. p7动态取得产品红：Direct300ms恢复为LAN后，交互和闲置都45秒未回，目标分别10/20秒；无请求错误。
5. 当前正在修复ACK样本计数/新鲜度与近期质量估计，并加入STREAM活跃期每对端最多5秒一次的既有备用探测。拥塞控制平滑RTT不变，闲置不新增计时器。单元63通过/1 ignored；首次p7交互2.64秒、闲置18.16秒、零错误，目标10/20秒均绿。需重复与防抖验证。
6. vendor测试入口第一次因excluded workspace的dev-dependencies失败，是执行器准备问题。改独立manifest与对应lock；iroh lock仅将noq-proto指向既有本地补丁，版本不变。独立manifest/committed lock实跑SUCCESS：noq431、iroh路径21通过，日志fresh-vendor-42509174050。

## 活动任务与下一步

- **完整baseline 42507224815**：最后观察仍在network执行；先查询实时状态，下载完整证据；不因工具超时另提交。
- 42508069745/42508095178 已结束并下载，见RUNS。较早两次node任务均已下载确认仅格式失败，patch已应用。
- 最新采样/hop/p7已提交9b80cb5d，新增文档均跟踪。当前差异是固定好路径参照与compare入口。源码快照排除untracked。
- 同一准确源码tree `064925806d1c3962f715942d7f637a4b6257783f` 已提交：net-unit **42509122554**、netlab-dynamic **42509155432**、net-vendor **42509174050**；control `16474c5`。恢复unit与dynamic已成功并下载，vendor已成功；编译/准备失败先分类，产品失败保留原门槛。
- 正式三次顺序A/B在途：baseline **42509612220**（tree f95f6e54）、candidate **42509632238**（tree4549630f）；candidate完整矩阵 **42509652259**（同tree4549630f）。control419b5f0。
- 首次baseline compare42509569373因漏同步并行度开关已主动取消，是准备输入错误，不算产品结果。正式baseline已补相同netlab.sh；生产代码仍为6485cb83。
- 三次顺序动态重复 **42509803932** 在途，tree90757944（base30335781，产品与9b80相同，新增参照/文档），control9d915b7。候选完整矩阵检验防抖/尖峰/转发。
- N1下一个已知输入不一致与N2建连/期限审计见 [NEXT-DESIGN](NEXT-DESIGN.md)。先拿在途证据，再取新回归红、统一quality/age/count缓存。

## 阶段状态

| 阶段 | 当前状态 | 未完成关键项 |
| --- | --- | --- |
| P0 | 计划/恢复入口已建立 | 持续维护 |
| N0 | 部分 | 完整baseline、固定好路、重复、其他工作负载指标 |
| N1 | 首版静态绿，动态修复中 | 样本新鲜度/置信、p7、全承载防抖/转发与重复 |
| N2 | 未完成 | 竞速/缓存失效/总预算/取消/退避与写语义完整核对 |
| N3 | 未完成 | 混合负载、背压/公平、共享vs分连接、闲置能耗 |
| N4 | 未完成 | 双端不对称/故障域/发现/双家与中继容量运维设计 |
| N5 | 未完成 | 补丁覆盖/退出、短全长矩阵、Mac/真手机、多机、最终Quality |

逐项缺口以REQUIREMENTS为准。不得把首批选路通过、某次编译绿或单一网络场景绿称为整体完成。

## 跨 compact/session 接续

读本文件、PLAN、REQUIREMENTS及最新用户指令；核对绝对路径/branch/HEAD/完整diff和既有任务状态。证据仅对source.json绑定的tree有效；不要用摘要覆盖未提交内容，不重复提交仍活跃任务。本树图谱为不完整增量图，不能由低风险/零flow推断无影响；已先使用图工具，缺失范围定向读源码，不擅自建立CodeGraph索引。

外盘缺失，在主仓只读 `git show feat/net:docs/net-optimization/STATE.md` 与PLAN恢复已提交计划；保留原未提交内容。
