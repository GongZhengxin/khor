# feat/net 当前状态与接续入口

更新于 2026-11-14T11:43:48.946072+00:00。活动目标“好 完成完整目标”保持active；完整[GOAL](GOAL.md)、[PLAN](PLAN.md) N0–N5与[REQUIREMENTS](REQUIREMENTS.md)未整体完成。root串行、不派子智能体；本机source-only，重型工作在Turing/macserver/授权khor-build。用户已授权实施与验证，未授权main合并、现装App替换、发行/TestFlight或生产Relay采购部署。

## 当前执行

- **Turing普通Release producer在途：turing-20261114T113837Z-6286623e**，control6247b13a0ee902557a6026923269318ffb1dc840，receipt在E/同run目录。只编译当前CLI与完整netlab依赖，features[]、原thinLTO/strip/cgu1、原provenance/fresh/resource门；不运行性能数据、不安装App。先poll同handle，不重派。E/pull-pool-release-compile-plan-v1.json及ops/verify-pull-pool-release-compile-v1.py已冻结。准确标准317及cfg(test)-only新压力1复用，不重复回归。
- **同源Mac原生回归GH43590483138/job132433946148已闭：314pass/0fail/原ignored1**（[Actions](https://github.com/colehank/khor-build/actions/runs/43590483138)）。准确source/control与11个control blobs、原12组每个test名、32个明确separate合同、14contact、4个原Linux-only cfg排除均核对；artifact14112262250/38847B/SHA2566c1cc072e169bc19954af8a25055db37645347aec4ea1afe380d767d422f2c56及CRC已保留。E/pull-pool-mac-verification-v1.json；独立Mac pending已闭、common ledger只追加completed，Turing pending未改。Rust1.95/arm64/jobs2/原admission门，未安装或发行App。原iPhone就绪窗口已过，不自动采样。

当前候选worktree **/Volumes/disk0/Khor/worktrees/current/khor-net-pull-pool-pressure-v1** / **experiment/net-pull-pool-pressure-v1**，clean；HEAD **ff3360cd6af3629c869e6e47028636e2ee0d46c3** / tree **4f0e7db7955531014e3b2ac7e8a8f100797bb989** / snapshot **6992bf6abd6202866e01fd3ec66f4c31db679b54**。E/pull-pool-pressure-source-v1.json。相对产品修复源29b05533504e90a16b52412ab1838f60005f0142/treee71011edd63b8f39734e8d44c6be31222a78d7bf，仅2个cfg(test)文件增加五对端压力合同。默认仍shared，内部separate opt-in；未并入canonical feat/net或main。

E=`/Volumes/disk0/Khor/evidence/net`；原receipt、远端execution及E/source-refs.json为实际调度依据。上版STATE逐字归档[HISTORY-20261114-through-pull-pool-resource-baseline](HISTORY-20261114-through-pull-pool-resource-baseline.md)，SHA256 `ec8e653e1163cc65000dce1474a61fadc125f227d61b45958f6755da191df8df`。其中在途/下一步是历史，不得据此重派。更早的全部证据、原失败、未执行人口通过历史档案与REQUIREMENTS保持，不被本入口替代。

## 最新完成的证据

- **资源基线104608Z-b34b246d：1pass/3fail**（sourceb292）。4实际extra满额，remote确已关闭1而旧Lease Arcs保留，第5peer仍被capacity拒绝，这是准确产品RED。by-value Endpoint完整cached文件/实际正校准通过。另2 owner用例原join(waiter,accept)未分离结果，不归为产品故障。
- **同产品owner诊断105305Z-6ccfbea6：2/2**（sourced4f，0/98.165546秒）。不改500ms owner、30ms waiter pending、原Incoming故障及5秒观察；只分开实际waiter结果与accept，记录一次aborted握手后继续接受。取消与到期均证明未到期waiter实际成功、额外容量归还、cached Main身份与真实Sessions保持；原单次accept观察器不足，未因此改产品拨号语义。原失败保留。
- **关闭容量修复110024Z-d65102fc：31/31**（source29b，0/118.244874秒）。仅在extra满额时，快照weak peers、释放map锁，回收实际close_reason非空的Lease permit后立即再试；健康4连接仍按原cap后退。相同资源测试证明remote已关、旧Arc仍活时peer5实际握手成功，其他3健康，旧Arc Drop不关闭新连接。无后台timer/probe、无抬高cap/期限。E/run/pull-pool-resource-native-verification-v1.json。
- **原标准12组110608Z-de8ff73e：317pass/0fail/原ignored1**（同source29b，0/502.085646秒），21files、identity/owner/原fmt和逐测试名闭证；31个明确separate合同及14个contact均过，多数旧case仍shared。E/run/pull-pool-resource-standard-node-verification-v1.json。**不重复此标准组。**
- **五对端全资源压力112137Z-7b003b5a：1/1**（当前ff3360，0/92.307242秒）。一个Turing进程内5个真实认证QUIC endpoints，非5物理设备/WAN性能。前4 peer持有32bulk/28实际decoded-ready与4extra，第5peer通过cached持有8foreground；新caller到期/取消均无RPC，满额时Sessions仍回应。只释放1个head便使该完整文件及随后另一个13块文件完成，其余11head仍保持。abort/join后11个实际STOP、全部credit/extra返还，5条cached Main原stable-id/真实Sessions保持；无重复offset。E/run/pull-pool-pressure-native-verification-v1.json。当前源是在产品相同的标准317上新增此1case，不能称一次完整318运行。

[PULL-POOL](PULL-POOL.md)与[PULL-POOL-DESIGN](PULL-POOL-DESIGN.md)记录归属、head cursor、bulk32/peer8＋foreground8资源边界。原公平性RED→同测试GREEN、旧27及全部准备/运输/写不重放边界保留；资源满额通过不等于Release性能或手机体验通过。

## 接下来的顺序与完整范围

1. 当前Mac准确source/control/artifact/全部人口已闭；收齐当前普通Release producer源码闭包、fresh CLI及netlab proof。未知结果恢复同receipt，不派替代。
2. 用当前Release先核真实布局，再在采数据前冻结新的速度优先共存协议及1/3/5业务、CPU/唤醒/字节成本。比较目录/PTY与完整文件并发、小文件首内容、吞吐及存活；不能用路径标签硬要求Direct，不能把原screen最后标签失败解释为已修。
3. N1置信/多middle/切换和成本、N2完整期限/取消/迟到Endpoint与policy队列变化/写入0-RTT/NET-16负向、N4冷hint/新peer发布/Direct UDP突断/peer relay自身移动/休眠/Wi-Fi蜂窝/Relay故障域与容量/NAT64，以及N5全部60项custom退出审计与最终同源Quality/Mac/phone/报告继续按REQUIREMENTS完成。手机在具体候选可测时重新安排。不得把“Mac＋性能＋手机”缩成全部剩余范围。

## 不可误用的既有证据与边界

- 原S三臂screen已按预定hard-fail停在7C high direct_after=false：10080读/14文件成功，但组合门失败；8A/9B未运行，原门与失败保持。D fixture-only最终Link固定有/无日志各3诊断共4320读/6文件成功，0/6复现，不是修复，不再原样重复。4个p6速度优先普通Release合同已过，不重跑。
- 旧fe61 Linux294/Mac290、eacc标准306均仅对准确源成立；新pool不能沿用它们当Mac或Release通过。最新317及压力1也不能证明全部N3。原公平性同测试GREEN及其他组件证据不等于N0–N5完整接受。
- 上游最近官方核验（2026-11-14T05:37:22Z）目标iroh1.4.1/Noq1.4.0已在独立候选，canonical feat/net仍旧依赖；60历史补丁仅1上游替代、2Noq移除红与1iroh38移除红有专属结论，56待退出核查，不能称全部custom收口。
- MacLab与Turing同LAN，macserver另网络。iPhone localhost已配对但尚未接受本候选换网/能耗。khor-build公开已获明确授权，不等于App发行/main合并。LIVE23961未动。
- Turing UID1110、原flock/磁盘/工具门；计时不与同host编译混跑。root唯一ledger写者；stage一次、launch一次；collect原返回码后核identity/owner及真实合同才闭ledger。Mac另主机可并行。
- 实验worktree图谱0nodes/edges/files，定向fallback；risk0/testgap0不证明覆盖。main图谱不代表候选，不擅自重索引。本机不跑Cargo/Rustfmt/Node/Bun/Xcode/产品；main9a4219ffe5e3与builder checkout HEAD5d3691保持。
