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
python3 scripts/dev.py check --task net-node --builder-ref feat/net
python3 scripts/dev.py check --task net-vendor --builder-ref feat/net
python3 scripts/dev.py check --task netlab --builder-ref feat/net
```

`net-unit` 验证格式与 khor-net 单元；`netlab-smoke` 的初版覆盖 p2/p3/p6，control d7d1af1 起加入 f4/f4s/f4i，显式2个场景并行用于功能回归；正式性能对照仍用单场景compare。`netlab` 完整矩阵保持原8并行。Linux 上先实测 user namespace、netem、nft 的能力再编译。源码使用原有 parentless snapshot + tree 校验，结果下载沿用 `dev.py result`，control commit 也写入回执。控制分支未经检查不得冒充已落地的默认入口。

`netlab-compare` 将 p2/p3/p6（含两个固定好路径参照）以单场景并行度1顺序重复3次，全部结果保留，即使baseline某轮产品红也继续其余重复。固定参照在同样双路拓扑完成准备后阻断竞争路径，再进行同样10秒预热和30次请求；Relay参照阻断唯一共享直连腿，Direct参照阻断中继数据面。参照是稳态可达到值，不计为首次拨号或故障切换性能。两版使用同一fixture，生产源码仍分别为旧/新策略。

## 动态恢复目标（p7，修改采样实现前冻结）

与 p6 相同拓扑，先确认两端都由约80ms端到端中继承载（设备单腿配置40ms），再预热10秒，将300ms直连恢复为局域网速度。持续200ms间隔目录请求时，两端10秒内回到直连且无请求失败；闲置时20秒内回到直连。观察最多45秒并记录实际恢复时间，包括超限/未恢复，不能只报告通过者。

专用 `netlab-dynamic` 以 `NETLAB_TEST_THREADS=1` 顺序运行 p7，避免普通性能验证夹入其他场景的CPU负载。`NETLAB_TEST_THREADS` 默认仍为8，完整矩阵保留原有并行压力；报告必须写清执行模式，不能混用数字。

2026-11-09 几何核对：`lab::slow(relay,40ms)` 给经过中继设备的单腿设置40ms往返，设备A→中继→B端到端约80ms。早期结果行“relay40ms”是设备配置值，实际路径读数84/88ms与此一致。已改标签和说明，拓扑、负载、200ms请求门槛及10/20秒恢复目标均未改变。
