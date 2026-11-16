# feat/net 当前状态与接续入口

更新于2026-11-16 15:04 UTC。用户“做完吧”继续完整目标；最新询问进度不取消执行。[GOAL](GOAL.md)、[PLAN](PLAN.md) N0–N5尚未整体完成。root串行，不派子智能体。本机source-only，重型工作限Turing、macserver与授权khor-build。

## 当前唯一运行

- Turing当前唯一提交：登记/整Relay故障域V3，revision `registry_fault_domain_v3`，control `a1c59502fbaad7cfd0e55849eec347e2b718eeea`，`E/registry-fault-domain-native-plan-v3.json`。查`E/source-refs.json`原receipt再观察，不重复启动。准确源1dff35a6/tree455e42c4/snapshot668b4024，只改pkarr测试。新两场景各固定3轮＋原pkarr/Relay restart/token/jitter/stuck合同，预期7pass/原1ignored live test，首红停止，不补样本。
- 此源独立worktree `khor-net-background-jitter-v1` / `b04e3cb3d80b8e4628e9a35ebc0b7ed98025d871` / tree `08f8d3c9af754c010402a5fbf7734935c30c59bb` / snapshot `5c1fc9ed9ddca176987bc0b7c237d97f49ca0e4e`。在I8＋early fixture上，只增加每Wires的std RandomState与稳定有界正扰动：首次5秒、睡眠300秒、上限300秒不变，连续失败2起增加0–25%，无新依赖、任务或peer表；前台与联系/换网代次不变。
- canonical已以merge **eed1ab9af01d78627b3b73dfac7aab1aa60f8ec4**采用I8/DNS/cold-hint caller删除/early fixture/jitter。相对通过完整358的b04，所有非docs文件逐字一致；不是最终canonical整tree的Quality/Mac接受。新N4测试仍在独立worktree，未采用；main未合并、未发行。

## 最新已核证结果

E=`/Volumes/disk0/Khor/evidence/net`。已闭证项不因compact/session重跑；所有原红、未执行与原门继续保留。

