# feat/net 当前状态与接续入口

更新于 2026-11-14T06:22:12.524522+00:00。用户授权持续实施与远端验证，活动目标“好 完成完整目标”仍未完成。root串行、不派子智能体。完整[GOAL](GOAL.md)、[PLAN](PLAN.md) N0–N5和[REQUIREMENTS](REQUIREMENTS.md)保持。未获main合并、正式发布、现装App替换或生产Relay采购部署授权。

此前STATE逐字归档[HISTORY-20261114-through-separate-pull-screen.md](HISTORY-20261114-through-separate-pull-screen.md)，SHA256 `85c364bde3626ce7627a3be9055ad1a477a557a2601cbf682d3b58aafd64ba53`；该档案内的pending、下一步和“尚未运行”均为历史，不能用于重派。E=`/Volumes/disk0/Khor/evidence/net`。

## 当前执行与下一步

**唯一Turing在途：turing-20261114T062632Z-8dc26135**，fixture-only普通Release编译；control b4dd806a17205a85a0e18858acddcc081affe088，source ddc62a719cbbc0445617b7940ddea96ca68f85e6/tree e27863d51c82001603728bf3e7ff668fa9c4d2c3/snapshot2c3dcc0bdd8c91219041e29484f86b63b105e05a。独立worktree khor-net-separate-pull-path-diagnosis-v1，只有workload.rs保存/输出原门使用的final Link及对侧Link，产品0变化、门槛0变化。原S未变。E/separate-pull-path-diagnosis-design-v1.json在改源/运行前固定3场high/separate有日志诊断，不作性能接受。collect→verify-turing-record→verify-separate-pull-path-diagnosis-compile-v1.py后，才运行已准备的freeze-separate-pull-path-diagnosis-runtime-v1.py；runtime freezer尚未执行，不得越过编译证明。共同ledger以executor与pending筛选；无在途Mac，末条可能是其完成记录。

**独立文件QUIC三组screen已停止且未通过。** E/separate-pull-three-arm-progress-v1.json phase=`stopped-isolated-hard-functional-failure`；不可运行原第8A/9B或补替代场。停止报告E/separate-pull-three-arm-stopped-summary-v1.json已生成，原计划、阈值、所有失败与未执行人口保持。

当前工作：定位第7C高延迟原生失败`direct_after=false`。该场720读全部成功、192MiB文件完整、PTY关闭/120+120实际重叠/队列读回均通过；最终是否Direct且非down的组合检查失败。原complete只存bool，未存最终Link对象，不能据此判定实际掉线或单纯显示错误。x journal有一次path selected Relay（RTT302ms，质量估计301.38ms，Direct RTT306ms/质量312.47ms），但这是切换后快照、不是决定输入，也未绑定到最终cached Main；不能直接宣布根因。先查Wires sampling、实际连接编号与选路；必要时只追加独立诊断，不改变已冻结screen结果。用户要求速度优先，修复须符合这一方向。

## 独立文件连接实验的准确源与证据

worktree `/Volumes/disk0/Khor/worktrees/current/khor-net-separate-pull-v1`，branch experiment/net-separate-pull-v1，HEAD **dad4306132a51e1807da2058988e13f315b262ac** / tree **0db6508aa2bd2094b9dfcfee6c35715a0af20f17**。默认shared；内部实验flag启用每pull一条额外现有Main ALPN连接，目录仍cached Main、终端Tunnel。未采用/安装/发布，详[SEPARATE-PULL](SEPARATE-PULL.md)。

