# 高BDP共存qlog诊断（已闭）

本批只诊断，不是普通Release性能接受。原[Release对照](WORKLOAD-RELEASE.md)中的117c39候选仍因high两次失败而被拒绝。

预先固定三次fresh high病例、替代0；源码Q tree d372d293f69f4980bc0926797bf99f99ffa60f3a，snapshot 7e328dc5d000873f1cca809683fa41c0fa472e99，base117c39。仅启用上游optional qlog、隔离trace目录及壁钟锚点，没有改算法/重试/业务期限。

编译045808Z-c6558108已0/617.837255秒，原native verifier核Release opt3、实际qlog feature、20个first-party unit与同producer CLI/test；19原files及source/hash/identity/group空均核。先前引用前置失败045344Z-11d9f8d9原1/25.822100秒保留，零native编译/产品运行。

| run | 原退出码 / 秒 | 样本 | qlog字节 | EOF不完整末条 |
| --- | --- | --- | --- | --- |
| turing-20261113T051025Z-5907dd5c | 0 / 180.620308 | 720 ok | 326,868,694 | 6/8 |
| turing-20261113T052027Z-1edb3353 | 0 / 182.737763 | 720 ok | 333,501,091 | 6/8 |
| turing-20261113T052344Z-509f2cd6 | 0 / 184.584695 | 720 ok | 330,356,066 | 5/8 |

全部2160条原记录成功、三份完整192MiB文件。所有raw trace保留并绑定SHA/字节；部分连接退出时最后一条JSON-SEQ记录截断，明确保留原尾字节/hash，不能以此证明后续事件不存在。700MiB采集保护均未触发，原collector512MiB压缩/1GiB展开与2秒caller/240秒file/600秒owned门未放宽。

结论：三次未复现原超时，**对原故障未定因**，停止追加相同诊断。qlog有运行开销且是不同instrumented产物，三绿不能抵销原unlogged两红，不能认定修复或完整N3通过。接续按明确的新问题做静态定位或受控消融，不靠重复挑绿。

E/workload-qlog-diagnostic-summary-v1.json绑定三个专属proof；E/workload-qlog-diagnostic-progress-v1.json/results-v1.json的coordinator仅负责后两次，故其runtime_executions=2不是全批只有2。exec68965/PID40415已终态0，ledger无pending，root恢复唯一writer。

上游元数据于2026-11-13T05:25:07Z核：iroh1.4.1、noq/noq-proto1.4.0仍最新稳定；见E/upstream-latest-recheck-20261113T0526-v1.json。GitHub Noq latest UI旧tag不可代registry latest。
