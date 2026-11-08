# 验收协议 v1（N0，策略修改前）

2026-11-09。实现前冻结本文件的行为要求；实际运行状态与证据索引写入 STATE。

## 首批选路验收

- 单元：对 Direct / Carried / Relay 的所有配对，90 ms 与 5 ms 的有效测量中始终选择 5 ms；从活的慢路和刚切换的慢路都允许显著改善，快路不能被类型较优的慢路抢走。
- 真实网络 p6：设备各有独立 Strict NAT 的默认出口，另共享一条可直连的网络。中继设备单腿配置 40 ms RTT（端到端经过两次，约80 ms），共享网直连分别为约 300 ms 或局域网速度；两种场景应分别稳定使用中继和直连。
- p6 每例等待 10 秒供候选建立，再给最多 15 秒达到目标；连续 30 个目录请求（200 ms 间隔），零错误，p95 < 200 ms，观察期末两端仍在目标路径。报告样本量、p50/p95/max 和路径。30 例不报告 p99。
- 现有 p2（打字时慢直连回局域网）保持 10 秒内、无连接退役；p3 黑洞的最慢请求保持 1 秒内、无退役。先验证执行环境及这些基线，失败保留并诊断，不能删除或扩大原目标。
- 所有配对、暂停、only-mine 和 0-RTT 原有安全回归继续成立。

策略层初版使用所有路径一致的相对/绝对收益门槛，调用 iroh 的新鲜样本确认；不在此预先定死最终参数。每次参数改变记录对这些目标和 p1/p1j/p1s、p5 的影响，选择有对照证据的版本。

## 对照、重复与结果归属

- 先在未改策略的源码上运行新回归，取得预期产品红；编译/工具/执行器错误独立记录，不算复现。
- 然后对相同场景、工具链与负载运行候选；p6 与恢复场景至少 3 个独立重复。记录每次结果，不仅保留最好一次。
- 固定好路径作为可达到的参考：后续测量需在同一拓扑中单独屏蔽其他承载，不能把另一台机器的 ping 充作基线。
- 正常性能对照顺序运行，CPU 忙测试显式施加负载；执行器被构建挤占的结果需注明。
- 后续工作包的新增数值目标在其修改前按已取得基线补充；缺失的全矩阵、真机能耗和中继故障域测量仍是未完成项。

## 远端入口

控制仓库独立分支 `colehank/khor-build:feat/net`，通过现有 `development.yml` 的 Linux `network` job 执行；Mac job 对网络任务跳过。任务：

```sh
python3 scripts/dev.py check --task net-unit --builder-ref feat/net
python3 scripts/dev.py check --task netlab-smoke --builder-ref feat/net
python3 scripts/dev.py check --task netlab-compare --builder-ref feat/net
python3 scripts/dev.py check --task netlab-dynamic --builder-ref feat/net
python3 scripts/dev.py check --task netlab-hop --builder-ref feat/net
python3 scripts/dev.py check --task net-node --builder-ref feat/net
python3 scripts/dev.py check --task net-vendor --builder-ref feat/net
python3 scripts/dev.py check --task netlab --builder-ref feat/net
```

`net-unit` 验证格式与 khor-net 单元；`netlab-smoke` 的初版覆盖 p2/p3/p6，control d7d1af1 起加入 f4/f4s/f4i，显式2个场景并行用于功能回归；正式性能对照仍用单场景compare。`netlab` 完整矩阵保持原8并行。Linux 上先实测 user namespace、netem、nft 的能力再编译。源码使用原有 parentless snapshot + tree 校验，结果下载沿用 `dev.py result`，control commit 也写入回执。控制分支未经检查不得冒充已落地的默认入口。

`netlab-compare` 将 p2/p3/p6（含两个固定好路径参照）以单场景并行度1顺序重复3次，全部结果保留，即使baseline某轮产品红也继续其余重复。固定参照在同样双路拓扑完成准备后阻断竞争路径，再进行同样10秒预热和30次请求；Relay参照阻断唯一共享直连腿，Direct参照阻断中继数据面。参照是稳态可达到值，不计为首次拨号或故障切换性能。两版使用同一fixture，生产源码仍分别为旧/新策略。

