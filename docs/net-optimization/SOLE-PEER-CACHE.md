# 唯一peer relay的缓存物理合同

预先协议E/sole-peer-cache-physical-plan-v1.json，固定四case各3fresh、替代0，Debug只判行为。仅测试overlay；产品源117c39未改，tree14e9f060422b3cfbd28976364d781e80daedeef7，snapshot05325c56df1ab25d53878bcc16537187a9abff7a。

远端格式准备053241Z-3a264e87已0/51.258588秒，12原files及source/hash/identity/group空核，准确fmt patch应用本地；native编译053516Z-5aa15758已0/81.366352秒，16files及原native verifier的20first-party unit/同producer CLI与test核。CLI SHA f129a57c仍与之前闭合唯一middle产物相同；旧ELF只读保全。

runtime053846Z-4b1bebf2，control09683361468cadb52033c1e83d1147c352a68703，原1/365.502303秒；36files/source/hash/identity、原有owner核干净，无强制/遗留。全矩阵原红保持，不能写成12/12。

| 场景 | 实际通过 | 说明 |
| --- | --- | --- |
| sp2 | 3/3 | 不可用的第二腿提示（原JSON null，非删map字段） |
| sp3 | 3/3 | 过时Direct/1ms第二腿提示，实际第二腿Relay |
| sp4 | 3/3 | X对目标的历史Direct提示，初始无活XY |
| sp5 | 0/3 | 暂停/恢复前置见证不满足，未证明产品负向缓存根因 |

前三类均hint前后保持、初态cold/no_relay=false、目标独有marker在原5秒返回，并见实际唯一M的Direct/Relay两腿及XRelay/YUDP真实drop counter。用现有公开DeviceDoc/load_pile/set_road/flush写隔离roster，无生产test hook或适配transport。

原首业务微秒（按固定执行次序）：
- sp2: 520744, 519725, 528708
- sp3: 528011, 547317, 524143
- sp4: 513165, 509044, 509370

sp5所有三次middle_down=false，前两次first_leg_up=false，末次middle_resumed=false。原读失败、恢复超时及全部记录都保留；既不能替换分母，也不能据此将根因归给30秒MISS缓存。下一版前置须用实际M→Y暂停拒绝、X→M真实只读和恢复真实目标内容见证，不能只依赖UI Link.down_ms。set_machine_paused接受设备名；已核lib.rs定义，不能误称名字参数无效。

准确专属proof E/turing-20261113T053846Z-4b1bebf2/sole-cache-physical-verification-v1.json；ledger已闭。不重开前三类已闭固定批次；未来修正前置需新协议、新source、独立原始分母。整个NET-16的其他策略、缺字段、多middle、移动/失败与成本仍待；N0–N5保持。
