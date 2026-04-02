# Claw-Code 核心运行时深度融合计划 (Runtime-Centric Evolution)

## 1. 核心目标
将 Claw-Code 从“带功能的 CLI 工具”重构为“以 Runtime 为核心的 Agent 操作系统”。通过解耦 UI (CLI) 与执行逻辑 (Runtime)，实现多渠道接入、细粒度权能控制和高可用模型调度。

## 2. 演进架构 (Proposed Architecture)
- **Core Runtime**: 负责状态管理 (Session)、权能校验 (Identity) 和工具执行 (Tools)。
- **Gateway Layer**: 处理所有模型请求，提供重试、Failover 和路由。
- **Channel Adapters**: 将不同来源（CLI, Slack, Web）的消息转化为统一的 `ChannelMessage`。
- **Identity Provider**: 维护多租户/多用户下的 API Key、WebAuthn 凭证及权能 (Capabilities)。

## 3. 实施路线图

### 第一阶段：权能融合 (Identity -> Runtime) [当前目标]
- [ ] 在 `runtime::Session` 中引入 `identity::CapabilityRegistry`。
- [ ] 重构 `runtime::permissions` 逻辑，将原有的 `PermissionMode` 映射为细粒度的 `Capability`。
- [ ] 在 `runtime::ToolExecutor` 中强制执行前置权能检查。

### 第二阶段：模型调度重构 (Gateway -> Runtime)
- [ ] 修改 `runtime::conversation`，将原本对 `api::ProviderClient` 的直接依赖改为对 `gateway::ReliableClient` 的依赖。
- [ ] 允许在 `.claw-code.json` 中配置网关路由规则。

### 第三阶段：IO 抽象化 (Channels -> Runtime)
- [ ] 定义 `runtime::io` Trait，解耦对标准输出的依赖。
- [ ] 实现 `ChannelBridge`，使 `runtime` 能通过 `ChannelManager` 接收和发送消息。
- [ ] 实现 CLI 适配器，将其作为 `channels` 的一个实例。

### 第四阶段：薄化 CLI 与 Server 启动
- [ ] 移除 `claw-cli` 中重复的逻辑，使其仅作为 `runtime` 的一个前端。
- [ ] 在 CLI 中增加 `serve` 子命令，一键启动 `server` 守护进程模式。

## 4. 详细执行方案 - 第一阶段：权能融合

### 4.1 权能映射表
| 原模式 (PermissionMode) | 对应 Capability 集合 |
| :--- | :--- |
| `Allow` | `[ReadFiles, WriteFiles, ExecuteCommands, WebFetch]` |
| `ReadOnly` | `[ReadFiles]` |
| `Restricted` | 根据配置动态 Grant |

### 4.2 改动路径
1. **`crates/runtime/src/lib.rs`**: 导出新的 Identity 关联类型。
2. **`crates/runtime/src/session.rs`**: 在 `Session` 结构体中持有 `CapabilityRegistry`。
3. **`crates/runtime/src/permissions.rs`**: 将 `verify_permission` 改为 `check_capability`。
