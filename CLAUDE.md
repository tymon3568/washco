# WashCo Network

## Project Overview
Two-sided car wash marketplace for Vietnam. Owner SaaS + Driver discovery via Zalo Mini App.

## Tech Stack
- Backend: Rust (Axum + SQLx + PostgreSQL/PostGIS + KeyDB + RustFS)
- Frontend: SvelteKit 5 (Runes) + shadcn-svelte + Tailwind CSS 4
- Infrastructure: Podman + APISIX + etcd
- Package manager: bun (frontend)

## Dev Server Startup

### Prerequisites
```bash
podman compose up -d postgres keydb     # Required: DB + cache
# Wait for healthy: podman compose ps
```

### Start all dev servers (3 terminals)
```bash
# Terminal 1: Backend API (port 8080)
cargo run -p washco-server

# Terminal 2: Owner dashboard (port 5173 via Vite)
cd frontend/owner && bun run dev

# Terminal 3: Driver PWA (port 3001 via Vite)
cd frontend/driver && bun run dev
```

### Port Map
| Service            | Dev (Vite)  | APISIX (prod-like) |
|--------------------|-------------|---------------------|
| Backend API        | :8080       | —                   |
| Owner dashboard    | **:5173**   | :8088               |
| Driver PWA         | **:3001**   | :8089               |
| APISIX dashboard   | —           | :9090               |
| PostgreSQL         | :5432       | —                   |
| KeyDB              | :6379       | —                   |
| RustFS console     | :9001       | —                   |

**IMPORTANT:** During development, always use Vite dev ports (:5173, :3001), NOT APISIX ports.
Vite proxies `/api/*` → `localhost:8080` and `/api/v1/queue/ws/*` → `ws://localhost:8080`.

### Test Credentials
- Phone: `0932640968`, OTP bypass: `000000` (dev mode)
- Tenant: tymon, Email: tymon3568@gmail.com, Pass: Admin@123
- Tymon tenant_id: `019d0d8f-79b4-77d0-866c-601b48bf53fc`
- Tymon location_id: `019d0ddb-3b48-7510-983d-b17be7ee62d9`

### DB Access
```bash
PGPASSWORD=washco psql -h localhost -U washco -d washco
```

## Build & QA Commands

```bash
# Backend
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt
cargo run -p washco-server

# Frontend (owner)
cd frontend/owner && bun run dev
cd frontend/owner && bun run build
cd frontend/owner && bun run check
cd frontend/owner && bun run lint
cd frontend/owner && bun run test:unit

# Frontend (driver)
cd frontend/driver && bun run dev
cd frontend/driver && bun run build
cd frontend/driver && bun run check

# Infrastructure
podman compose up -d                    # All services (production-like)
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

## Current Phase: Phase 2+
Modules: identity, location, catalog, queue, booking, review, promotion, analytics, admin
Frontend: Owner dashboard + Driver PWA
