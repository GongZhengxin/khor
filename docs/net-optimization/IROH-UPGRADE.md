# iroh 稳定版升级与补丁退出

2026-11-12，按用户U18实施。完整目标仍为[PLAN](PLAN.md) N0–N5，当前执行入口见[STATE](STATE.md)。本页描述升级候选与已验证边界；旧版、全部原红及冻结性能源保留。

## 候选和迁移

隔离树 `khor-net-iroh-upgrade-v1`，分支 `experiment/net-iroh-upgrade-v1`，冻结源 **02807c79e04a92c46fb2e6c14d2ecf107b98336c** / tree **f87d31f97b89474fe52297d87e3827de33550493**。
使用已核对的最新稳定 **iroh1.4.1 + Noq1.4.0**，根及vendor锁一致。Canonical feat/net仍为旧运行时5064fe2a（iroh1.2.0/Noq-proto1.3.0），尚未合入本候选。

- 以旧发布包为共同基线三方迁移，7处冲突的逐文件记录为E/iroh-upgrade-source-merge-v1.json。保留本地速度选路、pending队列硬上限/路径ID释放重开、原专属回归，并保留上游并行Initial发送与blocked-Relay回归。
- 发布包均与registry SHA256一致。解包目录再次逐文件对照原包：iroh85文件、Noq-proto70文件，无不同（E/upstream-20261109/published-directory-audit-20261112-v1.json）。未将修改后的vendor当作上游源码。
- app net/node Rust和四份wire metadata相对旧整合源dc73837e逐字不变，E/iroh-upgrade-runtime-diff-v1.json。依赖变化仍改变运行时，旧性能不自动适用。
- Turing生成锁/formatter的准备轮不是原冻结源接受。首编译红与共享source保护拒绝均保留；原准备目录完整改名保存，未reset/删除。后续冻结源才执行下列回归。
- 本机source-only；重型验证Turing与macserver/private khor-build。无本任务main合并、发布、生产Relay采购部署或已装App替换。

## 已完成的候选验证

| 范围 | 精确run与结果 |
| --- | --- |
| Linux全vendor库门 | 084724Z-35d6dfe3，0/226.384657s；Noq467过，iroh197过/原flaky ignored1；node lib/tests编译、三锁、post-check源码diff均通过 |
| Mac同树库门 | 43186999295；与Linux相同命名人口及原ignored1，16原files/11controls核验 |
| 应用固定12 gate | 085339Z-ab57a92e，0/385.723121s；273过/原identity helper ignored1，与旧dc738人口相同 |
| 36真实流合同 | 090144Z-223b1901，0/31.781419s；4准确冻结解析器重算，独立74Fetch RPC/10peer；与273有重叠，不累加 |
| 实际CLI取消两合同 | 095017Z-a01289cd，0/186.335657s；2795源文件/CLI prior→node provenance/选中binary核对；两parent及两声明helper，6不同任务的Cancel IPC到达，4/24owner退出无遗留；控制Python sidecar，非真实agent bundle/WAN |
| netlab目标编译 | 100217Z-d286d5bb，0/120.570580s；locked compile及artifact/binary SHA/源码diff通过，runtime0，不称native_provenance接受 |
| 静默first-action诊断 | 100747Z-15afeb63，0/332.109970s；2 guards/6fresh读，12准备/12完整≥10s静默/12真实STREAM阳性；普通性能样本0 |
| 实际接口换网三场景 | 102059Z-2a7ba71e，0/186.422335s；dual/v4-NAT/v4→v6，372逻辑请求/744起止全ok、非移动端先读、内核地址独立核对；未证明实际QUIC端口重绑定/同连接/手机/完整N2 |

所有Turing run前缀为turing-20261112T；完整source/control/bundle/原文件hash/进程组空与行为proof分别保存在E/run目录，ledger终态。单次换网读取不构成p99/SLO或能耗结论。IPv6 QAD unit含地址不可用提前返回，普通ok不独立证明IPv6 WAN。

## peer relay 新红与确认

新源 **105138Z-c9de2969** 原101/150.987066s：第一笔setup读取10.039537s超时，**尚未注入middle换网**；16原files和准确attempts核验，原失败保留。
旧dc738同control69d23d4f的 **110236Z-baeba5f5** 原0/213.826096s：124请求全ok，middle实际IPv4变化，变前carried连续报告≥10s、变后再观察carried。
新红启动load90.18/128逻辑CPU，旧绿9.78；这只是差异，不能据一次对照归因上游或负载。新源原样确认 **110840Z-1e90b3dd** 已0/339.290546s，124请求全ok且变后再观测carried；源/控制/原hash/进程组空/行为已核验。其启动load144.61比首红更高，不能以单一host load解释。三轮及5份完整相同fixture绑定在E/iroh14-peer-mobility-comparison-v1.json。**首连间歇红未定位/未豁免**；不再无假设原样重跑挑绿。IPv6 overlap两合同111507Z-d7912aa7已0/165.659413s，248请求/496起止全ok、内核地址/隔离/remote-first核验；同前缀替换，不称手机/同连接或完整N2。实际QUIC NAT端口合同111859Z-de624e0b也已0/99.774935s：124请求全ok、30/30 warm远端stream选中新tuple、mismatch0，两个frozen metadata/原source/hash/进程组空核验。未证明同连接/逐packet/手机。下一步独立冻结新栈同条件混合负载对照，唯一状态见STATE。

## 补丁退出与原失败

E/iroh-upgrade-patch-ledger-v4.json的60项历史账目为：**1项上游替代库验证、2项保护由删除对照支持保留、57项待逐项退出核查**。57不代表全部必要；各项原原因/覆盖/退出合同见[UPSTREAM](UPSTREAM.md)和[VENDOR-AUDIT](VENDOR-AUDIT.md)。

- iroh22手工IPv6字面地址trim已替换上游relay_tls_server_name；helper与两测试体忽略空白相同，新源库门通过。未扩称手机/WAN。
- Noq5删除源e69b137c：091835Z-cb319e57原101，4未变fixture三过一红，合法已分配/回收ACK-only最大号加低号需ACK包，误增RTT counter5→6。基线Noq1.4同四测试绿，保护保留；这是状态unit模型，非wire性能。
- Noq2删除源79d2ca64：093340Z-deb1f7de原101，双方reporting保持both、服务端握手响应前发STREAM时出现不允许的0-RTT ObservedAddr TX1；基线同fixture绿，保护保留。两删除源不晋升。
- 新/旧完整node诊断090450Z-bfc82a31 / 092140Z-5216816b均1387人口、1274过/同38失败/75原ignored，无新独有失败。20项直接观察Git2.25.1不支持initial-branch；CLI/agent/native等前置不完整。两CLI合同已另按正确前置通过，**整套诊断仍非Quality完整接受，也不豁免其他原红**。
- 首CLI诊断094221Z-91240bdf控制漏声明合法helper导致严格metadata拒绝；原失败保存，v2只补控制声明与attempt持久化，未改产品或断言。
- Mac退出43189070840为50GiB启动余量门拒绝、零产品测试。后续只读库存43205919189又被GitHub付款/额度在prepare前拒绝，storage skipped、零Mac操作；已请求恢复额度或常规SSH，Turing继续。不降门/清未知缓存。

## 仍需完成

peer relay前置红定位、实际QUIC NAT端口及IPv6重叠恢复、新栈同条件负载体验对照、其余补丁语义/退出核查、完整原生Quality与Lab/真机/能耗验收仍开放。官网BLE/Tor/Nym扩展未接入本轮；上游custom transport不替代khor的两腿选择、信任或自动接管。
