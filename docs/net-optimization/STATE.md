# feat/net 当前状态与接续入口

更新于 2026-11-13T12:41:53.615174+00:00。完整目标是[GOAL](GOAL.md)、[PLAN](PLAN.md)的N0–N5，**未完成**；逐项缺口见[REQUIREMENTS](REQUIREMENTS.md)。root串行，不派子智能体。

此前STATE逐字归档[HISTORY-20261113-through-current-noq-matrix](HISTORY-20261113-through-current-noq-matrix.md)，SHA256 `8ea6c57858cbf72d124e9ee81eb1fdc3a8c627deb23db89ea3aedb0b55667944`。它的pending/进程描述只适用于原时点，不能据历史重派；更早历史链接亦保留。本页是当前执行入口。

## 当前执行器与紧接工作

唯一pending是Direct/Relay配置隔离的普通Release compile-only：**turing-20261113T124033Z-8cfa87a1 / PID212526**，receipt=/Volumes/disk0/Khor/evidence/net/turing-20261113T124033Z-8cfa87a1/receipt.json，control **c94834f2c8c6949713eb5e2b538d88af9b36459f**。先核真实remote execution/PID；终态collect→identity→实际producer核验→ledger闭，不重派。source-refs由root唯一writer；旧本机20826/53952两个调度器均已实际退出，无后台协调器。

源在khor-net-direct-isolation-v1 / experiment/net-direct-isolation-v1，HEADd6b97c3d、2文件staged；准确tree **59639aac742bd404fc9bea1618f29345907ac238** / snapshot **5a41f54d37d8755cb54e31478e61a5cddff6c2f4**。endpoint.rs已逐字还原I117的BBR，其他产品runtime亦与I117相同；不含R的hop改动、W唤醒或M38移除。变化仅实验夹具及验证工具。E/direct-isolation-{source,design}-v2.json、E/direct-isolation-native-compile-plan-v2.json。原123643Z-635d197d 1/46.232886s为格式准备红，10files/identity/空owner闭，Rust/业务0；v2仅应用原导出格式补丁，原件保留。

问题是允许已配置/实际发布Relay候选，是否改变同一BBR下真实Node交互长尾。预冻normal/direct/direct/normal/normal/direct，6场、各原720记录/192MiB/2秒caller/600秒owned cap，替代0；原before/after Direct、文件/PTY/重叠/全部功能门保持。两个真实endpoint.json须存在且正确解析，normal发布本Lab真实Relay，direct_only不发布Relay。配置/发布不证明同时有两条活Noq copy，须保留journal并限定归因。Main用于目录/文件、Tunnel用于PTY本来就已分连接，不能声称新建这种分离。**仅夹具诊断，不建议在产品禁用Relay**。

编译器在改源前以原native_provenance复核并核存上批Noq model选中exe+dep-info；复用旧可信完整CUBIC producer110223Z-34e7344c与model120149Z-9c128417的实际unit key/哈希，原凭据不改。46项验证工具回归只在两份源SHA相同才复用，不再重复。producer闭后冻结实际runtime控制再计时；本配置隔离尚无runtime结果。

## 已闭Noq分量矩阵：不全局切CUBIC

**120549Z-13cd71dd 0/1115.477027s**，50files/identity/CPU/19个private user/net namespace及全部owned清理闭；19/19原native门，**1101/1101请求ok**、替代0。普通Release opt3/strip/thinLTO/codegen1，无qlog/diag，运行时编译0。协议0c3eaf80d795a78c3909f475ff285e015c483bbb8a12e3456c7ed8bdc76fa12a；全量E/cc-matrix-comparison-verification-v1.json，各run下cc-matrix-runtime-verification-v2.json。

