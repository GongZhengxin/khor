# 计划完成审计

2026-11-09。逐项对齐 PLAN；“已有代码”不等于已验证，“部分”不能支撑整阶段完成。最新运行状态见 STATE，源码/运行收据见证据目录。最终收口时逐行复核，不删除尚未完成的要求。

| ID | 要求 | 当前证据与缺口 | 状态 |
| --- | --- | --- | --- |
| N0.1 | 当前源码、调用链、补丁与覆盖核对 | 选路/注册/确认/看守已跟踪；完整协议与补丁覆盖还未核完 | 部分 |
| N0.2 | 同条件 baseline/candidate/固定好路径对照 | 历史A/B/负向及失败按原版本保留。最新公开main c3b098对产品集成117c39的同夹具Release唯一middle ABBAAB已闭：基线三次5秒超时、候选345236/366587/366320μs成功；详SOLE-PEER-RELEASE。此前长稳A/B各5case及idle成本差异见STATE。最新共同Release双profile ABBAAB已闭，8640记录/12file全留；candidate high只有1/3，原3timeout/232blocked不豁免，见WORKLOAD-RELEASE。 | 部分 |
| N0.3 | 可追溯 Linux 执行器、私库调度与回执 | namespace/netem/nft、CLI、源码 tree/控制 commit 与原始 artifact 已实际验证 | 已有实证 |
| N0.4 | 场景分类与指标/门槛冻结 | p6/p7 和原 p2/p3 有协议；完整矩阵 baseline 42507224815 已分类（含配对busy与观察red），其余工作负载目标待补 | 部分 |
| N1.1 | 类型、端到端质量、停顿、样本时间/可信度 | 共享quality/age/count与ACK校正有证据；延迟读/旧包/初始估计三项红→真实时间窗434Noq/26iroh绿；path事件及新源实际行为仍待验证，置信/波动未齐 | 部分 |
| N1.2 | 去掉直连硬优先，分开可用性与排序 | 单元红→绿；同v2 fixture三轮A/B慢Direct/快Relay与反向及候选固定好路参照均通过，保留全部基线红 | 已有重复实证 |
| N1.3 | 有意义收益确认、防抖、故障快速通道 | 36普通stalls为2/2、p1s0/0移动；37候选Mac42555870179实际36路径/438Noq/9home绿。旧Linux dynamic25/27及hop8/9红保留；Turing候选recovery6/6，但corrected hop出现f4i回退红，基线/CPU/最终矩阵未收口。started计量reducer实际2红→4绿只证明fixture，不替代产品 | 部分 |
| N1.4 | 防止稀疏、积压、换网旧样本与 CPU 污染 | p7七相位×三轮最新21项全绿，idle至19.61秒、裕量小；真实时刻/过期质量已红→绿；CPU防抖/置信/探测成本未齐，可选日志只作定因 | 部分 |
| N1.5 | 多连接/协议观测共享与生命周期一致 | 178真实双连接/真实echo与两清理控制先过后，shared carrier释放判断真红；已改为核Main/Tunnel/served全部live消费者，179 Linux/180同源Mac原整fixture红转绿（各wires11＋24/20合同），181CPU/182hop证据保留；189旧owner实际红→193 Linux/194 Mac原整fixture绿，195严格hold恢复3/3、201参照9/9但automatic仍有慢轮；peer共享selected tuple与per-conn open paths边界明确，完整hop/多协议仍待 | 部分 |
| N1.6 | Peer relay纳入最优连接：实测完整两腿、双向、多middle与移动/失效 | 原单/双middle prepared合同和失败完整保留。最新真实唯一Direct首腿/Relay第二腿首业务：Debug固定3、Release候选固定3均实选FWD via M且marker返回，原no_relay=false/Idle，不需重开App；公开main同Release三次5秒未返回。缺失/过时hint、旧Direct/失败、全部策略负向、多middle/移动范围仍未齐。 | 部分 |
| N2.1 | 近期成功缓存与失效、坏地址不能耗尽期限 | 现有双尝试基础，尚未完成审计与对照 | 待做 |
| N2.2 | 有界竞速、输掉的尝试/取消释放 | pair/doctor HTTP探针外层取消、配对取消、按完成取首成功已有三红→42571393016绿；拨号输者及延迟候选资源边界仍缺 | 部分 |
| N2.3 | 可用连接后持续优化、有限备用与探测预算 | p7 覆盖恢复目标，具体机制待证据收敛 | 待做 |
| N2.4 | 总期限传递，区分路径/连接/远端/本机慢 | 原Op5/30/300/3600秒减.5秒by已贯通入口/锁/hop/普通exchange/Accepted/一次读重试；42571393016预算/流/探针14绿。Mac138与Turing141长ACK阳性绿、Turing141真实namespace/nft长黑洞F7绿（16合同）。grace之前取消143rd F8实际红→150th同函数Linux21绿、151st同源Mac19便携绿；F7独立确认/外部thread取消及replacement/Weak owner生命周期通过。F9 tick与F10长Accepted取消158实际红→160 Linux24/161同源Mac20绿，16秒健康Accepted控制保持；无pending proof的tick/served独立close与多quiet epoch仍缺；更新10秒/200ms by运行绿，IPC剩余预算/取消未跨进程与多阶段覆盖仍缺 | 部分 |
| N2.5 | 随机退避、并发预算、恢复事件合并 | 211/212已修稳定接口集合被无关更新反复推迟的通知timer，原1秒及30项journal合同绿；确定性指数退避/jitter与其他事件合并仍缺 | 部分 |
| N2.6 | 读写重试与 0-RTT 边界 | Accepted与普通wire/真实读重试同by、Sessions严格2次/Mkdir严格1次有Mac实证；全部写动作与包级0-RTT重放专项、最终准确源验证仍缺 | 部分 |
| N3.1 | 实测队列/并发/缓冲/外层流阻塞 | 原Noq独立方向FIFO及真实Node分层校准、低/高BDP政策数据见MODEL-DECISION/IROH-WORKLOAD-RESULTS/历史STATE；不是只有Noq模型。最新公开main对集成117c39的同Release双模型ABBAAB已闭：low过、high交互2/3失败，8640完整记录与12file，吞吐收益不抵销阻塞；独立上游qlog诊断编译中。SSH/hop/同步共存及完整资源成本仍缺。 | 部分 |
| N3.2 | 交互/前台/批量的优先级、背压、公平 | 已有真实Node低/高profile优先级/窗口/取消工作和原始失败；最终产品集成的Release固定双模型对比已闭：high1/3通过、3timeout/232blocked；原失败使当前候选被拒绝，正在定位。原caller/bytes/样本/真实重叠门保持；同步/预览/全负载公平与实际资源取消仍待最终对齐。 | 部分 |
| N3.3 | 比较共享与分连接，按数据选设计 | 尚无本轮比较，不能预先认定增加连接有效 | 待做 |
| N3.4 | 保留个人网拓扑，改进闲时/事件/恢复开销 | 174/176真实resident观测到各274–280存活thread，尚无闲时/能耗结论；需1/3/5设备数、空闲字节、CPU/唤醒与手机能耗记录 | 待做 |
| N4.1 | 两端完整路径及不对称可达性 | 实验室 p6 有完整测量；用户已澄清MacLab/Turing同LAN，构建macserver另机且A/B依用户异网报告；A/B公共Relay目录/正文120/120通过，Direct/NAT/v6穿透未验；localhost iPhone前台换网正在采样 | 部分 |
| N4.2 | 中继整机/出口/登记故障域验证 | 完整baseline曾取证；本轮静态确认北京双URL同机、默认自家登记集中北京，实际双端故障/冷启动仍缺 | 待做 |
| N4.3 | 单家/双家必要性与成本实证 | 尚未定案 | 待做 |
| N4.4 | 移动IPv4/NAT映射与IPv6地址/前缀独立或同时变化；Wi-Fi↔移动网、地址族切换、双方与peer-relay移动；缓存/发布/发现收敛 | U12明确；210生产通知真实红→211 Linux/212 Mac原整fixture绿，原1秒不变；215又完成同prefix IPv6实替换/IPv4保持的物理注入校准；Node业务、前缀/IPv4/NAT与真机矩阵未执行 | 部分 |
| N4.5 | 有测量支持的中继布局、容量与运维设计 | 部署/采购单列，当前未执行 | 待做 |
| N5.1 | 每项 vendor 补丁的原因/覆盖/上游/退出条件 | 已升独立产品候选至iroh1.4.1/Noq1.4.0，canonical尚未合。E/iroh-upgrade-patch-ledger-v4.json的60历史项为1上游替代已验、2移除红而留、57待退出核查；不得说57均必要。部分vendor及应用/物理恢复专属证据见STATE，逐项退出仍未齐。 | 部分 |
| N5.2 | 稳定短回归、完整矩阵、长时间与重复 | 稳定性前后各5case、600秒与32真实目录请求、原取消及成本回退已闭；最新产品Quality13/13首次通过、唯一middle Debug3/ReleaseABBAAB闭证。原完整NET-16/移动/成本/最终同源矩阵未全齐，不以短绿代全绿。 | 部分 |
| N5.3 | Mac 原生、真实手机与多机验收 | 集成117c39完整原生Quality13/13首次通过；最新Mac Lab43355810513原native成功、checks原WebKit失败留证/唯一同源重跑77浏览器case过，inner native ZIP/probe/source/contract已核、无本地产品执行/安装替换。物理iPhone仍旧安装版，新候选跨网/换网/能耗未接受。 | 部分 |
| N5.4 | 最终准确源码质量检查、集成复核和限制说明 | 产品集成117c39保留当前公开main apps/与.github/，完整Quality43348869616首次13/13及Mac Lab同源通过（原WebKit失败仍保留）。最终NET-16/N3/N4/custom收敛后的准确源复核、canonical集成与报告尚未齐；无main合并/发布。 | 部分 |
| D1 | 最终设计与决策说明 | DESIGN 是进行中记录，未完成的决策必须继续收敛 | 部分 |
| D2 | 状态/职责边界 | DESIGN 有职责表，还须与最终代码一致 | 部分 |
| D3 | 持续运行的验收矩阵 | 入口已运行；稳定门禁与失败分类尚未完成 | 部分 |
| D4 | 相同条件的前后对照报告 | 历史逐问题A/B原件和失败保持；SOLE-PEER-RELEASE完成实际公开产品对候选同发行配置的唯一middle首业务ABBAAB，基线5秒右删失，不计算有限倍数加速。Release目录/PTY/完整文件双模型固定ABBAAB正执行；完整对照报告未齐。 | 部分 |

