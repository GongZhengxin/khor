# 首个恢复ACK与采样窗口的顺序

2026-11-12。此页记录隔离回归与待验证的最小修正，尚未接受。原四秒物理门、两秒确认、双边决定后样本、三次恢复probe/250ms间隔及已有取消/安全合同保持。

## 观察与可验证假设

原生日志两fixed已全部终态停止：145400 R1原warm/decision过；145752 R2业务124全ok，但X/Y Go5411001/7404483us，原两门红。结果E/iroh14-native-trace-results-v1.json SHA f1cc7d4e76d25bed4064f7963703bf3afb5e751af08cc957ee4dc29faab49f51。TRACE只改CLI opt-in filter，存在CPU/IO开销；不是性能或修复接受。两轮都有三端原生Noq/STREAM日志阳性，但没有唯一drive connection id，不能合并相同stream编号宣布逐包根因。

R2的X在+1.657s有一个observed=true的新ACK，stats.latest_rtt为93.923193ms，而Reading质量仍138.701662ms；相应recovery=None。+3.408s才开始最终两秒确认，+5.411s Go。原行绑定E/iroh14-recovery-ack-order-observation-v1.json SHA d488dd2868eeaa31674a290e4b20e4f606ab84d68619d3c7ac0f4c564c1fd48f。

生产read_paths先读各copy并尝试已有episode的质量更新，然后mark_silent，再recovery_probes。若第一个故障后提示已经是决定后RTT，mark_silent会结束首次可达性hold并移除Silence；后续recovery_probes因此没有机会保存该故障的有界质量episode。此前单元先提供独立receive hint并建立episode，再喂新RTT，没有覆盖ACK先到的顺序。

这能解释一条旧瞬态质量继续参与中位数的链；尚未证明它是全部首连、无carried、NAT opening或手机现场问题的唯一根因。

## 先验协议与红源

E/iroh14-recovery-ack-order-plan-v1.json SHA cad768d3783b0428764ba098e73c4e23952f42e822e1b6b76af57b5ad1fbcd64在改动前冻结：从67建立独立khor-net-iroh-recovery-ack-order-v1 / experiment/net-iroh-recovery-ack-order-v1。只提取原完成顺序到Measured::complete_read私有helper，并新增a_first_recovery_ack_keeps_the_bounded_quality_episode回归。

当前红source **70a6f7d9565e83d0f8bc42bb76940ed366d3ce10** / tree **5965044b670eec762b366d3e23b9120e82016ee6** / snapshot **0988a71b0182b1e0c792e1d84a6eac22bcdf1df6**。把helper内联回read_paths并移除新test后，文件与67逐字相同；其他源码/vendor/锁/物理/parsers保持，source-audit SHA dd08574a32bdb6886a324c6d0ec15039331031322f17e23de7aa4d0e0208ffc0。本地只作Git/标准库源码核对，没有本机Rust执行。

远端full vendor预期红150712Z-549ba3dd在途，未出真实结果前不记红回归已证。若证实，再仅调整完成顺序：在mark_silent撤回故障上下文前，先按同一heard/silent/open条件评估原recovery_probes；不新增状态或配额。已有新鲜性检查、质量step、Moving及预算逻辑均保持。

## 接受范围

红→绿须同一原full vendor/全部旧命名测试及原ignored；之后准确源固定12应用命令。物理screen另在运行前冻结，保留所有失败，不能因单元绿晋升。修复漏建episode会增加此前漏掉的实际probe，原预算不变不等于流量/能耗不变；仍须健康噪声、静默与成本对照。完整N0–N5、Mac、混版、真机和原67无carried失败保持。

## 红已证实，最小候选在途（15:13 UTC）

150712Z-549ba3dd终态101/200.479139s，17files：Noq467原通过/iroh197原通过逐名保持、原ignored1，仅新增合同红（left131ms/right85ms）；node test compile过。准确三锁/source diff空/hash/identity/group空、proof/ledger闭。红不是格式或编译失败。

当前候选3b734073b410ec4130b4b57def3bf096b10c3e7a/tree473ee06d1a836d56d9e39bd10a7a4016fdacf530/snapshot4a688a33bd3da7f095cd8880ed1fd9519fb5ce09，只把complete_read三行调整为原recovery_probes先评估、mark_silent后完成。反转此三行与红源逐字一致，测试段完整不变，source-audit SHAa814baae82dab7b97cc92c24b620c8ee08647104f5d6a155d7c20c75bc2d2f5d。151200Z-d504e81e原full vendor在途，未出结果前不记库绿或实际恢复已修。
