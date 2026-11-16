# 桌面静默成本

2026-11-16：成本尚未接受，当前先建立有真实产品 resident 的测量。不能从无新增 timer、原生单元绿或一次连通推断手机省电。

准确被测产品是 5f51e340 / tree395f91ae / snapshotb846df21，普通 Release producer `turing-20261116T090926Z-6289823d`，CLI SHA256 `64af80ac8f5e1d7d1352d7beb7e6137e466836de4e0dd0a3ce11be5fcebb50cc`。独立夹具 producer `093646Z-60fe4209` 用原 opt3/strip/thin-LTO/codegen1；复制原依赖锁、全部8个根 patch，编译前核验依赖是原锁子集。产品源码、锁与CLI未改。格式检查0，编译收据在 `E/RUN/idle-cost-build-verification-v4.json`。

固定矩阵为两种配置各1／3／5个resident、各三轮，共18场。先独立3节点probe验测量器；它不是替换失败trial。每场验证所有有向业务，再settle30秒、quiet60秒、记录CPU ticks/HZ、RSS、线程、namespace接口bytes/packets，最后用原2秒期限读取真实目录。CPU用单核百分比与实际采样边界，原始计数全部保留。quiet内无夹具Node查询，既有产品任务仍运行。Direct-capable是网络能力配置，不宣称quiet期间逐包使用Direct。

Turing内核没有可用的调度唤醒计数，perf访问级别为4；不更改宿主权限或sysctl。线程上下文切换仅作有明确缺陷的代理：快照只含共同线程ID，遗漏退出线程，也不能独立确认线程代际。报告不称其为硬件wakeups或手机能量。桌面CPU/字节测量不替代iPhone前后台、锁屏、换网与能耗。

E=`/Volumes/disk0/Khor/evidence/net`。协议 `E/idle-cost-plan-v4.json`、`E/idle-cost-runtime-plan-v1.json`，当前run与下一步以STATE/原receipt为准。源码在 [fixture](../../tests/net-idle-cost-v1/README.md)。前三次编译入口拒绝保留：跨run build_commit比较、原producer PATH未还原、standalone遗漏根patch的offline resolver拒绝；前二0编译，后一无compiler-artifact，均0业务，不归类为产品红。V4沿原门改正夹具，不减断言或绕过provenance。

N3仍须同源共享/分连接共存及同期公开main对照、全部负载、公平与实际手机成本。未在观察结果后补造成本预算，也不以桌面namespace结果宣告能耗通过。

首个3节点probe `094918Z-b4a35bc1` 已原0/132.304s闭证：每进程单核CPU约8.57–8.81%、RSS97–105MiB、275–276线程，eth0静默每方向约30–43KiB，三个后读23.606/26.774/28.170ms。指标仅该配置、准确CLI、60秒窗口成立。CPU不低，但未证明由网络新增或哪段工作导致；完整18场与必要的基线/诊断继续。
