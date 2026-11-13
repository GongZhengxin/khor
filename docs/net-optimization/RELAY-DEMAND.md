# Relay静默故障中的真实前台请求

原idle c观察到fresh TCP20.0秒、再次heard22.9秒，但其原30秒合同不包含前台业务。独立测试树62b28e084a8cf72593a9ed1ba270327d281f9d49只新增2测试文件162行，产品逐字I2/R，原idle测试保持。

冻结协议E/relay-silent-demand-design-v1.json：原双strict IPv4 NAT与单Relay几何，真实旧alpha TCP tuple双向drop、新tuple可通；每场2个setup读，再40次新marker目录读取，caller2秒、250ms计划间隔、全部样本与串行积压保留，首个有效响应≤2秒。3个固定独立重复，无替代。

普通Release producer184400Z-8e2c478a 0/234.817081秒，Rust3分03秒，22files与严格来源已闭。测试5f607b385e9e6dfd96af183bd33adb265247dca699b6f7b8ba7ea3b9f6c80634；实际CLI重新编译后逐字同7e8c52bc，产品源未改。

runtime184914Z-5cfe53b4 1/198.559489秒，15files/identity/native/CPU/owned清理闭，**三场全红**。每场setup2/2过，故障后40读10次2秒超时、30次成功；共126实际调用96ok/30timeout。首个有效响应分别20042371、20040904、20049739μs。全部nft规则、原TCP tuple、42起止记录/样本关系已独立核验，见run/relay-silent-demand-runtime-verification-v1.json；失败不删除。

该几何复现了真实前台等待，不等同手机现场、全部故障域或所有策略。静态常规15秒Relay ping加最长5秒pong与观测相符；需候选对照验证因果。仅有空闲30秒内恢复不能作为最佳体验接受。

下一候选设计：实际请求已发且尚无ACK时，在按RTT缩放的短等待后请求现有Relay健康检查。由iroh继续判断pong/重建TCP；不因请求慢直接判死QUIC，不重放写，不把轻量健康检查升级成全endpoint重新绑定。检查仅属于有界请求，调用方取消后不保留新watcher；iroh现有actor合并并发提示，后台常规15秒节奏保持。具体候选与重复协议在E提前记录。原慢健康Relay、长Accepted、取消及后台成本必须回归，绿后才讨论晋升。