- Linux045906Z-abc0de50：原12组**293pass/0fail/原ignored1**；原286默认shared，新增7项明确mode-ON真实QUIC合同。Mac GH43563143698/job132351061154同源**289/0/原ignored1**，差4条原Linux-only cfg；新增7项和14contact全过。E/separate-pull-mac-verification-v1.json，artifact14101007961大小/SHA/CRC/20files/全部11控制blob/ARM64/rust1.95/jobs2/installed guard998f及50GiB启动门已核。无App安装，其他任务LIVE23961未动。
- 普通Release producer050842Z-6b257594，0/562.186521秒，CLI **9add2509af6c3d96d3ac82783bc7bbc9803a931ffca34f21355ee8dc897404ca**，full netlab **4ca36fb23937b0ceecff08c7d5b3b7bac64f2cbe071e79ccc3fa957ff7c0b3bd**。source snapshot0a97f374b33f6d7a3b5840b48c55fbf001cb7700、原Release/features[]/thinLTO/CLI强制refresh actual fresh=false、全部source与复用合同准确核验。E/run/separate-pull-native-compile-verification-v1.json。
- 真实布局有日志诊断shared051918Z-ca81d523、separate052331Z-eea39ea2均通过：1440读、2文件、2256响应stream配对。shared目录/file一Main；separate两Main且pull_wire_opened准确匹配filewire；终端均独立Tunnel。采集校准/界/CPU/owner/flag finally恢复已核，只证明布局，不计性能接受。
- 正式screen control **6ee198d424e0313d71812c1f0c5343947be50f99**，plan SHA **9d32bff84c5cda4e00dbb7bacab1f8d27370ca49a3161a2c18b4f881334ab6b2**。A原public-main runtime c3b098bdbf52a9b3b38e887ae74b90155292b544/treef6c239/snapshote22263；B/C同S普通ELF，仅shared/separate flag不同。
- 固定ABC/BCA/CAB九块、每块low100ms/high400ms、原720读/16或192MiB/caller2秒/file120或240秒/owner600，主门C/A相对10%+20ms与90%率、固定原main绝对门保持。实际仅运行7块，**10080/10080读、14完整文件**；整体功能false，未完成九块对照，不能接受N3。

|块|组|Turing run（20261114）|原生结果|
|---|---|---|---|
|1|A|053604Z-010c3ac7|0，1440读/2文件|
|2|B|054110Z-5872daed|0，1440读/2文件|
|3|C|054711Z-5f383a10|0，1440读/2文件|
|4|B|055222Z-35b35b63|0，1440读/2文件|
|5|C|055734Z-f273a63f|0，1440读/2文件|
|6|A|060328Z-4dd54696|0，1440读/2文件|
|7|C|060906Z-6589b6ce|1，高profile direct_after=false；1440读/2文件均成功|
|8/9|A/B|未执行|按预先hard-fail规则取消，不替代|

C三块每cell360样本的描述性p95：low bulk目录79638μs/PTY77024μs，high目录409698μs/PTY332833μs；high文件中位23.98445Mbit/s。均不抵销功能红，也不构成未完成A/C对照结论。两source fresh preflight052833Z-419e8012、053223Z-815dd1d4各原coldTCP高profile红保留，全部传输/几何及其他4门通过，依原MODEL-DECISION分层admit，不称全绿。

所有v1 freezer、record/verify/summarize helper和plan已冻结；不要覆盖输出、改阈值或重派已闭场。source fixture10个公共文件字节相同，lab.rs只增IPv6工厂、完整还原后逐字等于原V4工厂；E/separate-pull-fixture-equivalence-v1.json，不称11文件全字节相同。元数据准备错误及原生红均在历史保留。

## 已完成且不重复的恢复与基础修复

**纯IPv6固定功能矩阵完成**：E/ipv6-only-mobility-verification-v2.json phasecompleted，5类各3次、15/15场、**630/630真实读取**；双向首次最大1705510μs，原2秒门。实际old移除/new UDP源、无非loopbackIPv4、IPv6 Relay TCP、路由/CPU/PSI/owner核验。Relay-only3场恢复约0.507–0.509秒，nft反射器放行/其他UDP封锁的正负控制及两侧Hop::Relay通过。

边界：UDP阻断在配对前，证明已建立Relay地址变化恢复，不是Direct遇突然UDP黑洞；IID变更允许add/DAD/delete重叠，不证明原子断旧。非NAT64/DNS64、新peer发布、睡眠、手机、peer relay自身移动或成本验收。详[IPV6-ONLY-MOBILITY](IPV6-ONLY-MOBILITY.md)。V4d6341a1/tree1cbe5c0d，producer043812；产品CLI443c6e010293b83e7c0e963b6eb96edb3897dd4c0d6ada0bca9f6fca02666a43不变；前4与后11仅nft literal修正，功能carryover已核、不合池性能。原E0308编译拒绝和第5场nft准备失败保留。