300ms、20Mbit/方向、200ms队列、双向丢包固定0/2/2/0/0/2%，CUBIC/BBR各条件3轮。沿用原20秒bulk及typing→screen，新增10秒无应用请求→短流/screen；原Noq层10秒ask不是Node原2秒门的放宽。CUBIC/BBR吞吐中位：0% **16.507651/11.686758Mbit/s**；2% **0.325742/10.067335Mbit/s**。2% bulk echo p95 **763.900/455.850ms**；0% typing后screen **429.567/307.491ms**、quiet后screen **430.954/307.321ms**，这些screen各n9。CUBIC未过预冻分量screen，不能全局替换BBR。Noq模型中最大观测BBR1.283s/CUBIC1.407s，不是原Node 2秒超时的重现或根因证明；iroh/multipath/Relay、Node/IPC/同步/SSH/hop与能耗尚未由它覆盖。

原v1数据解析器只认行首marker，Rust libtest会把test-name前缀与首条stderr指标放在同一行，漏掉首CUBIC bulk/quiet行。v1原件/SHA保持；E/cc-matrix-parser-correction-v2.json预记确定性修正，v2收齐全部marker，原样本/时限/quantile/门/日志不改。原strict 2bulk/6quiet计数仍保持，全部原summary由raw重算。

该源现clean commit **d6b97c3d41d13fb1b9ef209adb89975bfdd727af**，tree **23b5d7e9d841e1411ba84a68db91ed1f95d7bcff**，worktree khor-net-cc-matrix-v1。其CUBIC endpoint仍为实验，不晋升。producer120149Z-9c128417 0/74.067894s、Rust18.97s，22files闭，testSHA7b8d34deba4507b1d96fe8c059e43bcc64c4ebae837c68c15d30a19d9e637fe7；原selected2文件保留。

构建工具修正是实际Cargo dev依赖不传播：请求test/example/bench保留自身dev边、所有normal/build边，typed/legacy IDs不一致、缺typed或未知kind拒绝；原source/actual row/dep-info/hash检查保持。原37+新增9回归在旧代码精确3fail/2error，在新代码46/0/0，真实普通Release亦获准。helperSHA7dbd2c0f4048f8cc96a3c9303565b6298c4999daccdd4fe360825d21d225ac7b；**这是明确修正后的验证器**，不能写成逐字未变。原5个准备/准入失败及未准入exe/dep-info归档完整见历史及原receipt，不作可信prior。

## 普通Node Release仍有未解决项

[COPY-WAKE-RELEASE](COPY-WAKE-RELEASE.md)固定18场/12960记录全闭：B6/6、4320ok；C6/6、4320ok，但目录尾延迟仍过原工程界限；W5/6、4200ok/2timeout/118blocked。W high在idle_before/bulk各原2秒TTY超时，low终端排队达数秒，拒绝作为修复；blocked不是网络超时。C本轮TTY成功不抹掉更早high2/3失败。qlog/native ACK各3绿亦不抹原红。

[CUBIC-ABLATION](CUBIC-ABLATION.md)普通Node固定3 high：111336Z-865645d6 1/504.032874s，2160ok、3文件完整/PTY关闭，但原门2/3（首轮direct_after=false），保留profile红。bulk目录/终端p95 351845/334029us，文件中位36.734944Mbit/s，较早C580146/350138us、24.307740改善只作非同时交错参考；不推广为根因或最终接受。当前Noq丢包矩阵进一步否决全局CUBIC方案。

## 地址变化与sole-middle的有限接受

[TUPLE-REFRESH-EXIT](TUPLE-REFRESH-EXIT.md)：固定R M M R R M六块/12native场景，744真实读全ok，180暖远端开流，替代0。R强属性3/3、零mismatch；删patch38 helper的M38强属性0/3、累计215 cached/actual mismatch，暖远端正确数0/30、30/30、0/30。当前R/M源、iroh1.4.1/Noq1.4.0的实测NAT合同下保留patch38，不能据Mac库双198绿删除。原30秒caller/120秒setup/45秒hold/240秒scenario/300秒owned保持。完整E/tuple-exit-nat-comparison-verification-v1.json，不覆盖全部IPv6、双端/middle移动、手机或能耗。

