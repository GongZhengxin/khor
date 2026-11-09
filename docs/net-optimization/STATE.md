# feat/net 当前状态与恢复入口

2026-11-09 · owner=root，串行，无子智能体。仅本文件维护当前任务；[PLAN](PLAN.md)维护完整范围，
[RUNS](RUNS.md)保留准确源/原始结果。旧STATE原样归档到[历史](HISTORY-20261109-before-diagnostic-results.md)，其中“在途”为历史。

## 授权与位置

活动目标“完成计划”已授权N0–N5实施/验证，取代最初仅规划状态。速度/体验优先，个人可信设备网。
没有合main、发布、中继采购/部署授权。root串行；不派子智能体。

| 用途 | 位置 / 分支 |
| --- | --- |
| 产品 | `/Volumes/disk0/Khor/worktrees/current/khor-net`，`feat/net` |
| 控制 | `/Volumes/disk0/Khor/worktrees/current/khor-build-net`，`feat/net`，322835b已push |
| 基线 | `/Volumes/disk0/Khor/worktrees/current/khor-net-baseline`，detached 6485cb83d3b1f0bb93d4633825dc956623d78af9；仅paths/hop/dev.py/netlab.sh四文件fixture修改，生产原样 |
| 证据 | `/Volumes/disk0/Khor/evidence/net/` |

本机source-only：禁止本机Cargo/Rust/Node/Xcode构建、测试、格式器与下载可执行物运行。
小型Python/脚本检查可用；Linux重型用私库khor-build GitHub network job，Mac沿既有macserver lane。
不触碰其他GUI/治理worktree、现装App或用户原会话。新目录/证据放`/Volumes/disk0/Khor`分类。
图谱部分覆盖，先图工具，缺失后定向读；本树没有CodeGraph索引，不擅自建立。

## 当前唯一在途任务：先查询，不重复提交

| Run | 任务与准确源 | 最后观察 |
| --- | --- | --- |
| 42536677230 | netlab-diagnostic；basedf5b0864 dirty；tree1113605daf5597c2663d5e6468c45dade34031c9；snapshot72b0f7e13c0cb7142db033434cbd8182a1ba610a；controlc3b117d | prepare成功，Linux network运行，Mac跳过；原f4s/p1s各一次、1线程 |

诊断代码5f69db3f：CLI可选net-diagnostics加显式环境开关才安装subscriber，使用已锁tracing-subscriber0.3.23。
记录ACK raw/delay/corrected、采样包时间、迁移初始/Go的Reading与Silence、恢复hold起止。
测量后保存各节点原serve.log，默认构建不开日志。**日志模式不作为SLO成绩**，原600秒/20次300ms/忙CPU及f4s断腿/1500ms门不变。
可选CLI feature实际编译/日志非空由此run确认；Noq/Iroh单元绿不替代它。

## 当前阶段判断

N0部分、N1仍红、N2–N4未完成、N5部分。完整范围和逐项缺口见[REQUIREMENTS](REQUIREMENTS.md)，不能以单元或p6绿完成整计划。

最新Watch生产代码519ad4aa，后移路Backup修复ff399658；5f69db3f只加可选诊断。
后续提交主要是文档/验收覆盖，恢复时仍核实际HEAD/branch/diff，不按本页覆盖工作区。

| 最新普通模式证据 | 结果与限制 |
| --- | --- |
| 42532849927，Watch动态三轮 | SUCCESS 27/27；p2 1.1秒/831.8ms/6.2秒，max50.38/52.60/75.08ms；p3 626.1/655.5/508.1ms，零错/退役；21相位绿，闲置phase0 18.42/19.53/19.20秒，20秒裕量仍小 |
| 42532770351，Watch p1s/p5 | FAILURE 1pass/1fail；p1s原600秒、20次300ms、忙CPU，alpha移动3次、beta1次，零打洞/退役，最慢671.8ms；p5 150/300ms原门绿。CPU稳定性仍红 |
| 42532877270，Watch转发三轮 | FAILURE 8/9；f4恢复0.8/0.0/0.3秒；f4i34.3/15.1/12.6秒；f4s0.0/**1513ms**/0.2秒，中轮超1500ms；均无回退。饱和0不代表真实零耗时 |
| 42531361022 vs 42531408799，相同准备v2 | ff候选2pass/2fail，baseline648为3pass/1fail；两者p1/p1j移动0/0，p1s均7/0，ff打洞更多，p5红而baseline绿。不能声称候选全面胜出；早于Watch修正 |
| 42531477192，ff转发三轮 | FAILURE 8/9；f4s中轮1.7秒超1.5秒；旧红保留 |

上述源和artifact目录见RUNS。不同源/VM/任务组合不计算“14降3”的因果改善比例。
旧15秒黑洞请求超时42527295249、旧full40为28pass/12fail的42521539154等历史失败继续保留，
不以最新三绿证明它们全部定因或最终源全矩阵达标。

## 已有修复与边界：不要重复造机制

