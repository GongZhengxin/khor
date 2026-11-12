# 空闲恢复后的首次操作与成本

本协议在新增首次操作数据前保存。旧p7及其active10秒/idle20秒选路门保持；
它们量两端重新选路的时间，不代表用户空闲后首次读取的响应。
源码备用keepalive15秒、STREAM活跃刷新最多每peer5秒一次，与旧idle16–17秒量级相符，
但不能独立证明旧尾部的唯一原因，也不预选缩短生产计时器的方案。

## 所有权准入

`LabNode.with`当前直接await设备JoinHandle。调用者future被丢弃时，内部设备任务可能继续执行；
这是需要真实验证的fixture所有权假设，尚不能凭静态代码把它算作已重现或已修复。

先添加固定filter `paths::idle_action::p8g_`的两个ignored诊断，必须显式`--ignored`执行：

1. 实际LabNode.with启动一个受控pending future，收到started后丢弃调用者；
   在1秒fixture界内必须观察到future Drop。旧实现即使红，也先释放受控等待并收尾后再断言。
2. 原始Device.spawn句柄被丢弃的负控必须能被检测；保留独立abort控制并回收，
   防止一个失灵的观测器把“没观察到任务”解释为自动取消。

两项各用原speed_pair的真实resident/设备运行时，5秒startup/cleanup只是fixture存活界；
测试线程1、诊断关闭、原shared guard/namespace/资源门不变。先取得准确源码的真实红和负控，
再修正fixture所有权，以完全相同两个测试复验。生产Noq/iroh/Node策略和旧场景参数不因此改变。
这只证明测试调用者对设备future的所有权；服务端任务、RPC取消、独立host/child与真机仍另验。

## 随后的首次操作测量

所有权准入通过后才实现并执行新测量。沿用speed_pair：慢Direct300ms、可用Relay约80ms，
两端准备阶段确认使用Relay，STREAM静默至少10秒，然后将原Direct恢复至0ms。
恢复后等待名义0/5/14秒，每方向各一个独立fresh，共六场景；不能先从另一端发业务预热。
首次前台目录读取前不查询Node状态或发其他业务；原echo的30秒caller与全部结果记录保留，
这是测量存活界，不是新UX达标承诺。记录实际单调时间、首请求服务/总时间、错误/超时与前后路径事实。

分开idle和首次操作的wire总字节/包区间；接口只读快照自身的时间与命令失败也须记录。
不能把网络总量当probe-only、把CPU当唤醒或把服务器结果当手机能耗。
观察工具/设备任务须由调用者拥有；计时前不得新增无主任务或预热动作。
需要原始配置/计数单调性、收据、进程组空、全部源/报告hash和source-owned复算；
缺测和失败保留，不按好看的首请求筛样本。

prefetch9组已按原冻结源/control全部结束，guard随后单独执行，未混入计时。
Linux实际format/编译成功、主所有权断言红、detached负控绿；旧红原件见STATE。
调用者持有的AbortHandle Drop修复已以逐字相同guard复验绿；旧红与新绿的控制入口相同。
实际run/source/proof见STATE；这仍只准入fixture所有权，零新增首次操作数据。
完整N0–N5、严重/quiet/跨连接质量、peer relay与移动、最终UX与真实手机能耗范围保持。


## 首批六场景的准确执行协议（首个数据前）

固定filter `paths::first_action::p8_`，显式ignored、线程1、诊断off；按Rust串行名称顺序
alpha0、alpha14、alpha5、beta0、beta14、beta5，每个均独立fresh speed_pair。
control先执行原两项p8g所有权guard；任一失败则不进入首次操作。所有旧p7门不改。
准备阶段分别记录两端实际Relay事实，然后采before_idle；10秒静默后恢复Direct，再等待名义phase秒，
采before_action、只发一次原30秒目录读取、采after_action，最后才查询两端路径。
实际restore前后、请求前后、命令前后均用同一单调epoch记录；没有准备后的Node状态轮询或另一端业务预热。

每次只读快照记录ip -s -j link及tc JSON/text。v1只采两个端点，错误地要求alpha命名空间内有两个netem；该轮metadata失败原样保留。
v2读回包含两端eth0/eth1及second路由器各端口的原始字节/包/错误/丢弃计数。
初始alpha eth1出口一个150ms netem，其入口对应second路由器的peer veth一个150ms netem；
beta无netem、无额外TBF；恢复后两个端点和second均无netem。
须以两端ifindex/link_index双向匹配绑定router peer，不能仅靠名称或队列数猜测对应端口。
实际JSON秒单位与text 150ms交叉核验，不用请求模型参数代替安装读回。
每条命令原2秒观察界、kill_on_drop、调用者持有Device任务；命令失败原样保留。

外部source-owned校验需六个完整场景与十二条逻辑请求起止记录、实际Rust六项计数、原p8g两项全绿；
每case校验配置、方向、阶段/静默实际时间、唯一caller、结果、前后配置与计数单调性。
请求错误/超时仍算完整失败样本，不从分母删掉；缺日志、缺case或配置不符标不完整，不补跑漂亮样本。

idle/action按各设备和接口分别计算差值，包含快照间隙，时间用实际命令界给范围；
两端重复观测同一批数据，不能相加当唯一字节，也不能称probe-only、CPU、唤醒或手机能耗。
6个样本仅作首次操作探索，无p99、最终恢复SLO或完整N1接受。


## v1配置缺口与v2修正（首个v2数据前）

v1六次读取均成功，但整轮因缺少入口安装读回而失败；不能按原Rust六项绿宣称模型通过。
实际vendored patchbay src/iface.rs::apply_condition把Egress装在device命名空间eth1，
Ingress装在所属router命名空间的veth。该文件SHA256为
`2e819ab15361340e8558b281627f6a768626169955ceafa9641f1fbd925c475f`，
根Cargo.toml用vendor/patchbay；读取到的registry同名文件逐字相同，但实际证据以vendored源为准。
v2只增加保留second Router句柄与调用者持有的只读采样任务，以及对应配置/计数核验。
原网络配置、六fresh顺序、准备和静默/phase时间、30秒caller与操作数量不改；revision显式升到
first_foreground_after_idle_v2，原v1数据不充作已通过的v2样本。
新增router读回会有独立观测开销，记录各命令时间与快照间隙，不能当作零开销前后对照。
