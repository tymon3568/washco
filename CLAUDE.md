# WashCo Network

## Project Overview
Two-sided car wash marketplace for Vietnam. Owner SaaS + Driver discovery via Zalo Mini App.

## Tech Stack
- Backend: Rust (Axum + SQLx + PostgreSQL/PostGIS + KeyDB + RustFS)
- Frontend: SvelteKit 5 (Runes) + shadcn-svelte + Tailwind CSS 4
- Infrastructure: Podman + APISIX + etcd
- Package manager: bun (frontend)

## Commands

```bash
# Backend
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt
cargo run -p washco-server

# Frontend (owner dashboard)
cd frontend/owner && bun run dev
cd frontend/owner && bun run build
cd frontend/owner && bun run check
cd frontend/owner && bun run lint
cd frontend/owner && bun run test:unit

# Infrastructure
podman compose up -d                    # All services
podman compose up -d postgres keydb     # Dev infra only
```

## Architecture Rules
- Modular monolith: each module in crates/modules/<name>/
- Module layers: api/ -> application/ -> domain/ + infra/
- application/ MUST NOT import infra/
- Cross-module via facade DTOs only, never domain types
- Tenant isolation: every tenant-scoped query must filter by tenant_id
- Money: BIGINT minor units (VND has no decimals)
- UUIDs: v7 for primary keys
- Outbox pattern for reliable side-effects (notifications, SMS)
- Presigned URLs for file uploads (never stream through backend)

## Current Phase: Phase 1
Modules: identity, location, catalog, queue (walk-in), analytics
Frontend: Owner dashboard
