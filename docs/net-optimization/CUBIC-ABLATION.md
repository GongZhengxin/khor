# 当前Noq拥塞控制的隔离诊断

本页由普通Release唤醒消融的真实反例触发，不把CUBIC预先选为产品默认。

W block5（turing-20261113T100245Z-0d77409d）有完整source/control/owner/资源及raw分析见证。low原功能门通过，但终端在bulk和idle_after累计排队，scheduled-to-end p95分别8.868761s与9.910280s；这只是该轮n120结果。high原门失败：720计划记录中600成功、2超时、118blocked；完整192MiB文件及PTY关闭均留证。一个终端超时发生在idle_before，也就是文件下载之前。blocked是前一失败后未发送的意图，不是额外118次网络超时。证据E/该run/copy-wake-original-failure-analysis-v1.json；E=/Volumes/disk0/Khor/evidence/net。

这一反例已足够拒绝把W当成已解决原超时；原固定9块仍完整执行，不删失败、不补绿。它没有单独证明传输、拥塞控制或应用排队中哪一层是根因。

准备的源码位于khor-net-cubic-ablation-v1，branch experiment/net-cubic-ablation-v1。以准确C共同fixture tree adbc7d6a为基线，runtime仅将endpoint::transport的Bbr3Config::default切换为当前Noq原生CubicConfig::default，另有明确的实验注释；无vendor、超时、调度、质量、0RTT或信任变化。不含W唤醒变化或R恢复补丁，保持分量隔离。源tree21f37a7a2fb96abcd601db486c26b687767cade2，snapshot74c762dfffb849b9ec4cb5e68f588dd9698df50d。

原始设计协议E/cubic-ablation-design-v1.json保存准备时状态；当前已在NAT全批闭证且调度器退出后派发普通Release compile-only，control44328e0e755fcec5eae0772e07fcbe59eb9eda48、run turing-20261113T110223Z-34e7344c。该producer已0/586.890585s结束并核20files/identity/原native凭据，实际编译8m54s。固定3 high runtime已派发，run turing-20261113T111336Z-865645d6、control f0975c8945327ae2e1513b4a5e0778446dc680a6；真实PID12911运行中，尚无结果。计划先用普通Release、无qlog/net-diagnostics、原high夹具固定3个新场景判断CC是否值得进一步调查，原2秒caller、720记录/场景与192MiB完整文件门不变，替代0。仅作分量诊断，不冒充当时交错的完整性能接受；真实producer哈希及控制已在E/cubic-release-runtime-{plan,control}-v1.json预先绑定。

旧源码中已有CUBIC在随机丢包道路吞吐很差的历史数据，不能为解决零丢包模型的终端尾延迟而忽略它。若CUBIC表现较好，仍需当前Noq的loss/idle/burst/短流与吞吐对照、完整原工程门和最终同源矩阵，才能讨论默认设计；若它仍失败，不能继续把BBR当唯一原因。本实验不删除任何BBR补丁。

性能批次的root串行调度器活着时，只允许源码准备，不抢Turing编译或修改其ledger/progress。NAT补丁38的固定对照现已全部闭证：保留臂3/3强属性过、移除臂0/3，故保留该补丁；详TUPLE-REFRESH-EXIT。两个调查都不替代完整N0–N5。


## 固定三轮的实际结果

runtime111336Z-865645d6结束1/504.032874s，15个artifact文件及全部来源、raw、CPU、owned清理、selected还原已核。2160条计划记录全部ok，三个192MiB文件完整、PTY均关闭；原native门2/3。首轮direct_after=false，因此仍记失败；不删除原门，也不把改走Relay当作同一profile下已通过。

原统计重算：pooled high bulk目录/终端scheduled-to-end p95为351845/334029us；完整文件吞吐中位36.734944Mbit/s。准确早先C参考为580146/350138us、24.307740Mbit/s。冻结早先B/C六个延迟格和吞吐界限均过，但不是同时交错比较，不构成默认设计或根因证明。证据E/cubic-release-diagnostic-verification-v1.json。

下一步复用当前源crates/net/tests/congestion.rs的双向独立FIFO校准、两种控制器、typing后screen和完整失败记录，补真正无应用请求的idle后短流测量；协议先固定实际Noq1.4、零/2%丢包、样本与来源再运行。该纯Noq层不替代真实Node/SSH/hop、公平/取消、手机或最终同源接受。历史Noq1.3丢包吞吐弱点仍保留。


## 当前Noq丢包与静默矩阵

预冻E/cc-matrix-design-v2.json：双向丢包顺序0/2/2/0/0/2%，其余固定300ms、20Mbit/方向、200ms队列和原seeds。每块原20秒bulk及typing后screen；新增仅测量的quiet case每控制器3轮，warm一字节→10秒无应用请求→一字节→6400字节screen。仍用原Noq ask 10秒期限，不能代替Node的2秒期限或宣称产品门已过。所有失败和实际分母全留。

仅隔离树khor-net-cc-matrix-v1增加该测量。source tree25fed870f2637708c85f5a758a583d3d9f44dc65/snapshotcd36eaf2a9227c6b01fc49bf23652dab29873e28，fixture SHA da7e5ca0b414af0a3b278902d8c1222044dafa38b1cc96d453643b76d126aeed。首编译准备112934Z-a0f72d29被格式门拦下，Rust编译/业务0，原失败保留；v2只用其导出格式修正。准确在途句柄见STATE。

分量诊断screen已在runtime前固定：全原调用成功，每loss下各控制器3轮完整bulk，CUBIC吞吐中位≥BBR的90%；各phase p95≤BBR×1.10+20ms。屏幕样本量很小，报告n，不作总体p99置信声明。测量不接纳默认控制器，也不覆盖真实Node/SSH/hop、共享/分连接、取消公平和能耗。