- **后台错峰原生RED→GREEN**：`142005Z-4e15b0ef`原101/114.097秒，原18＋journal控制通过，新增spread测试真实失败；实现后`142930Z-4cf80a91`原0/102.204秒、20/20通过，同测试/断言保持。各run的`background-jitter-{red,green}-verification-v1.json`，源/控制/清理已核。不是完整网络或物理手机接受。
- **b04完整网络＋真实集成全GREEN已闭** `143457Z-68b8fbe7`原0/**956.140秒**，原25组 **358pass/0fail/原2ignored helper**，全部测试名/原门/格式/source/control/owner核验完成。`E/RUN/background-jitter-full-regression-verification-v1.json`；global separate与原显式negative override保持，包括最新early跨族握手门、全部联系代次、共享/extra取消、IPC跨进程与真实文件/同步。不是Release性能或手机接受。
- **N4两次原拒绝已保留并闭证** `145418Z-a06cf28f`原101/34.316秒为测试SocketAddr类型缺失，0native执行；`145735Z-c6ae0446`原101/40.170秒为新增两项首次id-only lookup未找到记录，原pkarr1过/2新增失败/1ignored，0固定fault轮完成、后4组未执行。还不能称产品根因；V3只补原计划20s内的独立HTTP signed-record发布前提，不给客户端resolver预热，不改原15s lookup/2s payload/30s全故障恢复和固定3轮。V3保留首次HTTP状态，等待实际证据；0生产变化。
- **I8完整Quality** GH43873334184，准确b899669a/tree68f191b3，首个实际run/attempt1原13/13成功，包括signed SSH151.86秒。`E/net-product-quality-v8-verification-v1.json`；CLI/Node各3563文件逐项核3077 checkout＋486同run frontend生成文件、ZIP SHA/CRC/实际fresh产物均核。第一次派发push未完成而422，0run；原拒绝保留。
- **I8完整Linux原网络回归** `131052Z-bd3bf61a`原0/503.944秒，12组323pass/0fail/原ignored1。`product-v8-separate-node-verification-v1.json`。共享八STOP与独立extra关闭/handler回收/cached STOP均通过；取消旧fixture竞态定因见[CANCEL-LAYOUT](CANCEL-LAYOUT.md)，0生产取消逻辑变更。
- **I8原生Mac Lab** GH43874780032首轮4/4job，104浏览器case/0错误、98tracked＋18generated来源、实际arm64 backend包均核；`E/public-native-lab-net-product-v8-v1/lab-acquisition-verification-v1.json`。本机0产品运行，无安装替换。不是手机或整N5接受。
- **真实Node集成** `140707Z-88cbb82c`原0/467.727秒，13组33pass/0fail/原ignored1。源d53ce645只修early UDP fixture，生产逐字同I8。`node-integration-contract-verification-v1.json`。旧同族代理实际完成握手已直接证明；新跨族代理保留实际握手未完成前提、原no-write/positive-write及1＋3秒原门，不据旧RED声称安全漏洞。
- **同early源Mac补验** GH43884943661原early1/1通过7.25秒，但原workflow在artifact finalize时ETIMEDOUT失败、0可取artifact；raw job log/source tree/control/run attempt已留。`E/early-barrier-isolation-mac-progress-v1.json`已标partial，不是整workflow绿。14:29 UTC原macserver SSH alias仍banner超时。最终候选Mac回归纳入early，不能只为上传重复此旧run。
- **完整同ELF共享/分连接对照** `system-dns-coexistence-verification-v1.json`：原SCCSSC六块、12case、8640交互、12完整文件、24post-health均成功，原相对/绝对p95与文件率门过。低profile目录p95共享273082μs/分连接84206μs；高profile终端p99共享639109μs/分连接916769μs，n360描述。**保留shared默认**，separate opt-in实验保留；收益尚未证明整体稳定性。旧cold TCP准入RED和旧业务RED不抹掉。不是同期main/SSH-hop-sync/物理手机。见[COEXISTENCE-DECISION](COEXISTENCE-DECISION.md)。
- **DNS/NAT64** 原IPv6-only＋SIIT＋系统AAAA独立oracle下冷配对10秒RED；只删固定公共DNS覆盖后3次fresh配对/6双向目录GREEN。Mac原319网络合同通过，普通Release来源闭证，见[DNS-NAT64](DNS-NAT64.md)。系统DNS同步初始化的Apple边界仍在，不声称上游已彻底修复；固定DNS64不是动态CLAT/手机。
- **custom审查** iroh1.4.1/Noq1.4.0已核。60项逐项审查完成，10项专属消融反例、3观测API与其他合同边界详[PATCH-REVIEW](PATCH-REVIEW.md)。cold-hint唯一生产caller删6行已进入I8并通过原全组；vendor API本身尚留，未扩张无理由重写。
- **移动/恢复与成本既有证据** 双端dual/v4换址252读、IPv6-only15case/630读、单middle真实两腿/Release ABBAAB、旧失败缓存3红→3绿、5f恢复/路径控制已闭，详原专项文档。1/3/5设备18场54 resident interval成本已量化；worker数量screen因CPU门未过明确拒绝，未采纳生产cap。硬件唤醒与手机能耗仍缺，见[COST](COST.md)。

## 下一步与未完成范围

1. 先闭当前N4 V3原run，保留所有结果与未执行组；完整358已闭，不重复派。mac旧early上传失败记录已闭partial，不为上传单独重跑。
2. canonical生产分量已集成且核对相对b04仅docs差异。N4测试通过后纳入，冻结最终准确源，做Quality和原生Mac验收。main基线冻结93f6cb23，不追其他GUI任务，不碰LIVE23961。
3. 按[REQUIREMENTS](REQUIREMENTS.md)逐项对账；执行归并见[CLOSEOUT](CLOSEOUT.md)。继续N1/N2的多middle、预算/取消/重放及N3的SSH-hop-sync共存、成本，N4独立故障域/冷peer寻址/发布与Relay容量；现有窄合同不冒充完整场景。N4独立测试worktree `khor-net-registry-fault-domain-v1` 从b04建立，仅改pkarr integration，`E/registry-fault-domain-plan-v1.json`固定两场景各3次；当前V3已远端格式/提交/派发，所有原计划与失败保持。
4. 物理iPhone localhost现装为**TestFlight**（用户本轮确认），旧0.2.0和先前就绪窗口不能验收新候选。候选发行必须按main ancestor rc tag与签名环境政策；不绕过流程。先把授权范围内候选做具体可评审，再请求所需main合并/rc/TestFlight授权，不能现在半途提审批。
5. 最终同源报告、设计、持续矩阵与手机换网/锁屏/能耗仍缺，goal保持active。无main合并、现装App替换、rc/发行/TestFlight上传或生产Relay采购部署授权。

## 环境与调度

Turing UID1110/GID1002，自有`/tmp/khor-netlab-feat-net-1110`，原owner/flock/build.lock；Rust1.95/jobs16/nice10，磁盘start50/stop40GiB。性能计时CPU idle≥.8/PSI≤5/无同UID Cargo，同host编译与计时串行；Mac不同host可并行。stage/launch各一次；先查receipt、execution、原owner，再collect＋identity＋实际合同核验后关闭ledger。不得覆盖原失败或替换样本。

MacLab与Turing同LAN，macserver另一网络。Mac重型工作唯一run-build槽；原SSH不通时使用已授权公开`colehank/khor-build`，不自行安装替换产品。其他代码治理环境与资源按中央STATE协调，不借用其GUI实施权限。

本页此前逐字状态保存在[HISTORY-20261116-through1421](HISTORY-20261116-through1421.md)，SHA256 `2b24e36e7093d0eae10b1c95a1fffec2e3ac71f2d6c2b9dc6c4bf5091b1856ff`。其中在途文字均为历史，以本页与实际receipt为准；全部旧红/未执行/来源继续保留。
