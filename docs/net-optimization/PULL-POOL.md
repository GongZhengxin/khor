# 活动文件连接与前进额度

当前候选2e42b346/tree596ebda4，experiment/net-pull-pool-contracts-v1。默认shared，候选内部separate模式才启用；没有安装/发行。设计原文见PULL-POOL-DESIGN，后续运行前修订与准确proof见STATE和E/pull-pool-*-v1.json。

Node的canonical Arc<Endpoint>作为连接域：地址变动不创建新域，同key的新endpoint不能共享旧extra。Wires持有协调器，域只弱持有endpoint，peer只弱持有连接；活动文件及实际请求持有Lease。每peer一个准备锁、最多一个extra；node最多4个extra，满额继续cached。没有空闲连接保活timer。最后owner释放才关闭extra，主动退役则立即关闭/归还permit，即使旧Arc还活着；旧Drop不会清除新连接。

客户端bulk额度node32/peer8，foreground额度node8。额度随pending请求及已解码但尚未顺序提交的ready响应保留，提交/丢弃才归还；独立于服务端encoded-response额度，避免双向传文件互锁。正常每slice最多SLICE的payload，合计40slice是此额度模型的payload界，不包含QUIC/编码/请求对象和其他进程内存；实际资源成本还要测。

只用bulk会出现真实饥饿：A的head慢且7块ready占着8bulk，B首块虽已收到，却无法发后续任何offset。准确RED清理后失败；同一测试/控制/观察界下，修复后B完整13块完成且A始终未释放head。修复让当前提交cursor优先取可立即使用的bulk；若会被bulk挡住，则用foreground。排队的prefetch只有成为cursor时才提升，取消sem等待而不取消/重发RPC。cursor在提交或版本reset后更新，所有额度等待沿用入队前的原caller截止时间。没有新增后台轮询或网络探测。

27项实际QUIC合同已闭：原20隔离/准备、4共享lease/Endpoint/退役/全局extra合同，以及两个真实并发完整文件和上述晚到文件合同。完整并发文件各13唯一offset/精确bytes和完成后的part/note清理；取消一个时真实STOP且另一个仍完成；最后关闭extra、全部额度归还、cached Main仍以原stable-id响应。源/owner/原fmt已核。

这些证据尚不覆盖node32真实饱和、foreground8饱和后的新caller、准备owner取消/到期与等待者独立性、所有迟到Endpoint/policy变化、实际1/3/5设备成本、当前源标准/Mac、Release共存或手机。额外连接远端关闭且旧Arc保留的全局容量回收也需实测。参数4/32/8是待比较的候选，整体N3保持未完成。
