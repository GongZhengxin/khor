# 真实 Node 混合负载基线

2026-11-11，`resident_mixed_workload_v3`。这是 N3 的可复现实验协议，v3已取得下列三轮功能基线；不替代完整 N3 或最终用户体验门。沿用 Linux namespace / scratch resident / 原资源门，独立串行三轮，运行时冻结产品 tree、control、原始日志。产品调度和连接策略不随此实验修改。

固定两个真实 resident，公开 IPv4，配对后先达到 Direct。仅 x 接口双向配置：单向20ms、每方向8Mbit/s、40ms队列、零随机丢包。该配置的实际吞吐尚未独立校准，日志明确 `calibrated:false`；第一轮是测量入口和功能合同验证，不能把名义速率当实际容量、测得差异当拥塞控制收益。

实际调用链：目录 `ls_of → resident IPC → Main`；16MiB文件 `pull_path → resident IPC → FetchPath slices → Main`；终端 `open_on(OpenOnce argv) → reach → resident Tunnel → remote PTY`，使用现行 cookie/Version/Ready 协议。**当前终端本来就走独立 Tunnel ALPN，不能假定终端与文件在同一条 QUIC connection，或把创建第二连接当作尚未实现的优化。** 目录与文件是 Main 竞争候选，需随后按 journal/真实 wire 身份复核，而不是只看 UI 路型。

每轮按 idle_before / bulk / idle_after 顺序；每阶段目录与真实 PTY 各120个逻辑样本，100ms闭环调度，总720个。记录每个计划/开始/结束时刻、延迟、调度迟到、成功/错误/超时/前序终端失败导致blocked，原始记录不删除。2秒是实验调用边界，不是用户批准的SLO；终端失败后不隐式重连或把剩余样本丢掉。三轮用于定位，不声称足量p99或消除了顺序效应；前后空载阶段用于观察漂移，最终比较需要交叉顺序与多seed。

终端在 scratch home 打开真实 shell，关闭本地输入回显后 read/printf；请求不含响应前缀，每个响应含唯一phase/index，避免把PTY输入回显、启动命令或旧环形缓冲当成功。整个测试仅向新建scratch会话输入；现有会话和用户resident不参与。协议、流与测量future由父调用拥有，结束显式关闭会话，scratch清理继续兜底host pid。

文件由固定xorshift64 seed `6d697865646c6f61`生成16MiB，接收后逐字节全量比较；不得只以文件大小或请求返回认定完整。bulk最多120秒，记录完整开始/结束/字节/错误。两类交互至少各30个样本完全落在bulk真实在途区间内，否则本轮不是有效混合负载。每阶段前后记录resident wire计数，末尾保留两端journal。日志本身有开销，明确是diagnostic baseline。

首批只判测量完整性、所有逻辑操作成功、真实重叠、文件完整和scratch会话关闭；**不以其functional green声称延迟、吞吐、CPU、能耗或队列设计达标**。后续必须补独立链路校准、完整实际wire/stream身份、真实队列/许可/背压事实、同条件对照与候选公平调度、SSH/peer relay双腿、1/3/5设备、CPU/字节/唤醒及真实手机能耗。缺项保持在 STATE / PLAN 中，不把本协议缩成 N3 的完成定义。


v1首个可编译版本在三次PTY预热中均10秒未取得预期回复，计时样本为零，原因未确证，不当产品性能红。v2修正测试前置：Host Ready只释放启动脚本；先等程序实际输出就绪标记，再发送预热输入；标记通过printf分段生成，整串不在会被本地回显的启动命令中。Ready、程序就绪和预热应答仍共用原10秒绝对预算，不叠加等待。输入使用产品现有shell_start::input生成真实Enter/CR。增加仅启动期的有限原始PTY输出诊断，计时样本不打印内容。该修正尚未验证，不提前归因LF、产品或CPU；实际关闭仍只证明API回复，独立host/child退出验证待补。


