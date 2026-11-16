# Noq 1.4恢复行为退出审查

2026-11-16。结果按准确源码与原合同报告，未接受完整N0–N5。

原Noq subtree `2edce6019a81b91801d469305b128cea018d314a` 与02807已过467的完整库相同。诊断source `9e31b841` / tree `3cc2bfe8`，control `ad2ae8aa`；原测试源码与期限不改。每个运行副本只禁用一个恢复行为，fresh target构建后执行相同4个真实QUIC-engine合同。

| 移除行为 | 远慢旧路重排 | 健康等速不重发 | 原RTT快的静默旧路 | 切路后旧路才失效 |
| --- | --- | --- | --- | --- |
| 7：远慢旧路释放 | 失败 | 通过 | 通过 | 通过 |
| 9：不应答旧路释放 | 通过 | 通过 | 失败 | 失败 |
| 11：backup丢失tick重查 | 通过 | 通过 | 通过 | 失败 |

`turing-20261116T032958Z-36318d40` 原退出101，66.112秒，12合同=8通过/4失败，全部原生进程组已清。失败均为原在飞数据释放断言，不是编译或工具故障。payload及真实stream由既有合同驱动；不能把失败断言前未走到的payload接受写成已通过。

结论：当前保留这三项恢复行为。它们在已有QUIC-engine恢复合同中无法直接删除；不证明当前代码是唯一实现，不证明全部custom必要，也不替代物理应用/移动网/SLO。此前Noq2/5物理移除失败及iroh38真实NAT证据继续有效。

E=`/Volumes/disk0/Khor/evidence/net`。冻结方案 `noq-recovery-necessity-ablation-plan-v1.json`，独立核验 `turing-20261116T032958Z-36318d40/noq-recovery-necessity-verification-v1.json`，逐项账本 `iroh-upgrade-patch-ledger-v6.json`（53待审/1上游替换/6专属保留）。原失败日志、控制器原101和所有8绿保留。

2026-11-16再次查官方release与crates.io：iroh稳定1.4.1，Noq/noq-proto稳定1.4.0，均与独立产品候选一致。Noq GitHub latest alias仍指旧1.1.0，不能据此降级。准确metadata/checksum记录 `upstream-stable-check-20261116-v1.json`。canonical feat/net依赖尚未最终集成。
