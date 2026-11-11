# 大带宽时延积模型协议

2026-11-11 17:17 UTC，实施/计时前冻结。完整N0–N5不变；这是[SCHEDULING](SCHEDULING.md)
要求的大BDP覆盖，不修改原低profile v4、原2秒调用或120秒文件门。当前六block低profile
仍使用原冻结v3源码对；本页不能被当作已实现/已运行，也不能改变该campaign的输入。

## 模型与测量人口

| 项目 | 高profile v1 |
| --- | --- |
| 受限位置 | 沿既有几何，仅x的接口双向，各150ms单向延迟；名义Direct RTT300ms |
| 每方向速率/速率队列 | 50Mbit/s、40ms；零配置随机丢包，不把实际qdisc drop当零 |
| 文件 | 96MiB固定seed内容，256KiB切片，逐字节比较 |
| 每阶段操作 | 目录/真实PTY各120个，固定400ms计划，各最多一个在途 |
| 阶段 | idle_before、bulk、idle_after；全部720逻辑人口，失败/取消/缺失/重复保留 |
| 调用/文件边界 | 每次调用原2秒；整个文件新240秒边界，不按切片重置 |
| 程序就绪与预热 | 沿原10秒总界，不因模型大而顺便延长 |
| 记录/探针 | Buffered；同20ms父future借用探针、20000上限/Skip、512行阶段容量，flush另记 |
| 前置/关闭 | 真实双w3 guard；保留完整字节、原关闭API和Direct检查；独立host/child退出另验 |

400ms计划适配300ms基础往返，不通过增加每类在途量绕开排队。名义BDP为
50,000,000bit/s × 0.3s ÷ 8 = **1,875,000B**，约1.79MiB；八片为2MiB。
单纯减少在途量可能限制吞吐，必须在该模型检验。96MiB按名义速率理想序列化约16.1秒；
它只是几何量级，不是完成目标或运行结果。bulk全阶段与实际传输重叠的两种人口都报告，
不能把文件完成后的请求混成传输中延迟，重叠不足30/类不能接受该场景。

新manifest/revision必须与低profile明确区分；固定模型值由已冻结control/validator选择，
不能仅相信日志自己声明的profile。实现可共享有类型的固定Spec及记录器，原低profile仍逐项严格验证，
不得把新模型需要的常量替换成全局低profile常量。原低profile脚本/真实收据保持原件。

## 独立kernel校准

先在相同condition构造器/命名空间几何执行独立TCP，零resident/Noq，独立重复三轮。
三个明确TCP连接，nodelay；4次预热另列、64个64B echo完整记录，upload/download各96MiB。
上传必须收到远端完整比较确认，下载实际全量比较；连接/echo2秒、每次整段传输240秒，
实际服务器任务有父拥有者，所有结束路径回收。使用96MiB避免把8MiB短流慢启动当50Mbit稳态率。

预注册模型有效区间：去除但不删除4次预热后，稳态64echo的min RTT **270–420ms**，
两方向完整payload率各 **40–60Mbit/s**。这是模型/执行器校准，**不是用户SLO或Noq性能门**。
任一方向/任一轮失败保留原始人口，不以最快一轮代替其余轮或事后改区间。

每轮前后在x及其入口网关保存TC JSON/详细文本/接口计数；实际netem/TBF在同一设备、
根/父层次正确，rate应对应6,250,000B/s与50Mbit，delay对应0.15秒与150ms，
lat对应40,000微秒与40ms。burst/limit及实际接口必须读回留证，不预造builder未保证的值。
两个方向经过实际瓶颈的字节/丢弃增量都报告；单位或配置不明时不接受模型。

## 策略/在途量比较的隔离

kernel校准通过且Node自己的实际TC读回齐全，才接受高profile策略数据。
第一轮保持在途8，继续比较同一成功响应priority0/-1，首切片/其他回复0；两树只差一个const值。
沿 **A B B A A B**、每block三个fresh场景、每arm九轮，日志关闭；原始失败仍进入全部结果。
Mac运行时证据只可在完整Git零runtime差异证明支持时复用；高fixture编译/行为必须在Linux实际运行。