[LIVE-MIDDLE-RETRY](LIVE-MIDDLE-RETRY.md)：R9081b94d/treecf9d172b，原sp7三真实前置失败保留，修后070832Z-6b87e77c固定3恢复3/3，1013894/784788/878995us，总5秒不变，真实Direct首腿+Relay第二腿/held hint/marker/同PID/owner皆核。Linux wholehop92绿。仅有限Ask/Forced、无target road且首腿真开放时按原HOP_WITHIN3秒复查；后台/死首腿退避、TRIES2/trust/单跳/写不重放保持。只接受此窄恢复，不代表全部NET16/手机。

## 源、环境与权限

E=/Volumes/disk0/Khor/evidence/net；全部工作树位于/Volumes/disk0/Khor/worktrees/current/。canonical feat/net runtime仍5064fe2a、iroh1.2/Noq1.3，文档提交不代表集成。I117c39e5 clean，iroh1.4.1/Noq1.4.0，冻结公开main c3b098bd的apps/.github；原交互失败及目录尾延迟未解，不最终晋升。R9081b94d clean、不含W；W treef5faf2a2拒绝；M38 tree2d702718退出失败。原B/C及所有实验/源/日志保留。khor-build-net HEAD5d3691f2未移动，仅新增控制Git对象/refs分支。

本机source-only：Git、stdlib Python源/数据/metadata/ZIP/hash与SSH/GH编排；无本机Cargo/Rust/rustfmt/Node/Bun/Xcode或产品native执行。重型仅授权Turing/macserver/可信GH；khor-build已用户明确授权public。Turing UID1110、排他flock、jobs16/nice10/Rust1.95、50GiB准入/40GiB停止/90min outer、私有user/net namespace，无sudo/宿主网络改动。Mac实际guard50/40GiB、target100GiB，SHA998f5723eb447a1634411397c18a524e5ed0d50bd6dbd81001a16626219f64eb，不降；jobs2，仅已有安全inactive owned cache维护。MacLab/Turing同LAN，macserver另机/网络。

无main合并、正式发布/标签/TestFlight、已安装App替换或生产Relay采购部署授权。保护其他任务Lab23961；准确source/PID读中央STATE，不能复用历史PID。旧I Quality43348869616首次13/13、MacLab43355810513 attempt2及内ZIP/source/hash闭证只适用I，未本地执行/安装。物理iPhone仍旧0.2，新候选/真实换网及能耗未接受；有可用候选后再约新窗口。

## 全目标剩余

N1完整路径/共享测量/稳定与成本；N2总期限、取消/资源、读写与0RTT不重放、全部NET16；N3真实同步/预览/SSH/hop公平、背压取消、共享/分连接实测与1/3/5设备CPU/线程/字节/真实唤醒和手机能耗；N4双端/middle独立及同时IPv4/NAT/IPv6隐私IID/前缀、WiFi↔蜂窝/锁屏/休眠/发现发布、Relay故障域容量与布局；N5逐项custom、最终同源Quality/Mac/手机、设计/报告与回归均未齐。旧p1h包+33.970%/字节+37.908%不抹掉。ledger v5的60项=1上游替代、2Noq移除红保留、1 iroh38移除红保留、**56待退出审查**，不是56均必要。

[MOBILE-ADDRESS-AUDIT](MOBILE-ADDRESS-AUDIT.md)：iOS netwatch AF_ROUTE独立通知确在，App表面去重不证明漏地址；250ms连续chatter仅风险。最新稳定于2026-11-13T10:15:56Z官方registry再核仍iroh1.4.1/Noq+proto1.4.0，原JSON/hash见E/upstream-registry-recheck-20261113T101556Z/verification.json；最终交付前再核。所有阶段绿都不能当N0–N5完整完成。
