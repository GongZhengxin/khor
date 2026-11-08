# 网络验证运行账本

2026-11-09。证据根 `/Volumes/disk0/Khor/evidence/net/`；每个目录的 source.json / environment.json 才是准确源码和控制器回执。失败按产品、准备、格式或执行器分类。历史源码通过不覆盖后续修改。

| Run | 任务 / 源码 | 结果与适用边界 | 证据子目录 |
| --- | --- | --- | --- |
| 42505117705 | 旧策略 + 速度回归，net-unit | 产品红，60 pass / 1 fail / 1 ignored；90ms Direct 胜过5ms Carried | baseline-unit-42505117705 |
| 42505672250 | 首版候选 net-unit，tree b66d0343 | 61 pass / 1 ignored；不含后续所有 bind 注册 | candidate-unit-42505672250 |
| 42506109052 | 首版最终 net-unit，tree d0fdb004 | 62 pass / 1 ignored，含统一 bind 与 settling | final-unit-42506109052 |
| 42505323213 | 初次 baseline smoke | p6r 产品红；p6d 反向 wire 未建立，是准备红；p2/p3 绿 | baseline-smoke-42505323213 |
| 42506330051 | 修正 fixture baseline smoke，tree e73903a3 | p6d p95 34.02ms绿；p6r p95 369.78ms、Direct311/309ms产品红；p2 3.1s、p3最坏846.1ms绿 | baseline-smoke-42506330051 |
| 42506345592 | 同 fixture candidate smoke，tree ee158fdb | 4 pass；p6d p95 64.96ms；p6r p95 152.12ms、Relay84/88ms；p2 1.2s、p3最坏546.6ms | candidate-smoke-42506345592 |
| 42506805985 | hop 看守新增回归，net-unit | 产品红：relay30ms/hop22ms，固定10ms看守错误释放有价值hop | hop-unit-red-42506805985 |
| 42507205671 | hop 候选 net-unit | 仅格式失败，patch已应用 | hop-unit-42507205671 |
| 42507239186 | 同轮 net-node | 待查询详细分类；不得记为产品红 | 尚未下载 |
| 42507643289 | hop/p7候选 net-unit | 仅 p7 assert 格式失败，patch已应用 | hop-unit-42507643289 |
| 42507629732 | 同轮 net-node | 待查询详细分类；不得记为产品红 | 尚未下载 |
| 42507391687 | p7动态，tree 9ffe76e4 | 产品红：交互与闲置两场景均45s未回Direct；0请求错误；尚未含新采样修复 | dynamic-red-42507391687 |
| 42508095178 | net-node，tree 067d62b5 | SUCCESS，hop9 / wires10全部通过；不含随后vendor采样改动 | hop-node-42508095178 |
| 42508069745 | net-vendor，tree 067d62b5 | 执行器准备失败：excluded依赖不能从根workspace跑dev-dependencies；控制器已改为独立manifest + committed lock | vendor-unit-42508069745 |
| 42507224815 | 完整旧策略baseline，tree e73903a3 | 观察时仍在运行；恢复先查询，不重复提交 | 尚在途 |

新采样候选：同tree `064925806d1c3962f715942d7f637a4b6257783f`（含全部新增文档，无排除untracked），base bc6cded2，control `16474c5`：

- net-unit **42509122554**，snapshot `3c88d2f67ff691431427cbd3153b76d67d1abf66`。
- netlab-dynamic **42509155432**，snapshot `41fdf5a93ef5fc717145abd87c6b218adce2b130`。
- net-vendor **42509174050**，snapshot `301ecbca4134be116d877ceac658f70545bc92c1`。
- 当前均需查询结果，未记通过。source-only脚本轻量测试16项通过。

p6网络配置中的 relay40ms 是单腿配置；完整 A→relay→B 往返约80ms，实际84/88ms。已修正新日志和协议标签，未改变拓扑或200ms目标。前述A/B仅一次，四场景并行、不同托管VM；只能作为首轮证据，不能声称统计显著或替代重复/固定好路径参照。