v2仍三轮预热超时，有限原始输出仅为shell提示符、没有脚本就绪标记。静态调用链找到CLI差异：`spawn_shell_at`设置KHOR_HOST_STARTUP，而`crates/commands/src/lib.rs::host`调用host_main，后者明确传startup=None；Node自带host_args则会读取该环境变量。这个宿主入口问题另记，不将它归成网络延迟，亦不在网络基线中顺带修改命令引擎。v3沿仓库现有`crates/cli/tests/term_echo.rs`的真实`open_on`命令会话入口：OpenOnce固定token、显式/bin/sh -c运行同一个read/printf程序、同样的实际PTY/cookie/Version/Ready/Tunnel路径、同10秒就绪+预热总期限，样本/负载/正确性门不改。此版尚待远端验证。


## v3已收原件的三轮功能基线（13:42 UTC）

源3c1e852002e0ade43215c248e45caaf8a780b791 / tree66c5744b067f0a594da54d04485868938af55f07；control ebe8c8243ee639aa63aaa08a3202cbc9d1c0c8d8；Turing turing-20261111T133630Z-2aff8ae2，216.845秒终态0、17files/identity/原group空。三轮各720（合计2160）全部成功，三份16MiB逐字节一致；每轮目录/终端各120个样本完全在真实bulk在途期间内，API关闭成功。`workload-functional-verification-v1.json`和独立`workload-latency-decomposition-v1.json`与source ledger均已收齐，full_N3_accepted=false。

下表p95只按本组120个成功样本计算；每组失败/超时/blocked都是0，不能扩称p99或最终可靠性。单位ms，空载取idle_before；后空载原始数据也完整保留。

| 轮 | 目录服务 空载→bulk | 目录计划→完成 空载→bulk | 终端服务 空载→bulk | 终端计划→完成 空载→bulk | 文件有效负载Mbit/s |
| --- | --- | --- | --- | --- | --- |
| 1 | 64.349→107.011 | 65.417→296.407 | 43.767→80.774 | 45.115→81.886 | 7.120 |
| 2 | 61.795→111.742 | 62.889→319.670 | 43.363→82.103 | 44.747→83.314 | 7.127 |
| 3 | 66.466→110.273 | 140.546→289.425 | 43.821→83.887 | 120.563→97.632 | 6.735 |

文件完成18.850/18.833/19.929秒。有效负载率是实际字节/实际耗时，不是独立测得的链路容量。第三轮空载和后空载已有约85–93ms调度迟到；具体原因未定位，不能按系统load猜CPU、磁盘或产品网络。目录计划→完成约300ms含每类最多一个在途的观察器积压，不直接证明Node内部排队300ms。原始数据没有被“只看服务时间”或删除失败过滤；本轮没有足够原因直接采用stream priority、固定批量限速或新连接。

## 下一份校准与定位协议（尚未实现）

先在同一namespace几何和资源门、复用同一条件构造器下，独立验证内核TC瓶颈：明确TCP连接的64字节echo与双向各8MiB完整数据，nodelay、固定payload、预热另记、每步2秒/传输120秒总界，保存所有错误与原始时刻。读取实际qdisc/接口计数核单向20ms/8Mbit/40msbuffer，不能只信builder参数。校准作为模型/执行器证据，不是Noq或Node吞吐基准，也不替代UDP/中继双腿。校准门与运行版本在执行前冻结，不依据结果追改。

同时增加观察器运行时的独立定时探针，分解计划迟到是否出现在发起RPC之前；比较逐样本同步记录与有界内存记录/阶段末落盘的观察器成本，两种都有完整失败人口与取消/进程中断的缺样本判定。它们是测量模型对照，不当产品加速。之后再关联实际Main/Tunnel wire和队列/许可/发送缓冲，选择有因果支持的调度候选，按同条件前后对照复验；仍需多seed/交叉顺序、SSH/peer relay、规模与手机成本。
