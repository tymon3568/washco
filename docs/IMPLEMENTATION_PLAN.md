# WashCo Implementation Plan

Kế hoạch triển khai tuần tự tất cả các hạng mục còn lại.

---

## Trạng thái hiện tại

### Đã hoàn thành
- 14 backend modules (identity, location, catalog, queue, analytics, booking, review, notification, pricing, promotion, payment, staff, customer, inventory)
- 14 modules wired in router.rs
- 4 database migrations (592 lines)
- Owner dashboard: 12 pages (dashboard, queue, bookings, catalog, customers, payments, staff, inventory, reviews, promotions, analytics, settings)
- Driver web: 5 pages (home/nearby, location detail, review, history, layout)
- Infrastructure: compose.yaml (13 services), APISIX routing, Containerfiles
- Vietnamese diacritics applied across all UI
- docs/CP.md common patterns

### Thiếu
- Notification frontend (owner)
- Pricing frontend (owner)
- Seed data script
- End-to-end testing
- Backend integration tests
- Frontend component tests
- Driver web Vietnamese diacritics check
- Production deployment finalization

---

## Phase A: Frontend còn thiếu

### A1. Notification page (Owner dashboard)
- [ ] Tạo `frontend/owner/src/routes/(dashboard)/notifications/+page.svelte`
- [ ] Tab 1: Danh sách thông báo đã gửi (GET `/notifications`)
- [ ] Tab 2: Quản lý mẫu thông báo (CRUD `/notifications/templates`)
- [ ] Tab 3: Gửi thông báo mới (POST `/notifications/send`)
- [ ] Thêm types vào `types.ts`
- [ ] Thêm nav item "Thông báo" + icon vào layout
- [ ] `bun run build && bun run check`

### A2. Pricing page (Owner dashboard)
- [ ] Tạo `frontend/owner/src/routes/(dashboard)/pricing/+page.svelte`
- [ ] Danh sách pricing rules theo location (GET `/pricing/locations/{id}/rules`)
- [ ] Form tạo rule mới (POST `/pricing/rules`) - rule types: Surge, TimeBased, DayOfWeek, Demand
- [ ] Sửa/Xóa rule (PUT/DELETE `/pricing/rules/{id}`)
- [ ] Công cụ tính giá thử (POST `/pricing/calculate`)
- [ ] Thêm types vào `types.ts`
- [ ] Thêm nav item "Định giá" + icon vào layout
- [ ] `bun run build && bun run check`

### A3. Driver web - Vietnamese diacritics
- [ ] Kiểm tra và sửa tiếng Việt có dấu cho tất cả driver pages
- [ ] `cd frontend/driver && bun run build && bun run check`

---

## Phase B: Seed data & E2E verification

### B1. Seed data script
- [ ] Tạo `scripts/seed.sql` với dữ liệu mẫu:
  - 1 tenant (WashCo Demo)
  - 1 owner user
  - 2 locations (HCM, Hanoi) với operating hours + bays
  - 10+ services per location
  - 5 staff members
  - 10 customers với vehicles
  - 5 pricing rules
  - 3 promotions
  - Sample queue entries
  - Sample bookings
  - Inventory materials
  - Notification templates
- [ ] Tạo `scripts/seed.sh` wrapper để chạy migration + seed

### B2. End-to-end verification
- [ ] Start infrastructure: `podman compose up -d postgres keydb`
- [ ] Run migrations: `sqlx migrate run`
- [ ] Seed data: `psql < scripts/seed.sql`
- [ ] Start backend: `cargo run -p washco-server`
- [ ] Start frontend: `cd frontend/owner && bun run dev`
- [ ] Verify từng page hoạt động với real API
- [ ] Fix mọi lỗi API mismatch
- [ ] Verify driver frontend similarly

---

## Phase C: Testing

### C1. Backend integration tests
- [ ] Tạo `crates/server/tests/` integration test framework
- [ ] Test identity: register → OTP → login → refresh → /me
- [ ] Test location: CRUD + nearby search
- [ ] Test catalog: service CRUD per location
- [ ] Test queue: join → advance → complete flow
- [ ] Test booking: create → confirm → complete
- [ ] Test payment: create → complete
- [ ] Test multi-tenancy isolation (2 tenants, verify no cross-access)

### C2. Frontend component tests
- [ ] Setup vitest + @testing-library/svelte
- [ ] Test API client error handling
- [ ] Test formatVND utility
- [ ] Test auth flow (login → OTP → register states)
- [ ] Test key page components (queue kanban, analytics cards)

---

## Phase D: Production deployment

### D1. Infrastructure finalization
- [ ] Verify APISIX routes cho tất cả endpoints
- [ ] SSL/TLS config (Let's Encrypt hoặc self-signed cho dev)
- [ ] Environment variables documentation
- [ ] Health check verification (/health, /live, /ready)

### D2. Build & deploy
- [ ] Build containers: `podman compose build`
- [ ] Test full stack: `podman compose up -d`
- [ ] Verify all services healthy
- [ ] Test owner dashboard qua APISIX (owner.washco.local)
- [ ] Test driver app qua APISIX (app.washco.local)
- [ ] Test API qua APISIX (rate limiting, WebSocket)

### D3. Backup & monitoring
- [ ] Verify backup script (`infra/scripts/backup-db.sh`)
- [ ] APISIX Prometheus metrics working
- [ ] Log aggregation setup (basic)

---

## Thứ tự thực hiện

```
A1 (Notification page) → A2 (Pricing page) → A3 (Driver diacritics)
    → commit & push
B1 (Seed data) → B2 (E2E verification & fixes)
    → commit & push
C1 (Backend tests) → C2 (Frontend tests)
    → commit & push
D1 (Infra finalize) → D2 (Full deploy) → D3 (Backup/monitoring)
    → commit & push
```

---

## Tracking

| Phase | Task | Status |
|-------|------|--------|
| A1 | Notification page | DONE |
| A2 | Pricing page | DONE |
| A3 | Driver diacritics | DONE |
| B1 | Seed data | DONE |
| B2 | E2E verification | DONE (72/72 endpoints) |
| C1 | Backend integration tests | DONE |
| C2 | Frontend component tests | DONE |
| D1 | Infra finalization | DONE |
| D2 | Build & deploy | DONE (10/10 services running) |
| D3 | Backup & monitoring | DONE |
