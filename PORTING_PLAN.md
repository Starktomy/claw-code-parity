# ZeroClaw 平台能力全量移植计划 (Claw-Code 进化方案)

## 1. 核心愿景
将 `claw-code` 从单一的 CLI 助手升级为**企业级 Agent 操作系统**。通过直接移植 ZeroClaw 的成熟基础设施，使其具备多模型调度、多平台渠道接入、企业级安全认证以及可视化的管理能力。

## 2. 目标架构 (Target Architecture)

### A. 推理心脏：Gateway & Providers (移植自 `zeroclaw/src/gateway` 和 `zeroclaw/src/providers`)
- **模型路由**: 支持 Failover 和自动重试。
- **成本监控**: 实时计算每个 Request/Session 的 Token 消耗 (参考 `zeroclaw/src/cost`)。
- **安全拦截**: PII 敏感信息脱敏及 Prompt 注入防护 (参考 `zeroclaw/src/security` 和 `zeroclaw/src/trust`)。

### B. 通讯触达：Channels (移植自 `zeroclaw/src/channels`)
- **全平台支持**: Slack, Discord, Telegram, WeCom 等。
- **统一会话层**: 确保 CLI、Slack 和 Web 端共享同一个对话历史。
- **异步处理**: 支持高并发的消息推送与监听。

### C. 运行观测：Observability (移植自 `zeroclaw/src/observability`)
- **度量指标**: 基于 Prometheus 的请求耗时、成功率统计。
- **全链路追踪**: 使用 Tracing 进行分布式链路追踪。

### D. 扩展生态：Skills & RAG (移植自 `zeroclaw/src/skills`, `zeroclaw/src/skillforge`, `zeroclaw/src/rag`)
- **技能市场**: 支持动态加载和分发 Skills (参考 `zeroclaw/marketplace`)。
- **知识库增强**: 原生接入 RAG 能力，提供上下文记忆 (参考 `zeroclaw/src/memory`)。

## 3. 实施路线图 (Phased Roadmap)

### 第一阶段：基础设施搬迁 (Foundation) - [进行中]
- [x] 创建 `crates/observability` 并实现指标收集 (对应 `zeroclaw/src/observability`)。
- [x] 创建 `crates/gateway` 骨架并接入模型调度逻辑 (对应 `zeroclaw/src/gateway` 和 `zeroclaw/src/providers`)。
- [x] 配置 `crates/server` 整合新模块，提供基础 HTTP API。

### 第二阶段：多租户与安全 (Identity & Security)
- [x] 创建 `crates/identity` (整合自 `zeroclaw/src/identity.rs`、`zeroclaw/src/auth` 和 `zeroclaw/src/security`)。
- [x] 移植 API Key 管理与 WebAuthn 二步验证支持。
- [x] 实现基于权能 (Capability) 的工具调用控制。

### 第三阶段：消息生态接入 (Channels)
- [ ] 创建 `crates/channels` (对应 `zeroclaw/src/channels`)。
- [ ] 逐步搬迁 Slack, Discord 等核心 Adapter。
- [ ] 实现 `ChannelManager` 统一管理所有监听器。

### 第四阶段：高级 Agent 能力 (Skills, RAG & Memory)
- [ ] 移植 `zeroclaw/src/rag` 和 `zeroclaw/src/memory` 到 `crates/runtime` 或独立的 `crates/rag`。
- [ ] 引入 `SkillForge` 体系 (对应 `zeroclaw/src/skillforge`)。
- [ ] 整合外设和硬件能力抽象 (对应 `zeroclaw/src/peripherals` 和 `zeroclaw/src/hardware`)。

### 第五阶段：Web Dashboard & 自动化
- [ ] 移植 ZeroClaw 前端项目 (`zeroclaw/web` 或 `zeroclaw/apps`) 并挂载到 Server 的静态路由。
- [ ] 实现会话录制与重放 (Session Recording & Replay)。

## 4. 移植原则
1. **模块化重构**: 鉴于 ZeroClaw 的核心功能多在 `src/` 下，移植时需将其拆分为独立 Crates (如 `crates/gateway`, `crates/channels`)，以保持 Claw-Code 优秀的 Workspace 多 Crate 隔离架构。
2. **测试先行**: 每个模块移植后必须通过集成测试。
3. **零侵入同步**: 使用 Feature Flag 控制新旧引擎，确保不影响原有稳定功能。