在途1/2/8是下一独立因素：该组固定响应策略值，低/高profile分别采用预定
**8、1、2、1、2、8、2、8、1** 的九个fresh场景，每值三次；开始前另冻三树只差PULL_AHEAD一行。
记录固定策略的选择依据与source，不把它混作优先级的单因素效应。少量探索性重复不报告p99/显著性。
若此探索不足以决定取舍，保留不确定性并补有理由的验证，不按表现选择性替换旧轮。

全部报告完整成功/错误/timeout/blocked/cancelled/aborted与缺失人口、service/计划到完成
n/p50/p95/max、完整文件秒数/有验证的payload率、实际传输重叠人口和探针/flush成本。
无自动选胜。单纯高吞吐或小profile低延迟都不能替代完整前台文件/预览、连续控制下bulk公平推进、
共享/分连接、Direct/Relay/peer relay/SSH、1/3/5设备成本及手机/能耗验收。

## h0 v1实际失败与后续诊断（2026-11-11 17:48 UTC）

源96ae5eb7/controlabfadd4，`turing-20261111T173405Z-3747b66c`已302.367秒、退出101，
17files/identity/原group空。三轮各68echo（含4预热）全部ok，两方向各96MiB逐字节一致、server均成功，
实际配置读回为rate6250000B/s、单向delay0.15秒、TBF lat40000微秒；三轮min RTT300483–300514us。
但是upload25.214–25.301Mbit/s、download26.049–26.144Mbit/s，**全部未达40–60门**。
原job/contract保持失败，独立`high-kernel-bounds-failure-v1.json`核全部原始人口和物理失败，
不把功能数据完整当校准通过，不降低已冻结的物理区间，也不执行/接受未校准的高profileNode性能。

TBF实际burst均4093B；x三轮TBF drops增量287/252/211，gateway236/227/241，netem也有28–45个drop。
因此零配置随机loss不等于零实际drop。[patchbay代码](../../vendor/patchbay/src/qdisc.rs)的add_tbf
固定传`burst 32kbit`；[上游iproute2 TBF手册](https://raw.githubusercontent.com/iproute2/iproute2/main/man/man8/tc-tbf.8)
指出较高速率通常需要较大burst。它支持检查该参数，**不证明当前内核/计时器或本次瓶颈主因**。
当前还没有传输中的TCP窗口/拥塞/缓冲事实，不能仅按速率恰好约一半就改burst、socket窗口或队列。

同准确源的LOW回归 `turing-20261111T174224Z-40546e83`已95.427秒0，17files/identity/原group空；
三轮min RTT40265–40373us、upload约7.519Mbit/s、download7.544–7.553Mbit/s，
原35–80ms/6.4–9.6Mbit门、全部双向8MiB、12份实际qdisc读回都通过。
`kernel-low-model-regression-v1.json`保留Model->原condition的源连接与未放宽的实际profile checker；
旧verifier的字面调用字符串已不适合新的函数指针写法，原件不改。LOW通过不接受HIGH。

### 先补窗口诊断，保持物理参数

下一cohort只在h0独立TCP传输增加有界观察，不先改300ms/50Mbit/40ms/原burst、96MiB、2秒连接或240秒总界。
在client与server的upload/download有效传输期间，计划在本地payload阶段开始后3/8/15/25秒读取
`ss -tinm`，按实际TCP四元组限定本次ESTABLISHED socket；保留命令状态/原始输出/实际时刻与方向/端。
每方向/每端至少应有3秒与8秒两次有效观察；15/25秒仅在传输仍未完成时采样，不能因更早完成判失败。
记录cwnd/rtt/rcv_space、socket缓冲与可用的重传/交付率/窗口限制字段，不从字段缺失臆造数值。

观察future由原父future借用，与真实传输一起poll；不启动游离任务，命令kill_on_drop，
传输完成或原caller取消时观察结束。额外命令/日志有开销，标明diagnostic cohort，
不混入旧h0或Node策略计时，不改变原物理失败门或改写旧report。
须先看这两端事实再决定是否需要单因素burst/队列/窗口对照；任何后续模型变化另冻版本、重新校准。
此节是已冻结的诊断计划，**尚未实现或运行**。
