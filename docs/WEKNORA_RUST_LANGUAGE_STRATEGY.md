# WeKnora-Rust 语言选型与分阶段重写策略

本文档记录对当前 WeKnora 代码结构的查验结果，以及在重写为 WeKnora-Rust 时，各部分应优先采用的语言和迁移顺序。

> 结论：最高效的方案不是把所有代码强行改成一种语言，而是采用 **Rust 核心服务 + Python 文档/AI 生态适配 + TypeScript 前端 + SQL 数据层** 的组合。Rust 负责长期运行、并发、网络、检索、任务调度和安全边界；Python 保留在 OCR、版面解析、多模态和模型生态最快的位置；TypeScript/Vue 继续负责 Web UI；SQL 继续承载数据结构和数据库原生优化。

## 当前代码构成

从仓库文件类型和目录结构看，当前应用大致分为：

| 模块 | 当前主要位置 | 当前语言/技术 | 迁移建议 |
| --- | --- | --- | --- |
| 服务端 API / 路由 / 业务服务 | `cmd/server/`, `internal/` | Go | Rust |
| RAG 检索、chunk、向量库适配 | `internal/application/`, `internal/infrastructure/`, `internal/models/` | Go | Rust |
| Agent / MCP / 工具编排 | `internal/agent/`, `internal/mcp/`, `mcp-server/` | Go / TypeScript | Rust 核心 + TS/Node 兼容适配 |
| 文档解析 / OCR / 多模态解析 | `docreader/` | Python | Python 保留，逐步封装为 Rust 调用的独立服务 |
| Web UI | `frontend/` | Vue / TypeScript | TypeScript/Vue 保留 |
| 桌面端 | `cmd/desktop/` | Go / Wails | 后续评估 Tauri(Rust) |
| CLI | `cli/` | Go | Rust，单二进制发布 |
| 数据库迁移 / schema | `migrations/` | SQL | SQL 保留 |
| 微信小程序 | `miniprogram/` | WXML / WXSS / JS | 保留小程序技术栈 |
| 部署 | `docker/`, `helm/`, compose files | Docker / YAML | 保留，新增 Rust 镜像 |

## 推荐语言分工

### 1. Rust：核心服务与高并发路径

适合重写为 Rust 的部分：

- HTTP API server、middleware、auth、tenant/session 边界。
- RAG 检索链路：chunk 查询、rerank 前后处理、检索融合、上下文组装。
- 向量库、对象存储、数据库 repository trait 和实现。
- 任务队列 worker、wiki ingest 调度、失败重试、死信队列处理。
- Agent runtime、MCP 审批边界、工具调用安全沙箱边界。
- CLI 和部署探针。

原因：Rust 在 CPU 密集、并发 I/O、长期运行服务、内存安全和单二进制部署上更适合作为核心后端语言。

### 2. Python：文档解析、OCR、多模态和模型生态

建议保留或独立服务化的部分：

- PDF/Office/图片解析。
- OCR、版面识别、表格抽取。
- 依赖 Python 生态更成熟的模型推理 glue code。
- 快速试验的新解析器和模型适配器。

原因：Python 在 AI/文档处理生态中拥有最高开发效率。重写为 Rust 只有在某个解析器成为明确性能瓶颈后才值得进行局部替换。

### 3. TypeScript/Vue：前端交互

建议保留：

- Web 管理台、聊天 UI、Wiki 图谱浏览、配置页面。
- 类型化 API client 和前端状态管理。

原因：Web UI 的最高效率仍然是 TypeScript/Vue 生态；Rust 不适合作为主要前端应用语言。

### 4. SQL：数据库结构与查询优化

建议保留：

- schema migration。
- 数据库索引、约束、视图和数据库原生查询优化。

原因：数据库密集逻辑应靠 SQL 和数据库执行计划优化，而不是搬到应用层。

## 分阶段迁移顺序

1. **Foundation**：Rust 服务骨架、健康检查、版本接口、capabilities、Docker Compose 部署。
2. **API Contract**：对齐 `/api/v1` 响应 envelope、错误码、分页、SSE/streaming 协议。
3. **Repository Traits**：为 tenant、knowledge base、document、chunk、session、agent 建立 Rust trait。
4. **Read Path First**：优先迁移只读路径：KB list/view、doc list/view、chunk search。
5. **RAG Runtime**：迁移检索融合、上下文组装、chat streaming。
6. **Workers**：迁移任务队列、索引构建、wiki ingest 调度。
7. **Write Path**：迁移创建/更新/删除、权限、审计日志。
8. **CLI/Desktop**：迁移 CLI 到 Rust；桌面端再评估 Tauri。
9. **Python Service Boundary**：保留 docreader 为 Python 服务，通过 gRPC/HTTP 与 Rust 核心交互。

## 当前落地

当前 Rust 服务已经把这份策略暴露为运行时接口：

```bash
curl http://127.0.0.1:8080/api/v1/rewrite-plan
```

该接口用于在部署验证、CI 冒烟测试或后续迁移面板中读取当前规划。
