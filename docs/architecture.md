# WashCo Network - System Architecture

## 1. Architecture Overview

```
                    Clients (Browser, Zalo Mini App, Mobile Web)
                                     |
                              +------v------+
                              |   APISIX    |
                              | API Gateway |
                              |  :80/:443   |
                              +------+------+
                                     |
                 +-------------------+-------------------+
                 |                   |                    |
          +------v------+    +------v------+     +------v--------+
          | Owner Web   |    | Driver Web  |     |  REST API +   |
          | Dashboard   |    | Fallback    |     |  WebSocket    |
          | (SvelteKit) |    | (SvelteKit) |     |  (Axum)       |
          |  :3000      |    |  :3001      |     |  :8080        |
          +-------------+    +-------------+     +------+--------+
                                                        |
                                               +--------+---------+
                                               |  Rust Modular    |
                                               |   Monolith       |
                                               |                  |
                                               | +-------------+  |
                                               | | identity    |  |
                                               | | location    |  |
                                               | | catalog     |  |
                                               | | booking     |  |
                                               | | queue       |  |
                                               | | pricing     |  |
                                               | | review      |  |
                                               | | notification|  |
                                               | | analytics   |  |
                                               | | promotion   |  |
                                               | | admin       |  |
                                               | +-------------+  |
                                               +--+-----+------+--+
                                                  |     |      |
                                    +-------------+     |      +----------+
                                    |                   |                 |
                             +------v------+     +------v------+  +------v------+
                             |  PostgreSQL |     |    KeyDB    |  |   RustFS    |
                             |  + PostGIS  |     |   (Cache)   |  |  (Objects)  |
                             |   :5432     |     |   :6379     |  |   :9000     |
                             +-------------+     +-------------+  +-------------+

  Outbox Processor (background, same Rust binary)
       |
       +---> Zalo OA API / SMS Gateway / Email
```

All traffic enters through **APISIX** which handles:
- SSL termination and certificate management
- Route-based upstream selection (owner dashboard, driver web, API)
- Rate limiting, CORS, authentication plugins
- WebSocket proxying for real-time queue updates
- Health checks and load balancing

APISIX routing rules:
```
owner.washco.vn/*              -> owner-dashboard:3000
app.washco.vn/*                -> driver-web:3001
api.washco.vn/api/v1/*         -> washco-api:8080
api.washco.vn/ws/*             -> washco-api:8080 (WebSocket upgrade)
```

## 2. Tech Stack

### 2.1 Backend

| Layer          | Technology                  | Rationale                                                |
|----------------|-----------------------------|----------------------------------------------------------|
| Language       | **Rust**                    | Performance, safety, long-term maintainability           |
| Web Framework  | **Axum 0.8+**               | Async, tower ecosystem, extractors                       |
| Database       | **PostgreSQL 16+**          | ACID, PostGIS for geo queries, JSONB flexibility         |
| DB Driver      | **SQLx**                    | Compile-time checked queries, async, migration support   |
| Cache          | **KeyDB**                   | Redis-compatible, multi-threaded, pub/sub for real-time  |
| Object Storage | **RustFS**                  | S3-compatible, presigned URLs, photos/receipts           |
| Search         | **PostgreSQL + PostGIS**    | Location-based search, full-text search (pg_trgm)       |
| Auth           | **JWT (RS256)** + OTP       | Stateless API auth, phone-based verification             |
| Task Queue     | **Transactional Outbox**    | Reliable async processing (notifications, reports)       |
| Real-time      | **WebSocket (Axum)**        | Queue status, booking updates                            |

### 2.2 Frontend

| Layer              | Technology              | Rationale                                           |
|--------------------|-------------------------|-----------------------------------------------------|
| Owner Dashboard    | **SvelteKit 5 (Runes)** | Fast, lightweight, SSR support                      |
| UI Components      | **STDF** + Tailwind CSS | Design system consistency                           |
| Driver Mini App    | **Zalo Mini App SDK**   | Primary driver channel per PRD                      |
| Driver Web         | **SvelteKit 5**         | Fallback for shared links, store profiles           |
| Package Manager    | **Bun**                 | Fast installs, script execution                     |
| API Client         | **OpenAPI codegen**     | Type-safe API consumption                           |
| Maps               | **Mapbox GL JS**        | Location display, directions (Vietnam coverage)     |
| State Management   | **Svelte 5 $state**     | Runes-based, no external state library needed       |

