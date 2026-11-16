# 已确认选路及时传到应用状态

2026-11-16，候选，完整目标未接受。

原p7i在旧/新准确Release对照SNNSSN六场中1通过/5失败，旧S1/3、新N0/3，通常20.17–20.19秒超原20秒，唯旧一轮18.139秒；journal双方实际选择约18秒。所有原失败保留，不能归因新normal-gain唯一造成。源码另有明确额外链：iroh已确认Selected后，Wires每1秒采样，要求2次相同才更新Slot.word，IPC Hops/links读该词。这会再加1–2秒应用可见等待。

候选H9acc6097/tree06b0c2cd复用roads::follow已有PathEvent消费者，在started/Selected/Lagged的当前选择快照上通知Wires；按peer和稳定conn ID只更新仍存活的匹配槽，读取当前实际选择而非重放队列内旧remote。Opened不算选中，Unknown不造事实，同词不刷新年龄；旧conn/闭conn不得覆盖新conn或其他协议。journal关闭仍传播状态，Weak<Wires>不保持Node/缓存存活，无新增timer/probe/定期任务。原轮询与两样本fallback保留，用于未取得权威事件的读数。

新增两真实QUIC合同，不依赖Wires.tick或journal，验证消费者接线、弱所有权、相同词年龄、旧/闭ID和peer/协议边界。原Wires16合同（包括原两样本/Unknown/年龄）源码不改。044243Z-c6fc1ac1原0/116.250秒，Wires18/18、roads3/3，fmt/源/产物/原生进程组独立核验；E/run/selection-view-native-verification-v1.json。

该21项是组件接受，尚未证明原p7i物理20秒已绿。当前正准备准确H普通Release，原S/N失败、原数字门和未运行的N3阶段保持。原screen后5校准/3high未执行。

E=`/Volumes/disk0/Khor/evidence/net`。方案selection-view-events-plan-v1.json，源selection-view-events-source-v1.json，固定六场汇总normal-gain-idle-pair-verification-v1.json。未合main、替换现装App或发行。