完成审计还必须核对个人可信设备范围、原有信任与取消/写入语义、只用自家中继/暂停约束，以及源码、文档与运维方案的一致性。活动目标保持“完成计划”，不能缩成首批选路通过。


## U17／NET-16：单个存活邻居的peer relay救通

用户IMG_5608现场：5GA，hinton UI直连29ms、其他若干目标不通。只证明手机到hinton首段，hinton到每目标的第二段与当时策略尚未实测，截图不能证明某个源码分支。证据在外部phone-partial-reachability-20261110-IMG_5608，原图SHA与可见字段保留；未启动phone sampler。

验收须含：第一段Direct/Relay且唯一可达middle、第二段Direct/Relay/SSH；第二段记录缺失/过时、目标历史Direct/旧失败/本地或对端换址；正负控制middle能达目标、middle也不能达、暂停/未信任/递归hop均不得越权。真实两腿/端到端业务和原总期限/取消/无隐式写重放/安静能耗共同验证。候选准入不能把“声明配置了Relay”当作它当前可用，已有好路的性能比较与无路时的有限兜底应分别建模。保留原两middle和总3秒/更早caller、无全网广播和无界探测。源码已观察Legs::Dial排除Direct→Relay及要求已有两腿记录、wants_hop看历史Direct，具体改法仍需实际红与设计收敛，未记通过。


2026-11-13补充：独立当前产品集成候选的真实唯一middle首业务固定3次已过，三次初态Idle、no_relay=false，实证Direct第一腿/Relay第二腿及端到端目录，详见[SOLE-PEER-PHYSICAL](SOLE-PEER-PHYSICAL.md)。上方“源码已观察排除”是最初审计记录，不能覆盖后续候选事实；该窄物理合同通过不表示NET-16全部范围或canonical feat/net最终集成完成。缺失/过时hint、旧Direct/失败、负向策略、移动及成本继续保留为必要验收。

2026-11-13 2026-11-13T04:12:21.135145+00:00对账：以上只更新已有准确证据，其他历史缺口未据摘要推断关闭；最新在途以STATE及实际receipt为准。完整N0–N5范围保持。