### 2.3 Infrastructure

| Layer              | Technology              | Rationale                                           |
|--------------------|-------------------------|-----------------------------------------------------|
| Containerization   | **Podman + Compose**    | Rootless, daemonless, OCI-compatible                |
| API Gateway        | **Apache APISIX**       | Dynamic routing, plugins, SSL, rate limiting, CORS  |
| CI/CD              | **GitHub Actions**      | Automated testing, build, deploy                    |
| Monitoring         | **Prometheus + Grafana**| Metrics, alerting                                   |
| Logging            | **tracing + Loki**      | Structured logging, centralized                     |
| Hosting            | **VPS (Hetzner/DO)**    | Cost-effective for Vietnam market MVP               |

## 3. Backend Module Architecture (Modular Monolith)

```
crates/
  shared/                  # Shared types, errors, extractors, middleware
    src/
      auth/                # JWT validation, tenant context, role extractors
      error.rs             # AppError enum (small, stable)
      pagination.rs
      money.rs             # Money type (BIGINT-based, VND)
      lib.rs

  modules/
    identity/              # User accounts, authentication, roles
      src/
        api/               # Routes, DTOs, OpenAPI, wiring
        application/       # Use-cases: register, login, verify OTP
        domain/            # User, Role, TenantId, PhoneNumber
        infra/             # SQLx repos, OTP provider adapter

    location/              # Store profiles, hours, bays, amenities
      src/
        api/
        application/       # CRUD, geo search, capacity config
        domain/            # Location, Bay, OperatingHours, Address
        infra/             # PostGIS queries, photo storage adapter

    catalog/               # Service menu, vehicle types, pricing tiers
      src/
        api/
        application/
        domain/            # Service, VehicleType, PriceTier
        infra/

    booking/               # Reservations, time slots, grace periods
      src/
        api/
        application/       # Book, cancel, reschedule, no-show tracking
        domain/            # Booking, TimeSlot, GracePeriod, BookingPolicy
        infra/

    queue/                 # Walk-in queue, real-time status
      src/
        api/               # REST + WebSocket handlers
        application/       # Join, advance, complete, estimate wait
        domain/            # QueueEntry, QueuePosition, WaitEstimate
        infra/

    pricing/               # Price calculation, promotions, discounts
      src/
        api/
        application/       # Calculate price, apply promo, validate code
        domain/            # PriceRule, Promotion, PromoCode, Discount
        infra/

    review/                # Ratings, reviews, owner responses
      src/
        api/
        application/
        domain/            # Review, Rating, OwnerReply
        infra/

    notification/          # Push, SMS, Zalo OA messages
      src/
        api/
        application/       # Send, schedule, template rendering
        domain/            # Notification, Channel, Template
        infra/             # Zalo OA adapter, SMS gateway adapter

    analytics/             # Reporting, dashboards, metrics
      src/
        api/
        application/       # Daily summary, utilization, trends
        domain/            # Report, Metric, DateRange
        infra/             # Materialized view queries

    promotion/             # Campaigns, weather-triggered, off-peak
      src/
        api/
        application/       # Create campaign, trigger rules
        domain/            # Campaign, Trigger, TargetSegment
        infra/

    admin/                 # Platform admin: approve locations, metrics
      src/
        api/
        application/
        domain/
        infra/

  server/                  # Composition root: wires all modules, starts Axum
    src/
      main.rs
      router.rs            # Merges all module routers
      config.rs
      startup.rs           # DB pool, KeyDB, RustFS clients, DI wiring
```

### 3.1 Module Dependency Rules