**N2旧拨号超时覆盖新认证联系修复**：TF96e999dae8312d86a38c08095ba58c3e1c1274e1/tree41df16ad，原14合同12绿2红→14绿；Linux286/Mac282原12组全过、各原ignored1。全局contact守Asleep、每协议握手代次守失败出口，保留失败记录/原期限/读写边界，无新timer/packet。S已包含；详[DIAL-TIMEOUT-CONTACT](DIAL-TIMEOUT-CONTACT.md)。Mac原磁盘49.66GiB拒绝编译0，按既有受保护回收规则修启动reserve判定后67.21GiB，同源43542867922通过；原失败不删除。

**I4最终前版**4517d4cb/treecd52c290完整Quality43523128168首轮13/13、native Mac Lab43523194833首轮4jobs/77cases通过；WebKit原四诊断后锁1.65.1/27.2，旧I3红保留。不是当前S的完整Quality、不是手机接受。F3 silent/healthy固定6场168/168成功、silent0.873–0.954秒；双端V4/NAT及dual IPv6固定6场252/252、首次0.507–1.674秒。证据见归档、[RELAY-DEMAND](RELAY-DEMAND.md)、[BOTH-ENDPOINT-MOBILITY](BOTH-ENDPOINT-MOBILITY.md)。

## 已拒绝方案与上游

- I4+N2 shared原完整ABBAAB对照8640读/12文件成功，但N3高延迟目录和PTY长尾回退，E/contact-current-mixed-verification-v1.json为RED。
- PULL4两场硬失败共4timeout/93blocked，停止，未采用；H1收益margin原unit发现Relay200ms错误清195ms hop，未Release；H2两场虽1440成功，但目录超固定门已足以futility，随后原baseline硬失败停止。全部数据/未执行人口保留。
- H2两有日志诊断selected-change均0，仍未复现无日志回退；不能将churn认定唯一根因，不能盲目增margin。
- 2026-11-14T05:37:22Z已在线核官方Release/docs.rs/crates.io JSON：最新稳定iroh1.4.1/noq、noq-proto1.4.0，未yank，与S候选一致。E/upstream-recheck-20261114/verification.json，详[UPSTREAM](UPSTREAM.md)。canonical feat/net产品仍iroh1.2/Noq1.3，未把实验升级合入，现装App也未更新。
- custom历史60项目前1已上游替代、2Noq移除红保留、1 iroh38移除红保留、56待逐项退出核查。E/iroh-upgrade-patch-ledger-v5.json只4条有详细结论，不能称余56均必要。

## 剩余完整目标与资源规则

N1多middle、置信/流量/成本；N2全部预算/取消/资源/写与0RTT/NET16负向；N3当前screen拒绝后的合理架构、初始extra dial可靠后退、caps/多文件公平/小文件首字节/1-3-5成本；N4新地址发布与冷peer、Direct突发UDP黑洞、peer relay移动、睡眠/手机换网、中继故障域/容量；N5全部custom退出核查与最终同源Quality/Mac/mobile及设计报告。不得用局部绿缩减REQUIREMENTS。

iPhone现装仍旧版，之前“就绪”过期，真实候选可用后另约窗口。MacLab与Turing同LAN，macserver是另一异网设备。builder colehank/khor-build public获明确授权；未授权App安装、发布或生产Relay部署。

本机source-only：不运行本地Cargo/Rustfmt/Node/Bun/Xcode/产品；Git与stdlib Python源码/数据/ZIP/hash及远端编排允许。Turing UID1110/shared flock/jobs16/nice10/Rust1.95/启动50停止40GiB、私有user/netns；计时CPU>=80%idle/pressure<=5/无同UIDCargo，与编译串行。Mac jobs2、原50/40GiB与target100GiB及installed guard998f保持。root唯一ledger writer，无后台协调器。

先读本STATE/PLAN，核实际HEAD/diff/共同ledger，不能照历史pending/PID重派。S图谱0节点、不覆盖准确树（不是低风险判断）；按图谱优先后定向读实际源码，不擅自索引。其他任务LIVE23961、main及已安装App保持。
