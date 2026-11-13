# 补丁38退出的实际验收边界

问题是当前 iroh1.4.1/Noq1.4.0 是否已能正确跟随实际 NAT 远端 tuple，允许删除本地定期 reconcile helper。版本号和库测试通过都不能单独回答。

两臂是 R（tree cf9d172b）和仅移除该 helper、保留 select_path_on(true) 的 M38（tree 2d702718）。两臂不含 W 唤醒消融。Mac 完整 iroh 库对照已经结束：各198通过、0失败、原ignored1、0filtered，199项名称与结果一致；证据为 E/public-mac-tuple-exit-{baseline,disabled}-v3 中的准确源码、产物及核验记录（E=/Volumes/disk0/Khor/evidence/net）。这只满足库检查，尚不批准退出。

真实 Linux 验收沿用 `/Volumes/disk0/Khor/evidence/net/tuple-refresh-exit-plan-v1.json` 的原 r0/r1 和强 oracle。两臂必须先有相同 net-diagnostics feature 的真实 netlab 与实际 CLI producer；原 native_provenance、源/上下文/ELF/dep-info 核准全部保持。编译不与性能计时争用 Turing。

原物理夹具 `crates/cli/tests/netlab/paths/rebind.rs` 在R/M38两臂的SHA256已核，必须保持为 `ac97726906429a5dd63dd91b5c9ed99624c273d9a253f7facb916ffcf47e8a9b`。先核 r0 的真实外部端口变化，再执行 r1；不能用设置过规则、缓存状态或最终一次成功替代注入与业务。

固定 R M M R R M，每臂3个新场景，不挑绿、不替换。原124次读及30秒caller、120秒setup、45秒Direct保持和240秒场景上限不变。`scripts/netlab-nat-selection.py` 的 `warm_remote_requests_select_the_rebound_tuple_v1` 还要求：接口IPv4/IPv6集合不变，conntrack删除和实际50001→50002映射成立；全部124逻辑读成功；30次暖远端 stream-open 全部选中新 tuple、cached/actual 不匹配为零，并有新 tuple 的 journal 见证。r1 原Rust门没有直接断言这个选路属性，必须另核oracle。

此页不声称已经运行上述 Linux 对照。runtime control、producer哈希和实际批次句柄仍须在执行前落实到 STATE。任一资源/所有权/注入前置失败记未完成；产品失败原样保留。若 M38 通过，只支持在所测 R 源上退出这一补丁；整合 W 或其他运行时变化后，仍须核最终源的相关行为。

它不覆盖独立IPv6隐私地址/前缀、双端与middle同时移动、真实iPhone或能耗；这些仍属于完整N4/N5。