```
identity  <-- location, booking, queue, review (need user/tenant context)
location  <-- booking, queue, catalog, review, analytics (need location info)
catalog   <-- booking, queue, pricing (need service definitions)
pricing   <-- booking (need price calculation)
booking   <-- queue (backfill canceled slots), analytics
queue     <-- analytics
notification <-- booking, queue, promotion (send messages)
```

Cross-module calls go through **facade DTOs only**, never internal domain types.

### 3.2 Key Database Schema Concepts

```sql
-- Tenant-scoped tables always include tenant_id
-- UUID v7 for primary keys
-- Money as BIGINT (minor units in VND, no decimals needed)
-- PostGIS for location coordinates
-- JSONB for custom_fields, amenities, metadata
-- deleted_at for soft deletes with partial indexes

-- Example: locations table
CREATE TABLE locations (
    id              UUID PRIMARY KEY,        -- UUID v7
    tenant_id       UUID NOT NULL,
    name            TEXT NOT NULL,
    slug            TEXT NOT NULL,
    phone           TEXT,
    address         TEXT NOT NULL,
    district        TEXT NOT NULL,
    city            TEXT NOT NULL,
    coordinates     GEOGRAPHY(POINT, 4326) NOT NULL,
    bay_count       SMALLINT NOT NULL DEFAULT 1,
    queue_mode      TEXT NOT NULL DEFAULT 'hybrid',  -- booking_only | walkin_only | hybrid
    status          TEXT NOT NULL DEFAULT 'pending',  -- pending | active | suspended
    amenities       JSONB DEFAULT '[]',
    custom_fields   JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMPTZ,
    UNIQUE (tenant_id, slug)
);

CREATE INDEX idx_locations_geo ON locations USING GIST (coordinates)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_locations_tenant ON locations (tenant_id)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_locations_city_district ON locations (city, district)
    WHERE deleted_at IS NULL AND status = 'active';
```

## 4. API Design

### 4.1 API Structure

```
/api/v1/
  auth/
    POST   /register              # Owner registration
    POST   /login                 # Phone + OTP
    POST   /otp/request           # Request OTP
    POST   /otp/verify            # Verify OTP
    POST   /refresh               # Refresh JWT

  locations/
    GET    /                      # List (owner's locations)
    POST   /                      # Create location
    GET    /:id                   # Get location detail
    PUT    /:id                   # Update location
    DELETE /:id                   # Soft delete

  catalog/
    GET    /locations/:id/services
    POST   /locations/:id/services
    PUT    /services/:id
    DELETE /services/:id

  bookings/
    GET    /locations/:id/bookings    # Owner: list bookings
    GET    /bookings/mine             # Driver: my bookings
    POST   /locations/:id/bookings    # Driver: create booking
    PUT    /bookings/:id/cancel       # Cancel
    PUT    /bookings/:id/check-in     # Check in
    PUT    /bookings/:id/complete     # Mark complete

  queue/
    GET    /locations/:id/queue       # Current queue state
    POST   /locations/:id/queue/join  # Driver: join queue
    PUT    /queue/:id/advance         # Owner: advance job
    PUT    /queue/:id/complete        # Owner: complete job
    WS     /locations/:id/queue/live  # WebSocket: real-time updates

  reviews/
    GET    /locations/:id/reviews
    POST   /locations/:id/reviews     # Driver: submit review
    POST   /reviews/:id/reply         # Owner: reply

  promotions/
    GET    /locations/:id/promotions
    POST   /locations/:id/promotions
    PUT    /promotions/:id
    DELETE /promotions/:id

  analytics/
    GET    /locations/:id/analytics/daily
    GET    /locations/:id/analytics/utilization
    GET    /locations/:id/analytics/services

  # Driver discovery (public/semi-public)
  discover/
    GET    /locations/nearby          # Geo search
    GET    /locations/:slug           # Public store profile
    GET    /locations/:id/availability # Available slots

  # Admin
  admin/
    GET    /locations                  # All locations
    PUT    /locations/:id/approve
    PUT    /locations/:id/suspend
    GET    /metrics
```

### 4.2 Authentication Flow

