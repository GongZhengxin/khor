# 高BDP独立数据报容量诊断

2026-11-11，实施与结果前冻结。Turing只读预检无iperf/iperf3/netperf/nuttcp；不安装全局软件。
新ignored入口uq0、revision `workload_kernel_udp_capacity_v1`，固定任务
`netlab-workload-high-capacity`串行执行三个fresh场景。完整N0–N5保持。

## 要回答的问题

相同[分离转发几何](QUEUE-TOPOLOGY.md)在无TCP拥塞控制时，是否实际提供40–60Mbit/s的接收payload率？
两个router的WAN各root TBF50Mbit/40ms/65536B，接收方向LAN各root netem150ms/limit1000，endpoint无额外整形。
不改burst/queue/算法/窗口，也不把新协议结果改写为旧TCP40–60门通过。

依据：[RFC6349](https://www.rfc-editor.org/rfc/rfc6349.html)区分TCP equilibrium与瞬态，
[RFC9438](https://www.rfc-editor.org/rfc/rfc9438.html)定义CUBIC窗口行为。选择增加UDP诊断是工程判断，
这些文献不证明本次唯一根因。该诊断不测RTT，不冒充重新完成TCP echo/完整文件或Noq验收。

## 固定输入与有效性

- 先x→y，再y→x，互不重叠。每方向20秒，1200B UDP payload，70,000,000bit/s计划负载；
  总计划floor(20×70,000,000/(1200×8))=145833个包。以1ms absolute interval/Skip调度，
  每slot仅发该slot的7或8个序号，跳过slot不追赶突发。发送操作受原20秒总终点限制。
- 每包携带方向tag、固定计划序号和确定性内容。发送成功/失败bitmap全留；跳过序号由全集差集保留。
  接收逐字节核验，保留收到bitmap、重复/错误/异peer/越界数量，逐包归属唯一tuple，禁止隐式重发。
- 接收从绑定就绪起至25秒终点，父方向总界28秒、准备就绪2秒；父拥有所有namespace任务，取消即abort。
  绑定本次实际IPv4地址/临时端口，不使用用户配置/已安装App/host网络，不创建游离observer。
- 发送端按本地epoch记录21个1秒bucket；接收端以首个有效包为本地epoch，记录25个bucket。
  稳态窗各取[5,15)秒；不跨endpoint相减时钟。接收首包必须≤2秒，以使窗口落在20秒发送期内。
- pacer有效需要：无发送失败、成功数≥计划99%、实际发送总时长19–21秒、稳态payload率65–75Mbit/s，
  最大唤醒迟到≤20ms、迟到>10ms的slot≤1%，不借loop catch-up补漏。
- 接收有效需要无重复/内容错误/异peer/越界，实际receiver时长24–26秒，完整sequence集合是sent集合子集。
  sender/receiver tuple互相对应，全部收到包数与bitmap/bucket/payload字节一致；missing=sent−unique逐项报告。
  70Mbit负载高于50Mbit整形，实际drop允许且必须完整保留，不把零配置随机loss说成零实际drop或用户SLO。

## 判读与证据

每方向独立记录pacer/receiver人口、两端本地窗口rate、完整丢包分母、tuple和bitmap。
两个endpoint与两个router各before/after共八份TC JSON/详细文本/interface计数；沿原严格分离角色校验，
每个方向的实际路径byte增量必须至少覆盖两个方向中较小的已核验接收payload总量。
物理诊断门仍每方向稳态接收payload率40–60Mbit；观察有效与容量门分别记录。

Rust保留观察/容量任一失败的原退出；source-owned metadata只可证明观察完整性，不覆盖原红。
三个fresh的全部人口/错误/失败保留；不择优、不报p99/显著性、不混作TCP/Node策略对照。
即使三轮UDP容量均通过，也只定位IP容量，**不自动接受模型/旧TCP门/Noq/Node/N3或完整目标**。
后续模型/业务验收方法若需要修订，须基于证据另做明确决策，不能默默换门。
