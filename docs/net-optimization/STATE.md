# feat/net 当前状态与恢复入口

更新时间：2026-11-09

## 当前状态

**规划已落盘；网络优化实现尚未开始。** 用户本轮授权创建 worktree、分支 `feat/net`，将讨论形成跨 compact/session 的持久计划。没有运行本轮 netlab、原生构建、线上探测或部署。

| 项目 | 值 |
| --- | --- |
| worktree | `/Volumes/disk0/Khor/worktrees/current/khor-net` |
| branch | `feat/net` |
| 创建基线 | `6485cb83d3b1f0bb93d4633825dc956623d78af9`（当前 main / rc.8 基线） |
| 权威计划 | [PLAN.md](PLAN.md) |
| 当前所有者/方式 | root，串行；无本任务子智能体 |
| 活动远端任务 | 无 |
| 证据目录 | `/Volumes/disk0/Khor/evidence/net/`（预留，暂无测试产物） |

规划提交的 SHA 以实际 `git log` 为准，避免把自引用 SHA 写进本文件。主仓既有 `lab-preview/` 及其他任务 worktree 不属于本任务。

## 必须保留的用户决定

- **速度/体验优先，不能直连硬优先。** 这已是产品方向，不需要恢复后再询问是否坚持直连优先。
- 目标是极致个人设备网体验；netlab 用于复现、比较和回归。
- 遵守 macserver + 私库 `colehank/khor-build` 的远端开发规范，本机 source-only。
- 当前先讨论和保存计划。先前“没让你直接改，先讨论清楚”没有被本轮创建 worktree 的指令扩展成产品实现授权。
- 中继数量/位置、双家、切换阈值、流量隔离方式和 Linux runner 仍是待测提议。

## 阶段进度

| 工作包 | 状态 | 证据/下一步 |
| --- | --- | --- |
| P0 计划与恢复入口 | 已完成文档准备 | PLAN、STATE、NET 索引及本地 AGENTS 入口；仅文档检查 |
| N0 基线与实验入口 | 未开始 | 核对测试覆盖、冻结指标和门槛、验证 Linux 执行器方案 |
| N1 速度优先选路 | 未开始 | 覆盖慢直连/快中继及防抖，然后修改策略 |
| N2 连接与恢复 | 未开始 | 候选竞速、旧地址、请求总预算、恢复事件与安全重试 |
| N3 流量调度与后台 | 未开始 | 混合负载、背压、公平性、耗电与恢复并发 |
| N4 中继与寻址 | 未开始 | 端到端测量、故障域、双家实验与部署设计 |
| N5 整体验收 | 未开始 | 补丁覆盖、持续运行、真机与最终源码验证 |

## 已有事实与缺口

- 静态审阅基于上述 baseline：`crates/net/src/carry.rs::choose_settling` 的直连硬优先已确认，原测试包含 90 ms 直连优先于 5 ms 中继。
- 连接复用、退避、SSH/设备转发、journal、netlab 和 vendor 补丁都已有基础，不从零重造。
- 历史文档中的实验数字不是本轮测试结果。当前线上可用性、性能基线和所有验收门槛尚未验证/冻结。
- 现有 `scripts/dev.py` 任务有 `fast/native/lab`；`lab` 指 Mac 原生 Lab，不是 netlab；不存在已确认可用的 `--task netlab`。
- 中央其他 GUI 工作仍独立进行。本分支不自动合入 GUI 集成树，也不继承其他任务的实现/发布授权。

## 下一次接续

1. 读取 STATE、PLAN 与最新用户指令；核对 `git status --short`、分支和 HEAD。
2. 若仍在讨论，完善待决方案并同步文档；若用户明确开工，从 N0 开始，例行步骤自主完成。
3. 启动任何重型检查前，重新确认 `khor-build` 当前 driver/任务、资源槽与源码快照。Linux netlab 入口需先验证，不在 Mac 上直接跑 namespace 测试。
4. 用户新决定、实际代码变更、运行 ID、验证版本和未完成项及时更新本文件。不要积累互相矛盾的“当前状态”；历史移到简短记录中。

外盘缺失时，在 `/Users/zgh/Desktop/working/khor` 用 `git show feat/net:docs/net-optimization/STATE.md` 和 `git show feat/net:docs/net-optimization/PLAN.md` 只读恢复已提交计划；不覆盖外盘原未提交内容。

## 本轮记录

- 2026-11-09：用户明确创建 `feat/net` 并保存计划。以 main `6485cb83` 创建独立 worktree，保存六阶段计划、体验指标、15 类场景、远端验证边界和恢复规则。
- 本轮仅验证文档差异、链接/路径与 Git 状态；未启动网络实现、构建、测试、购买、部署或发布。