control781a907增加`netlab-hop`，同一f4/f4s/f4i fixture按单场景并行度1顺序三轮，保留每轮失败；不改其网络/门槛。`net-node`补跑journal单元，验证path事件诊断字段可读旧journal且不改变显示句。221b98fc起path事件附缓存的传输RTT与选路quality/age/count，只在已有事件读取，不新增timer或探测；这是事件处理时的连接缓存快照，不冒充选择器内部决策时刻。

## 动态恢复目标（p7，修改采样实现前冻结）

与 p6 相同拓扑，先确认两端都由约80ms端到端中继承载（设备单腿配置40ms），再预热10秒，将300ms直连恢复为局域网速度。持续200ms间隔目录请求时，两端10秒内回到直连且无请求失败；闲置时20秒内回到直连。观察最多45秒并记录实际恢复时间，包括超限/未恢复，不能只报告通过者。

专用 `netlab-dynamic` 以 `NETLAB_TEST_THREADS=1` 顺序运行 p7，避免普通性能验证夹入其他场景的CPU负载。`NETLAB_TEST_THREADS` 默认仍为8，完整矩阵保留原有并行压力；报告必须写清执行模式，不能混用数字。

control c6a20c5 起，`netlab-dynamic` 将 p2/p3/p7 一起顺序重复3次，覆盖变慢、黑洞及好路恢复；老control的动态任务仅含p7。具体filters/线程数/重复次数由每run的measurement-mode.json固定，恢复时按准确版本解释结果。

2026-11-09 几何核对：`lab::slow(relay,40ms)` 给经过中继设备的单腿设置40ms往返，设备A→中继→B端到端约80ms。早期结果行“relay40ms”是设备配置值，实际路径读数84/88ms与此一致。已改标签和说明，拓扑、负载、200ms请求门槛及10/20秒恢复目标均未改变。

## p2协议纠错 v2（改测试前冻结，速度优先）

原p2的Relay没有延迟注入，LAN与Relay都可能约1ms；把“必须回LAN”作为通过条件与用户速度优先决定冲突。旧版结果全部保留，未回LAN不能单独证明请求恢复慢，旧日志缺逐次延迟分布也不能回填为新协议通过。p3的真实请求卡顿独立有效。

保持原拓扑、预热、约21ms慢腿注入与10秒收敛要求。新通过条件：故障前至少一端确在将变慢的second腿（否则是准备失败）；注入后两端10秒内进入已知快路径集合（未注入延迟的LAN或Relay），不能仍使用second腿。没有连接退役。

v2首次执行前补充审计：Link.rtt_ms是各连接的旧平滑RTT，缺样本年龄，不能拿它当新鲜RTT设门；曾拟的≤10ms门不采用。路径是否在快集合由本实验的明确拓扑决定，实际用户体验由下面的逐次请求延迟/成功率证明；平滑RTT保留为诊断值，不冒充新鲜质量。这个纠错在任何v2实跑前记录，旧记录不回填。

前台p2持续200ms间隔请求，包含收敛过程并在第一次达到快路径后继续至少30次请求；零错误，最坏请求<1秒（沿p3体验上限），全部样本p95<200ms（沿p6），结束两端仍在快路径。超时/错误保留进样本与结果，报告路径、收敛时间、样本量/p50/p95/max，不只报接口名。新增延迟要求比旧p2更完整，不放宽10秒/零错误/零退役。

p2i仍是原闲置观察项，记录相同快路径收敛与退役，无额外业务请求，不把观察red自动转成测试pass证明。两版正式对照需复制完全相同v2 fixture后重跑；旧LAN-only结果不替代新协议证据。

## p7相位扩展（新增fixture前冻结）

为覆盖闲置恢复19秒附近的裕量，在原10秒预热后增加故障注入相位偏移：交互0/2/4秒，闲置0/5/10/14秒。每个相位新建相同Lab，不改变路径、请求间隔、45秒观测上限或交互10秒/闲置20秒门。变化前再次确认两端确在Relay，否则记准备失败；不把未打中的变化算成功。

七个独立Rust case均由p7_前缀纳入netlab-dynamic：每轮单场景顺序、三轮共21个p7观察；另有原p2/p3，各轮保留失败。结果写明相位，不能将同一VM的21个案例称为21台独立机器。扩展后的完整矩阵是40项，扩展前35项结果仍按原源码解释。此扩展只增加覆盖，不推断探测开销已达标，也不由老phase0数据回填其余相位通过。
