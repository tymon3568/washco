# WashCo Deployment Guide

## Environment Variables

### Required
| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://washco:secret@postgres:5432/washco` |
| `JWT_SECRET` | JWT signing secret (change in production!) | `your-32-char-random-string` |

### Optional (with defaults)
| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Server bind address |
| `PORT` | `8080` | Server port |
| `DB_MAX_CONNECTIONS` | `50` | PostgreSQL pool size |
| `DB_IDLE_TIMEOUT_SECONDS` | `300` | Idle connection timeout |
| `DB_MAX_LIFETIME_SECONDS` | `1800` | Max connection lifetime |
| `REDIS_URL` | `redis://localhost:6379` | KeyDB/Redis URL |
| `S3_ENDPOINT` | `http://localhost:9000` | RustFS/S3 endpoint |
| `S3_BUCKET` | `washco` | S3 bucket name |
| `S3_ACCESS_KEY` | `washco` | S3 access key |
| `S3_SECRET_KEY` | `washco-secret` | S3 secret key |
| `S3_REGION` | `us-east-1` | S3 region |
| `ALLOWED_ORIGINS` | `http://localhost:5173,...` | CORS origins (comma-separated) |
| `RUST_LOG` | `washco=info,tower_http=info` | Log level filter |
| `POSTGRES_PASSWORD` | `washco` | PostgreSQL password (compose) |

### Frontend
| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000`/`3001` | SvelteKit server port |
| `ORIGIN` | `http://localhost` | SvelteKit origin for CSRF |
| `PUBLIC_API_URL` | `/api/v1` | API base URL |

---

## Development Setup

```bash
# 1. Start infrastructure only
podman compose up -d postgres keydb

# 2. Run migrations + seed
./scripts/seed.sh

# 3. Start backend
cargo run -p washco-server

# 4. Start owner frontend
cd frontend/owner && bun run dev

# 5. Start driver frontend (separate terminal)
cd frontend/driver && bun run dev
```

Access:
- Owner dashboard: http://localhost:5173
- Driver web: http://localhost:5174
- API: http://localhost:8080/health

---

## Production Deployment (Podman)

### 1. Create `.env` file

```bash
cp .env.example .env
# Edit with production values:
# - JWT_SECRET (generate: openssl rand -hex 32)
# - POSTGRES_PASSWORD
# - S3_ACCESS_KEY / S3_SECRET_KEY
# - ALLOWED_ORIGINS
```

### 2. Build and start all services

```bash
podman compose build
podman compose up -d
```

### 3. Verify services

```bash
# Check all services are healthy
podman compose ps

# Test health endpoints
curl http://localhost/api/v1/health  # via APISIX
curl http://localhost:8080/health    # direct
curl http://localhost:8080/ready     # DB check
```

### 4. Access via APISIX

Add to `/etc/hosts` for local testing:
```
127.0.0.1 owner.washco.local app.washco.local api.washco.local
```

| Host | Service | Port |
|------|---------|------|
| `owner.washco.local` | Owner dashboard | 80 |
| `app.washco.local` | Driver app | 80 |
| `api.washco.local/api/v1/*` | API | 80 |
| `localhost` | Owner dashboard (fallback) | 80 |
| `localhost/api/v1/*` | API (dev) | 80 |

---

## Service Architecture

```
[Browser] → [APISIX :80] → [owner-dashboard :3000]
                         → [driver-app :3001]
                         → [washco-api :8080] → [PostgreSQL :5432]
                                               → [KeyDB :6379]
                                               → [RustFS :9000]
         → [washco-outbox] (background worker)
```

---

## Database

### Migrations
Migrations run automatically on server startup (`sqlx::migrate!`).

Manual run:
```bash
export DATABASE_URL=postgres://washco:washco@localhost:5432/washco
sqlx migrate run --source migrations/
```

### Backup
```bash
./infra/scripts/backup-db.sh
```

### Seed data (demo/dev)
```bash
./scripts/seed.sh        # migrations + seed
./scripts/seed.sh --reset # drop + recreate + migrate + seed
```

---

## Monitoring

- **Health checks**: `/health`, `/live`, `/ready` endpoints
- **APISIX metrics**: Prometheus at `http://apisix:9091/apisix/prometheus/metrics`
- **APISIX dashboard**: `http://localhost:9090`
- **Logs**: `podman compose logs -f washco-api`

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| API 502 | Check `podman compose logs washco-api` - DB connection? |
| CORS errors | Verify `ALLOWED_ORIGINS` includes frontend URLs |
| JWT expired | Token refresh should handle automatically; check `JWT_EXPIRY_SECONDS` |
| Migration fails | Check `DATABASE_URL` and PostgreSQL availability |
| S3 upload fails | Ensure RustFS is running and bucket exists |
