# 当前Noq拥塞控制的隔离诊断

本页由普通Release唤醒消融的真实反例触发，不把CUBIC预先选为产品默认。

W block5（turing-20261113T100245Z-0d77409d）有完整source/control/owner/资源及raw分析见证。low原功能门通过，但终端在bulk和idle_after累计排队，scheduled-to-end p95分别8.868761s与9.910280s；这只是该轮n120结果。high原门失败：720计划记录中600成功、2超时、118blocked；完整192MiB文件及PTY关闭均留证。一个终端超时发生在idle_before，也就是文件下载之前。blocked是前一失败后未发送的意图，不是额外118次网络超时。证据E/该run/copy-wake-original-failure-analysis-v1.json；E=/Volumes/disk0/Khor/evidence/net。

这一反例已足够拒绝把W当成已解决原超时；原固定9块仍完整执行，不删失败、不补绿。它没有单独证明传输、拥塞控制或应用排队中哪一层是根因。

准备的源码位于khor-net-cubic-ablation-v1，branch experiment/net-cubic-ablation-v1。以准确C共同fixture tree adbc7d6a为基线，runtime仅将endpoint::transport的Bbr3Config::default切换为当前Noq原生CubicConfig::default，另有明确的实验注释；无vendor、超时、调度、质量、0RTT或信任变化。不含W唤醒变化或R恢复补丁，保持分量隔离。源tree21f37a7a2fb96abcd601db486c26b687767cade2，snapshot74c762dfffb849b9ec4cb5e68f588dd9698df50d。

原始设计协议E/cubic-ablation-design-v1.json保存准备时状态；当前已在NAT全批闭证且调度器退出后派发普通Release compile-only，control44328e0e755fcec5eae0772e07fcbe59eb9eda48、run turing-20261113T110223Z-34e7344c。核真实PID4180496仍在编译，尚无runtime结果。计划先用普通Release、无qlog/net-diagnostics、原high夹具固定3个新场景判断CC是否值得进一步调查，原2秒caller、720记录/场景与192MiB完整文件门不变，替代0。仅作分量诊断，不冒充当时交错的完整性能接受；实际producer/control仍须在执行前绑定。

旧源码中已有CUBIC在随机丢包道路吞吐很差的历史数据，不能为解决零丢包模型的终端尾延迟而忽略它。若CUBIC表现较好，仍需当前Noq的loss/idle/burst/短流与吞吐对照、完整原工程门和最终同源矩阵，才能讨论默认设计；若它仍失败，不能继续把BBR当唯一原因。本实验不删除任何BBR补丁。

性能批次的root串行调度器活着时，只允许源码准备，不抢Turing编译或修改其ledger/progress。NAT补丁38的固定对照现已全部闭证：保留臂3/3强属性过、移除臂0/3，故保留该补丁；详TUPLE-REFRESH-EXIT。两个调查都不替代完整N0–N5。