```
Driver (Zalo Mini App):
  1. Zalo login -> get Zalo access token
  2. POST /api/v1/auth/zalo-login { zalo_token }
  3. Backend verifies with Zalo API -> creates/finds user -> returns JWT
  4. For booking: require phone OTP verification

Owner:
  1. POST /api/v1/auth/register { phone, business_name, ... }
  2. OTP verification via SMS
  3. Returns JWT with tenant_id + role claims
```

## 5. Real-time Architecture

```
Driver opens queue view:
  1. Connect WS to /api/v1/locations/:id/queue/live
  2. Server subscribes to KeyDB channel: queue:{location_id}
  3. When owner advances/completes a job:
     - Update DB
     - Publish to KeyDB channel
     - All connected WS clients receive update
  4. Client updates UI with new position/wait estimate
```

Queue wait time estimation:

```
estimated_wait = (position_in_queue * avg_service_duration) / active_bay_count
```

## 6. Outbox Pattern for Reliable Side-effects

```
Transaction:
  1. INSERT booking
  2. INSERT outbox_events [
       { type: "booking.confirmed", payload: {...} },
       { type: "notification.send", payload: { channel: "zalo", ... } }
     ]
  3. COMMIT

Outbox Processor (background task):
  1. Poll outbox_events WHERE processed_at IS NULL ORDER BY created_at
  2. Process each event (send Zalo message, SMS, etc.)
  3. Mark processed_at = now()
  4. Retry with exponential backoff on failure
  5. Dead-letter after max retries
```

## 7. Zalo Mini App Architecture

```
zalo-mini-app/
  src/
    pages/
      index.zml              # Home / nearby locations
      location-detail.zml    # Store profile, services, reviews
      booking.zml            # Book a slot
      queue.zml              # Queue status (real-time)
      my-bookings.zml        # Booking history
      review.zml             # Submit review
    components/
      location-card.zml
      service-list.zml
      queue-status.zml
      map-view.zml
    api/
      client.js              # HTTP client to WashCo API
      auth.js                # Zalo auth + JWT management
    app.json                 # Zalo Mini App config
```

Zalo Mini App communicates with the same REST API + WebSocket endpoints.
Zalo-specific: login via Zalo SDK, share via Zalo social APIs, QR code deep links.

## 8. Deployment Architecture (MVP)

All services run as Podman containers behind APISIX gateway.

```
Single VPS (4-8 vCPU, 16GB RAM):
  podman compose:
    - apisix             (API gateway, SSL, routing)       :80/:443
    - apisix-dashboard   (APISIX admin UI)                 :9090
    - etcd               (APISIX config store)             :2379
    - washco-api         (Rust binary)                     :8080
    - washco-outbox      (Rust binary, same codebase)      (no port exposed)
    - owner-dashboard    (SvelteKit SSR, bun)              :3000
    - driver-web         (SvelteKit SSR, bun)              :3001
    - postgres           (with PostGIS)                    :5432
    - keydb              (cache + pub/sub)                 :6379
    - rustfs             (S3-compatible object storage)    :9000

  External:
    - Zalo Mini App (deployed via Zalo Developer Platform)
```

### 8.1 APISIX Route Configuration

```yaml
routes:
  - uri: /*
    host: owner.washco.vn
    upstream:
      type: roundrobin
      nodes:
        owner-dashboard:3000: 1
    plugins:
      cors: { allow_origins: "https://owner.washco.vn" }

  - uri: /*
    host: app.washco.vn
    upstream:
      type: roundrobin
      nodes:
        driver-web:3001: 1
    plugins:
      cors: { allow_origins: "https://app.washco.vn" }

  - uri: /api/v1/*
    host: api.washco.vn
    upstream:
      type: roundrobin
      nodes:
        washco-api:8080: 1
    plugins:
      cors: { allow_origins: "https://owner.washco.vn,https://app.washco.vn" }
      limit-req: { rate: 100, burst: 50 }

  - uri: /ws/*
    host: api.washco.vn
    upstream:
      type: roundrobin
      nodes:
        washco-api:8080: 1
    enable_websocket: true
```

