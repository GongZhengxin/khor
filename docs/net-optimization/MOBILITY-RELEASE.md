# 当前集成版的移动地址回归

此页记录有限 Linux netlab 结果，不代表完整 N4、物理 iPhone 或能耗验收。

当前产品 runtime 逐字 I2 `3093f866`（R `9081b94d` 加验证工具修正），iroh1.4.1/Noq1.4.0；测试树 `a5de5685` / tree `90a1d7813377d611ea93b3276a36d3876e28e09c` 只增加两个 Release bridge，共35行。原始场景和 helper 不改。

固定三场 `163223Z-b6d7f5c7` 已闭，0/108.146424秒，15files/identity/native来源/CPU/owned清理均核：

| 原始场景 | 已核结果 |
| --- | --- |
| 地址注入校准 | 7次正向包检查＋1次旧地址负向检查；实际同前缀IPv6替换，IPv4不变 |
| IPv6-only，双向发起 | 124次真实目录读全成功，含2次setup；换址后的首次a→b/b→a为258230/1217614μs |
| IPv6-only，远端先发起 | 124次真实目录读全成功；首次b→a为1732189μs，完成后a→b为13665μs；原事件顺序已核 |

Node场景保留IPv4内核地址，但在两侧真实namespace中阻断eth0 IPv4输入/输出；不是双栈fallback测试。核验了旧IPv6消失、新IPv6存在、IPv4快照不变、前后nft隔离规则与实际包检查。原read30秒/setup120秒/scenario240秒/owned300秒保持；以上首读只有各1次，不作分位数或UX达标声明。

证据：E/turing-20261113T163223Z-b6d7f5c7/current-ipv6-mobility-block-verification-v1.json，E=`/Volumes/disk0/Khor/evidence/net`。CLI普通Release SHA `7e8c52bc4d21a3471e3c0592ad1a874147a7d024c067bda23a7f4d08213b34df`；新原生准入与两次编译前控制器错误见STATE，失败原件保留。

剩余包括整网IPv4/NAT、双栈前缀、地址族转换、双端同时变化与peer relay移动；发布/登记收敛、新连接地址hint和旧epoch不能只凭这些读成功认定完成。真实Wi-Fi↔蜂窝、锁屏/休眠及能耗仍待候选真机验收。当前执行见STATE。
