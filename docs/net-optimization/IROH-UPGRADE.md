# iroh 稳定版升级与补丁退出

2026-11-12，按用户U18实施。隔离树`khor-net-iroh-upgrade-v1`，分支`experiment/net-iroh-upgrade-v1`，运行时基于已验整合候选dc73837e；canonical feat/net仍未升级。

- 官方稳定iroh1.4.1，配套Noq1.4.0，发布源码来自已保存且SHA256匹配的crates.io包；源清单在E/upstream-20261109/source-manifest.json。
- 以旧发布包为共同基线三方迁移，保留原专属回归，并保留新版阻塞Relay发送不阻塞Direct Initial的回归。E/iroh-upgrade-source-merge-v1.json记录逐文件动作和7处冲突。
- 冲突决议：保留本地速度选路与上游Initial发送任务并存；保留原pending队列硬上限与path-ID释放事件，未以简单dedup冒充等价；Noq统计保留本地公开遥测；vendor iroh锁中Noq-proto继续本地来源。
- 补丁22的IPv6字面地址QAD改用上游relay_tls_server_name；域名/IPv4/IPv6的上游测试与真实IPv6 QAD测试保留。此为源码替代，已有新源库门验证，完整产品退出见下述边界。
- 其余补丁先作为迁移候选保留，逐项按UPSTREAM/VENDOR-AUDIT原合同决定复用、必要移植或删除。不把三方merge成功当语义验证，也不把原60项全部标成仍必要。
- workspace、net/node的iroh及直接Noq测试依赖统一要求1.4系列；锁文件必须由Turing生成，禁止本机Cargo。
- 准备轮允许远端生成锁/formatter patch并报告编译不兼容，产物不算冻结原源码验收；应用补丁后再冻结源码、运行原回归。不得修改原性能冻结源或原失败日志。
- 完整N0–N5、速度优先、完整peer relay两腿、移动IPv4/IPv6变化及无需重启自动接管要求不变。当前没有发布、main合并、App替换或新真机验收。

## 当前证据

- Turing冻结源02807c79 / tree f87d31f9 / snapshot dc21d783，control bd13e1e7，run turing-20261112T084724Z-35d6dfe3原0/226.384657s。17原files/SHA/source/bundle/原group空核验；Noq全lib467过/零ignored，iroh全lib197过/原flaky ignored1，node lib/tests编译过，三个锁与冻结源逐字一致，post-check源码diff为空。
- 原Noq双方报告both的真实0-RTT专属回归通过。iroh上游relay_tls_server_names、qad_probe_accepts_ipv6_literal_relay_url、blocked_relay_does_not_delay_direct_initial命名测试均通过。QAD IPv6测试有地址不可用提前返回，不能把普通ok独立扩称IPv6 WAN/手机通过。
- 补丁22手工trim已消失，上游helper与两专属测试函数体忽略空白后逐字相同；库层替代已验证。E/iroh-upgrade-patch-ledger-v2.json记录60历史条目中的1项上游替代、59项仍待等价/退出审查，不宣称59全部必要。
- 没有改动net/node的Rust源码；四份原source-owned wire元数据脚本相对dc73837e逐字不变。依赖变化仍须新源应用回归，旧性能结论不自动迁移。
- 应用固定12 gate：turing-20261112T085339Z-ab57a92e（在途），source02807c79/control68cadb57。
- macserver库门：43186999295（在途），相同tree f87d31f9、snapshot82206b8f/controlbd13e1e7；独立Mac共享guard，首bootstrap已通过。不是正式发行或App替换。

## 退出与全库诊断的后续边界

- Mac库门43186999295已收件核验：Noq467/iroh197及原ignored1与Linux名单一致，node lib/tests编译过；三锁与冻结树一致，源码diff空。16原files/11controls，stable-upgrade-gate-verification-v1.json。
- 固定12 Turing 085339Z-ab57a92e 273过/原辅助ignored1；36真实流合同090144Z-223b1901完整通过，独立74Fetch RPC/10peer及4 frozen metadata重算；人口相互重叠，不累加。
- 完整1387 node新源诊断090450Z-bfc82a31原101，1274过/38失败/75原ignored。20项明确旧Git不识别初始分支参数；还有CLI/agent/native等前置与其他失败。旧源dc738同control完整对照092140Z-5216816b正在跑。全库driver尚未准备Quality原生节点套件的完整前置，不能把原红直接归上游，也不能冒充全库绿。
- Noq5删除对照091835Z-cb319e57：4原fixture三过一红，已合法分配/回收的ACK-only最大号加更低需ACK包，误增加RTT样本5→6。只删除生产保护、fixture/lock相同；Noq1.4基线同四测试均绿。此保护保留。证据iroh-noq-ack-exit-comparison-v1.json。
- 60项历史账目v3明确1项上游替代库验证、1项保护由ablation支持保留、58项仍待逐项退出审查。完整设计目标保持开放。
