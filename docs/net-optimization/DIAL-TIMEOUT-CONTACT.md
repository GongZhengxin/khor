# 新联系与旧拨号超时：待验证边界

当前普通与冷early两个入口已真实复现旧超时覆盖新认证联系，原红及负控制见下方首轮结果；已有隔离产品修复草稿，**尚未通过修复验收**。

## 执行前诊断记录

以下为原执行前假设与草稿状态，保留其时间边界，不代表当前尚未执行。

I4准确源4517d4cb沿用I3/F3的Node逻辑。普通line_to_inner和冷early拨号各持每peer/protocol的dial mutex；等待者只共享其等待开始之后完成的失败。CallerExpired不登记peer失败；Asleep以contact_version同锁检查，不能覆盖该peer后来认证联系，且缓存Asleep也查该version。其他TimedOut/Refused目前不查同一contact version便登记failed，等待者也未因新contact丢弃其旧失败。因此要验证：旧拨号真实10秒无应答后，是否会覆盖过程中已成功认证并echo的同peer新联系、重建down/backoff并抑制排队调用。原六项Asleep测试不覆盖这个failure类别。

提前冻结E/stale-dial-timeout-reproduction-design-v1.json；独立experiment/net-stale-dial-timeout-v1，worktree khor-net-stale-dial-timeout-v1，基于4517d4cb。仅加peer_contact/timeout.rs和显式mod路径，原六项Asleep测试body逐字保持。固定ordinary/cold_early两入口×same_peer/none/other_peer三控制，共6项；对端真实ignore QUIC握手，不合成DialFailure、不改原10秒产品时钟。一个真实调用占锁，另一个实际poll后排队，再建立认证入向连接与双向echo；旧调用保留实际TimedOut，检查同peer新联系不应被盖掉、排队者应真实echo成功；无联系/其他peer控制继续共享真实失败。

草稿尚未远端rustfmt/编译/运行；E/stale-dial-timeout-draft-source-audit-v1.json明确tests_executed=0。Turing当前受正式计时对照独占，在其全部固定块完成前不执行新编译或夹具。接续先核STATE/source-refs，不按草稿派任务。若复现，先留原红，再设计最小状态所有权修复和原负控制；若不复现，保留结果，不能宣称修复。cold_early不等于已发送恢复0RTT数据，不补写重放验收。

另一个仅供后续custom审计的事实：普通拨号首个ep.connect已同时携带direct/relay hints；第二identity-only尝试在原1500ms后开始，最多两个slot，新carrier复用第二slot并先join被abort任务。journal的Won::Direct是初始尝试标签，不能当实际包路径证据。升级后能否去掉冗余尝试仍未实验，不能据新版接口介绍直接删除。

## 首轮真实原生复现：两红、负控制保持

标准net-node231847Z-7feaf11f已101/437.635519秒闭证，14files/identity/owned全核；准确a1fb52c7/tree003c9231，产品0改。net70（原ignored1）、hop92、wires16、journal30通过；dial_budget原46＋新6共52，50通过/2失败。失败仅ordinary/cold_early的same_peer；四个无/其他peer负控制与原六Asleep均通过。两失败原JSON明确contact版本0→1并保持1、旧owner actual TimedOut、排队者返回失败、down/backoff被重设，端点已显式关闭；原断言“an old timeout cannot erase newer authenticated contact”失败。这两条是已确认的新联系/旧超时边界，尚无修复。

E/run/stale-dial-timeout-reproduction-verification-v2.json保留完整原结果。原控制set-e在此停，后续IPC及六租约组共24预期项未执行，不能说标准全通过；通过新case的JSON被原Rust harness捕获，外部仅有其原生ok记录与两失败JSON，已明确记录此范围。v1的两条后续未执行log名称曾用旧记忆；v2从准确原控制派生命名，原v1保留，全部原生数据/计数/根因证据不变。

后续修复必须先补同peer不同协议的负控制，区分真正的新联系与仍然有效的协议失败；不能让Main持续流量无限掩盖Tunnel失败。保持原caller期限/有界拨号与Asleep保护，不添加全局rebind/写重放或周期重试。图谱先核main9a42旧结构的两调用入口line_to_inner/ask_early及failed影响；准确候选没有图谱覆盖，实际实现以候选源码和新证据为准。

## 协议控制、修复草稿及夹具v2

固定14原版235301Z-230e7e9b已101/196.613845秒闭证，12files/identity/owned完整；原2 red不变，另12通过，其中同身份新Tunnel两负控制通过。八个实际marker完整，原六Asleep body保持，E/run/dial-protocol-original-verification-v1.json。源码0bcde20d/tree18dae1e6，仅新测试几何增加双ALPN、原六timeout测试body保持，显式assert实际ALPN。

隔离fix/net-dial-contact-repair-v1起于0bc，产品提交d15c2d1d/tree5c9e7b44，仅link.rs与wires.rs。保留peer-wide Asleep版本，增加每协议新握手generation；普通流量不递增该握手generation。typed DialContact参与两个入口缓存判旧，三个失败出口都guard；实际失败计数/history保持，只有仍有效失败可写backoff/down/清旧served。新contact和负状态提交采用slots→presence→down，关闭served在状态锁外。无新timer/packets/tasks、原请求/拨号时钟、读写/0RTT边界与PULL8保持。E/dial-contact-repair-design-v1.json与source-audit-v1.json；图谱main旧结构先核caller/impact，准确新worktree无覆盖（detect0不能当低风险/已审通过），已定向读锁序与完整diff。

修复首轮001134Z-d8dc2ab1 **101/206.730660秒**完整保留，12files/identity/owned闭。12过/2失败，但两正向场景失败在测试服务端对旧已取消握手的accept().await.unwrap，而非原down断言；仅6/8marker，无业务修复接受。原调用真的开始下一拨号后，旧握手取消是正常传输事件，串行fixture不应因此panic或挡住下一请求。E/run/dial-contact-repair-fixture-failure-v1.json。

提前冻结E/dial-contact-fixture-cancellation-v2-design.json后，仅改测试服务端为cap8 JoinSet并发，记录取消/关闭/容量，shutdown信号后abort并join所有handler，再关闭端点、检查原所有断言。原10秒产品timer、5秒setup/queued、15秒owner watchdog和14测试body不改。原版5066bfb9/tree49acdc35/snapshotc45c6af3；同一fixturecherry-pick到修复版96e999da/tree41df16ad/snapshot1cc2cb53，产品代码逐字d15，所有peer_contact源码在两树逐字相同。formatter v1因孤立src缺tests/util解析失败保留，v2仅远端stdin/stdout格式化两源；后续原控制cargo fmt --all --check仍保持。无本机Rust/产品运行。

当前原版fixturev2唯一run002222Z-fb33f5b6在途，控制27ff89e4d3df21b44c4f1a36cebc1d43d5ac6347/原固定14。必须先核它仍有原2 red且其余12过、8marker/全handler清理，再调度相同fixture的96e修复。不能把原版前一夹具的红与修复v2混为同fixture因果证据；两旧失败都保持。完整标准Node和Release/最终Mac/真机仍未补齐。
