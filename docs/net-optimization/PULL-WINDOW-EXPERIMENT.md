# 文件预取窗口隔离对照（尚未运行计时）

前置新N3对照8640/8640功能过，但high bulk目录520→963ms、PTY338→695ms，吞吐18.657→24.413Mbit/s不抵销交互回退。假设应用待传窗口过大有待实验，尚未归因BBR/任一vendor补丁。

先冻结E/pull-window4-design-v1.json，在独立khor-net-pull-window4-v1 / experiment/net-pull-window4-v1，从DB7bac仅改PULL_AHEAD8→4一行；当前8片2MiB，4片1MiB，单片仍256KiB。准确dca27a72edd6d91d710283b24631d3b77294e7f2/tree91b8577e30bddac900f0a8ab4c8d42ba1a672612/snapshot211c9058d966c52ee9804f744522844c83373eca；无CC/队列/fixture/features变更。该实验基于DB，保留I4新浏览器锁的正式候选不得由它回退，最终只可能cherry-pick这一常量改动。暂不采用窗口4。

普通Release producer233938Z-8f18bccd已0/550.476037秒闭证，22files/identity/strict native完整，实际Rust8m19、CLI fresh=false。CLI0b0968df21921930f338df819715a91bbafe4e51eb9f74ee71a78ea138df19c1、fulltest5aa21a498f587a8339683f3d777ccf548dc6b838e9dd51f5b2bcd78c875f123e；相对DB源frame仅link.rs，Git反向恢复同DB tree，实际CLI字节改变。E/run/pull-window4-native-compile-verification-v1.json；control5494ea7db988366bcdf037b182b18a4613eeffa3。只有编译，业务计时0；原selected5存档/来源工具/46旧validator准入复用规则保持，不拷Cargo fingerprint。

计划两臂新校准后固定8/4/4/8/8/4六个high场景，4320请求、6完整192MiB文件；原2秒caller/240秒文件/400ms采样与owned600秒不改。8臂复用DB214246Z-2606e4e9 closed native，4臂复用上producer，统一strict source/selected5/CPU/disk/清理门，无运行编译或替代。需先冻结实际runtime控制和source bindings再调度，当前尚未创建runtime控制或发起校准。

结果须同时报告窗口8对4的因果性能/吞吐取舍，以及原main工程界限：high bulk目录p95<=592518.1μs、PTY<=391538.2μs、完整文件median>=16.791371Mbit/s（原main18.657079×90%）。不以传完或吞吐绿代交互接受，不改原门或删坏样本。共享/分连接、其他真实负载、资源/手机成本、最终同源验收仍属于完整目标。
