# 新联系与旧拨号超时：待验证边界

当前仅为静态假设和测试草稿，**没有产品修复、没有新原生测试结果**。不得把下述描述计作已确认缺陷。

I4准确源4517d4cb沿用I3/F3的Node逻辑。普通line_to_inner和冷early拨号各持每peer/protocol的dial mutex；等待者只共享其等待开始之后完成的失败。CallerExpired不登记peer失败；Asleep以contact_version同锁检查，不能覆盖该peer后来认证联系，且缓存Asleep也查该version。其他TimedOut/Refused目前不查同一contact version便登记failed，等待者也未因新contact丢弃其旧失败。因此要验证：旧拨号真实10秒无应答后，是否会覆盖过程中已成功认证并echo的同peer新联系、重建down/backoff并抑制排队调用。原六项Asleep测试不覆盖这个failure类别。

提前冻结E/stale-dial-timeout-reproduction-design-v1.json；独立experiment/net-stale-dial-timeout-v1，worktree khor-net-stale-dial-timeout-v1，基于4517d4cb。仅加peer_contact/timeout.rs和显式mod路径，原六项Asleep测试body逐字保持。固定ordinary/cold_early两入口×same_peer/none/other_peer三控制，共6项；对端真实ignore QUIC握手，不合成DialFailure、不改原10秒产品时钟。一个真实调用占锁，另一个实际poll后排队，再建立认证入向连接与双向echo；旧调用保留实际TimedOut，检查同peer新联系不应被盖掉、排队者应真实echo成功；无联系/其他peer控制继续共享真实失败。

草稿尚未远端rustfmt/编译/运行；E/stale-dial-timeout-draft-source-audit-v1.json明确tests_executed=0。Turing当前受正式计时对照独占，在其全部固定块完成前不执行新编译或夹具。接续先核STATE/source-refs，不按草稿派任务。若复现，先留原红，再设计最小状态所有权修复和原负控制；若不复现，保留结果，不能宣称修复。cold_early不等于已发送恢复0RTT数据，不补写重放验收。

另一个仅供后续custom审计的事实：普通拨号首个ep.connect已同时携带direct/relay hints；第二identity-only尝试在原1500ms后开始，最多两个slot，新carrier复用第二slot并先join被abort任务。journal的Won::Direct是初始尝试标签，不能当实际包路径证据。升级后能否去掉冗余尝试仍未实验，不能据新版接口介绍直接删除。
