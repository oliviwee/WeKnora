# WeKnora-Rust Workspace

This directory contains the Rust rewrite track for **WeKnora-Rust**.

Repository: <https://github.com/oliviwee/WeKnora>

The workspace is intentionally isolated from the legacy Go, Python, frontend, and mini-program code so the rewrite can proceed incrementally without destabilizing the current product.

## Scope

The initial milestone provides a runnable Rust HTTP service with:

- Dependency-free HTTP routing under `/api/v1` for the foundation milestone.
- Health and version endpoints for deployment probes.
- A typed error envelope that can be reused by future RAG, knowledge-base, model-provider, and agent modules.
- Domain DTOs for capabilities and migration status so operators can distinguish implemented Rust features from compatibility stubs.

## Run

```bash
cd rust
cargo run -p weknora-rust
```

Environment variables:

| Variable | Default | Purpose |
| --- | --- | --- |
| `WEKNORA_SERVER_HOST` | `0.0.0.0` | Bind host for the Rust server. |
| `WEKNORA_SERVER_PORT` | `8080` | Bind port for the Rust server. |

## Rewrite Strategy

1. Keep wire contracts compatible with the existing `/api/v1` API before replacing handlers.
2. Port stateless boundary code first: configuration, error envelopes, routing, and observability.
3. Introduce repository traits for knowledge bases, documents, chunks, sessions, and agents.
4. Add database/storage/vector implementations behind those traits.
5. Move LLM provider adapters and ingestion workers module-by-module.
6. Switch deployment entry points only after parity tests pass against both Go and Rust services.