### 8.2 SvelteKit Container Setup

Both SvelteKit apps are containerized with the `node` adapter and run via `bun`:

```dockerfile
# frontend/owner/Dockerfile
FROM oven/bun:1 AS builder
WORKDIR /app
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile
COPY . .
RUN bun run build

FROM oven/bun:1-slim
WORKDIR /app
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json .
COPY --from=builder /app/node_modules ./node_modules
EXPOSE 3000
ENV PORT=3000 HOST=0.0.0.0
CMD ["bun", "./build/index.js"]
```

### 8.3 Scaling Path

```
MVP:        Single VPS, all containers via podman compose
Growth:     Managed Postgres + replicate app containers across VPS nodes
Scale:      Extract hot modules (queue, booking) into separate services
            Add read replicas for analytics
            CDN for static assets + presigned URL delivery
```

## 9. Security

| Concern              | Approach                                                    |
|----------------------|-------------------------------------------------------------|
| Authentication       | JWT RS256, short-lived access (15min) + refresh (7d)        |
| Tenant isolation     | tenant_id in JWT claims, enforced at repo layer             |
| Rate limiting        | APISIX limit-req plugin + Tower middleware                   |
| Input validation     | Validate at API layer before use-case entry                 |
| SQL injection        | SQLx compile-time checked queries, parameterized            |
| File uploads         | Presigned URLs, never stream through backend                |
| Secrets              | Environment variables, never logged                         |
| CORS                 | APISIX cors plugin, whitelist per-route origins              |
| Audit                | Log pricing changes, booking mutations, admin actions        |

## 10. Development Workflow

```
# Local development (infrastructure via podman)
podman compose up -d postgres keydb rustfs apisix etcd   # Infrastructure
cargo run -p washco-server                                # Backend API (native)
cd frontend/owner && bun run dev                          # Owner dashboard (native)
cd frontend/driver-web && bun run dev                     # Driver web (native)

# Full stack (all containers)
podman compose up -d                                      # All services

# Testing
cargo test --workspace                                    # All Rust tests
cd frontend/owner && bun run test:unit                    # Frontend tests

# Quality gates
cargo fmt && cargo clippy --workspace -- -D warnings
cd frontend/owner && bun run check && bun run lint
```

## 11. Project Directory Structure

```
washco/
  docs/
    prd.md
    architecture.md
    api-spec.yaml           # OpenAPI spec (generated from Axum)

  crates/
    shared/
    modules/
      identity/
      location/
      catalog/
      booking/
      queue/
      pricing/
      review/
      notification/
      analytics/
      promotion/
      admin/
    server/
    Cargo.toml              # Workspace root

  frontend/
    owner/                  # SvelteKit 5 owner dashboard
      src/
        routes/
        lib/
      package.json
    driver-web/             # SvelteKit 5 driver web fallback
      src/
        routes/
        lib/
      package.json

  zalo-mini-app/            # Zalo Mini App (driver primary)
    src/

  migrations/               # SQLx migrations

  infra/
    apisix/
      config.yaml           # APISIX static config
      apisix.yaml           # Routes, upstreams
    podman/
      Containerfile.api     # Rust API build
      Containerfile.owner   # Owner dashboard build
      Containerfile.driver  # Driver web build

  compose.yaml              # Podman compose (all services)
  .github/
    workflows/
  CLAUDE.md
```

## 12. Phase Mapping to Modules

| Phase   | Modules to Build                                            |
|---------|-------------------------------------------------------------|
| Phase 1 | identity, location, catalog, queue (walk-in), analytics     |
|         | Owner dashboard: setup, daily ops, basic reporting          |
| Phase 2 | booking, pricing, promotion, review, notification           |
|         | Zalo Mini App: discovery, booking, queue, reviews           |
|         | Driver web fallback: store profiles, shared links           |
| Phase 3 | analytics (advanced), promotion (weather/auto), admin       |
|         | Multi-location views, optimization recommendations          |
