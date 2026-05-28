# WeKnora-Rust

> Rust 语言重写版 WeKnora：面向企业知识库、RAG 检索问答、Agent 编排和自托管部署的下一代实现。

本仓库地址：<https://github.com/oliviwee/WeKnora>

## 项目状态

WeKnora-Rust 当前处于 **foundation（基础设施）阶段**。本阶段不会一次性替换全部原有功能，而是先建立稳定、可测试、可渐进迁移的 Rust 服务骨架，再逐步移植知识库、文档解析、向量检索、聊天、Agent、MCP、Wiki 等核心模块。

当前已完成：

- Rust workspace：`rust/`
- Rust 服务 crate：`rust/weknora-rust`
- 无第三方依赖的最小 HTTP 服务入口
- `/healthz` 健康检查接口
- `/api/v1/version` 版本信息接口
- `/api/v1/capabilities` 迁移能力说明接口
- 统一 JSON 错误响应 envelope
- 基础 HTTP contract 测试

## 为什么重写为 Rust

WeKnora-Rust 的目标不是简单“翻译代码”，而是利用 Rust 在可靠性、并发安全和部署体积方面的优势，重新梳理 WeKnora 的服务边界：

- **更强的类型安全**：把 API、配置、错误、任务状态和数据模型尽量表达为编译期可检查的类型。
- **更可控的并发模型**：为后续文档处理、向量索引、Agent 工具调用和异步任务队列提供清晰边界。
- **更稳定的服务契约**：先保持 `/api/v1` wire contract，再逐步替换内部实现。
- **更轻量的部署路径**：为单二进制部署、边缘部署和资源受限环境预留空间。

## 快速开始

### 环境要求

- Rust 1.95+
- Cargo 1.95+

### 运行 Rust 服务

```bash
cd rust
cargo run -p weknora-rust
```

默认监听：

```text
0.0.0.0:8080
```

可以通过环境变量覆盖：

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `WEKNORA_SERVER_HOST` | `0.0.0.0` | 服务监听地址 |
| `WEKNORA_SERVER_PORT` | `8080` | 服务监听端口 |

### 测试接口

```bash
curl http://127.0.0.1:8080/healthz
curl http://127.0.0.1:8080/api/v1/version
curl http://127.0.0.1:8080/api/v1/capabilities
curl http://127.0.0.1:8080/api/v1/rewrite-plan
```

## Docker Compose 部署

WeKnora-Rust 提供独立的 Docker Compose 文件，适合验证 Rust 服务是否可以完整容器化启动。

```bash
docker compose -f docker-compose.rust.yml up --build
```

默认会构建 `docker/Dockerfile.rust`，启动 `weknora-rust` 服务，并将容器内 `8080` 端口映射到宿主机 `8080`。如果本机端口已被占用，可以覆盖宿主机端口：

```bash
WEKNORA_RUST_PORT=18080 docker compose -f docker-compose.rust.yml up --build
```

部署完成后可验证：

```bash
curl http://127.0.0.1:${WEKNORA_RUST_PORT:-8080}/healthz
curl http://127.0.0.1:${WEKNORA_RUST_PORT:-8080}/api/v1/version
curl http://127.0.0.1:${WEKNORA_RUST_PORT:-8080}/api/v1/capabilities
curl http://127.0.0.1:${WEKNORA_RUST_PORT:-8080}/api/v1/rewrite-plan
```

停止服务：

```bash
docker compose -f docker-compose.rust.yml down
```

## 开发命令

```bash
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo test --manifest-path rust/Cargo.toml
```

## 当前 API

### `GET /healthz`

返回 Rust 服务健康状态。

```json
{
  "status": "ok",
  "service": "weknora-rs",
  "version": "0.1.0"
}
```

### `GET /api/v1/version`

返回项目和 API 基础路径信息。

```json
{
  "name": "WeKnora-Rust",
  "version": "0.1.0",
  "api_base_path": "/api/v1",
  "implementation": "rust"
}
```

### `GET /api/v1/rewrite-plan`

返回 WeKnora-Rust 分模块语言选型和迁移计划。详细说明见 [`docs/WEKNORA_RUST_LANGUAGE_STRATEGY.md`](./docs/WEKNORA_RUST_LANGUAGE_STRATEGY.md)。

### `GET /api/v1/capabilities`

返回当前 Rust 迁移阶段、已实现能力和仍待迁移模块。

```json
{
  "migration_phase": "foundation",
  "implemented": ["health", "version", "capabilities", "typed-error-envelope"],
  "compatibility_stubs": ["auth", "knowledge-base", "knowledge", "chunks", "chat", "agents", "models", "datasources", "wiki", "mcp"]
}
```

## 迁移路线图

1. **基础设施**：配置、HTTP 路由、错误 envelope、健康检查、版本接口。
2. **API 契约**：对齐现有 `/api/v1` 响应结构，建立 Go/Rust 双实现 contract 测试。
3. **数据访问层**：抽象知识库、文档、chunk、session、agent 等 repository trait。
4. **检索与索引**：迁移 embedding、向量库、关键词检索和混合检索链路。
5. **RAG 与 Chat**：迁移问答、上下文组装、流式输出和会话管理。
6. **Agent / MCP / Wiki**：迁移复杂任务编排、工具调用审批、自动 Wiki 和知识图谱。
7. **部署切换**：完成兼容性测试后，再替换默认服务入口。

## 目录结构

```text
docker-compose.rust.yml         # WeKnora-Rust Docker Compose 部署文件
docker/Dockerfile.rust          # WeKnora-Rust 容器镜像构建文件
rust/
├── Cargo.toml                  # Rust workspace
├── README.md                   # Rust 子工程说明
└── weknora-rust/
    ├── Cargo.toml              # Rust 服务 crate
    ├── src/
    │   ├── api/                # API handler
    │   ├── domain/             # 响应 DTO / 领域结构
    │   ├── config.rs           # 环境变量配置
    │   ├── error.rs            # JSON 错误 envelope
    │   ├── http.rs             # 最小 HTTP request/response 层
    │   ├── lib.rs              # 路由分发入口
    │   └── main.rs             # TCP server 入口
    └── tests/                  # HTTP contract 测试
```

## 贡献说明

欢迎围绕 Rust 重写方向提交 issue 和 pull request。建议优先提交小而清晰的改动：

- 保持 `/api/v1` 对外契约稳定。
- 为新增接口补充 contract 测试。
- 优先抽象 trait，再落地具体数据库、向量库或存储实现。
- 不要在同一个 PR 中同时迁移多个业务域。

## License

MIT