- 全承载RoadOrder按实测速度比较；Direct没有硬优先。收益/确认统一，hop发现与清理共用门槛。
- Noq最新ACK样本计数含等值；选路样本使用与原平滑器相同的ACK-delay校正，原raw RTT/PTO/拥塞数学保留。
- Reading近2秒最多8个观测tick中位数，年龄来自采样ACK时间；初始估计不是ACK，读stats不刷新年龄。
  窗口缩小同步重算质量，空窗保留最后已知值；没有额外timer。ACK时间是协议接收路径时间，不是内核时间。
- 同一代表副本的quality/age/count共享缓存，读取不发包/不通知paths_stream；冷/活副本偏差与CPU污染仍待解。
- Moving快分支也需决定后两个新观测；Hold时复测当前路。故障leave仍独立，无应用请求重放。
- Watch普通慢质量需原2秒窗和决定后两个观测才打洞；探测预算不增。真实无应答1秒、RTT超2秒探索及30秒节流保留。
- Noq坏/明显慢Backup重排在飞可重传帧，健康同速不重排、MTU排除/原loss与CC/QUIC去重保留。
  既有Backup LossDetection也重检，覆盖先移路后坏，未新增timer/逐发送扫描。
- journal事件measurement是回调时缓存，**不是实际决策输入**；u128 JSON回读兼容已有journal28绿。
- 稳定性准备v2先冻结再实现：Relay设备100ms往返仅引导Direct，清除后原30秒settle及600秒测量，观察期不限制类型。

## 最新单元与补丁审计

- **42536711138 SUCCESS**：根格式、435Noq/33iroh；tree1113605d、snapshot71a5b250ed41979a7e8d66d01991b06254b813d3、controlc3b117d。
  `decision-diagnostics-unit-42536711138`已下载，5f69db3f全部vendor/CLI/脚本与快照无diff。
- **42537392595 SUCCESS**：根格式、435Noq/33iroh/**9项选家/QAD/网桥**；based5e926bf clean，
  treed7f8f38cdeaabefc398f5580a747c7e599209c49、snapshota256f6708e9ad3920c1d01561b785e6e1425d1bd、control322835b。
  `vendor-home-coverage-42537392595`已下载。不是iroh全部158项；旧33过滤器遗漏的9项现在才取得当前源实证。
- 产品9项Python、control准备1项与sh-n/bash-n/diff-check通过。本机不运行Rust格式器。
- [VENDOR-AUDIT](VENDOR-AUDIT.md)逐项盘点45条目标/覆盖/退出；Noq1/4/5/6专属回归、上游逐函数等价、原生代理/真机/能耗仍缺。
  固定发布源码及PR事实见[UPSTREAM](UPSTREAM.md)，不升级依赖、不按一个补丁退出删除整树。

## 接下来按顺序做

1. 查询42536677230终态并下载完整证据。若失败先区分构建/日志缺失/准备/原产品断言；不缩短或放宽场景。
2. 对p1s每个Go/当前路关闭事件，关联实际Reading和ACK raw/delay/corrected；对f4s关联静默判定、决定后应答、hold解除和Go。
   现有缓存快照提示候选质量可能残留故障值，但尚未证明根因；不要凭快照调gain或加固定等待。
3. 由确定性行为回归先取红，再最小修复。保留速度优先、故障快通道、健康备用/探测预算和不重放应用操作。
4. 关闭日志后重跑相应普通模式三轮、稳定性与最终准确源完整矩阵，不能用诊断模式收N1。
5. N1稳定后继续N2–N5：总期限/取消/竞速/缓存/随机退避与0-RTT，混合负载公平/能耗，中继与登记故障域/容量方案，Mac/手机/多机及最终可信Quality。

N2七类预算/取消/写入行为已在ACCEPTANCE实现前冻结；静态发现锁/hop不受总by约束、普通首拨未传by、早数据回答交出后失去Aborts。
尚无N2代码或行为红。NAT helper只看selected Direct，需先核opened/有效样本，不能把d2/d4未选Direct直接当穿透失败或回填通过。
具体审计入口见[NEXT-DESIGN](NEXT-DESIGN.md)。

## 源码证据与恢复规范

现78个本地immutable `refs/khor/net-evidence/<run>`，清单`evidence/net/source-refs.json`。
重要快照不依赖七天artifact保留；`git rev-parse REF:source`须等于收据tree。
快照是parentless容器，读取用`git show REF:source/<path>`，不得直接覆盖工作区，也不push这些本地ref。
收据终态及时刷新pending；目前只42536677230未完成。

恢复先核本目录、branch/HEAD/diff与已有run，保留未提交内容。两main及产品feature未合并发布。
外盘缺失时，从共同仓只读`git show feat/net:docs/net-optimization/STATE.md`与PLAN恢复已提交文档；不覆盖原外盘未提交内容。
治理工作若需接续，遵守中央code-quality STATE及外盘中断RESUME入口，不把本网络任务冒充治理任务。
