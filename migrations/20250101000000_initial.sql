-- Enable PostGIS
CREATE EXTENSION IF NOT EXISTS postgis;

-- Enable pg_trgm for text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- ============================================================
-- IDENTITY MODULE
-- ============================================================

CREATE TABLE tenants (
    id              UUID PRIMARY KEY,
    business_name   TEXT NOT NULL,
    owner_name      TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE users (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    phone           TEXT NOT NULL,
    name            TEXT NOT NULL,
    role            TEXT NOT NULL DEFAULT 'owner',
    is_verified     BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_users_phone ON users (phone) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_tenant ON users (tenant_id) WHERE deleted_at IS NULL;

-- ============================================================
-- LOCATION MODULE
-- ============================================================

CREATE TABLE locations (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    name            TEXT NOT NULL,
    slug            TEXT NOT NULL,
    phone           TEXT,
    address         TEXT NOT NULL,
    district        TEXT NOT NULL,
    city            TEXT NOT NULL,
    coordinates     GEOGRAPHY(POINT, 4326) NOT NULL,
    bay_count       SMALLINT NOT NULL DEFAULT 1,
    queue_mode      TEXT NOT NULL DEFAULT 'hybrid',
    status          TEXT NOT NULL DEFAULT 'pending',
    amenities       JSONB NOT NULL DEFAULT '[]',
    custom_fields   JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_locations_tenant_slug ON locations (tenant_id, slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_locations_geo ON locations USING GIST (coordinates) WHERE deleted_at IS NULL AND status = 'active';
CREATE INDEX idx_locations_city_district ON locations (city, district) WHERE deleted_at IS NULL AND status = 'active';

CREATE TABLE operating_hours (
    id              UUID PRIMARY KEY,
    location_id     UUID NOT NULL REFERENCES locations(id),
    day_of_week     SMALLINT NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
    open_time       TIME NOT NULL,
    close_time      TIME NOT NULL,
    is_closed       BOOLEAN NOT NULL DEFAULT false,
    UNIQUE (location_id, day_of_week)
);

CREATE TABLE bays (
    id              UUID PRIMARY KEY,
    location_id     UUID NOT NULL REFERENCES locations(id),
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    name            TEXT NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_bays_location ON bays (location_id) WHERE is_active = true;

-- ============================================================
-- CATALOG MODULE
-- ============================================================

CREATE TABLE services (
    id                  UUID PRIMARY KEY,
    tenant_id           UUID NOT NULL REFERENCES tenants(id),
    location_id         UUID NOT NULL REFERENCES locations(id),
    name                TEXT NOT NULL,
    description         TEXT,
    vehicle_type        TEXT NOT NULL,
    base_price          BIGINT NOT NULL,
    duration_minutes    INT NOT NULL,
    is_active           BOOLEAN NOT NULL DEFAULT true,
    sort_order          INT NOT NULL DEFAULT 0,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_services_location ON services (tenant_id, location_id) WHERE deleted_at IS NULL AND is_active = true;

-- ============================================================
-- QUEUE MODULE
-- ============================================================

CREATE TABLE queue_entries (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    queue_number    INT NOT NULL,
    customer_name   TEXT NOT NULL,
    customer_phone  TEXT,
    vehicle_type    TEXT NOT NULL,
    service_id      UUID NOT NULL REFERENCES services(id),
    service_name    TEXT NOT NULL,
    bay_id          UUID REFERENCES bays(id),
    status          TEXT NOT NULL DEFAULT 'waiting',
    joined_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at      TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ
);

CREATE INDEX idx_queue_active ON queue_entries (tenant_id, location_id, status)
    WHERE status IN ('waiting', 'in_progress');
CREATE INDEX idx_queue_daily ON queue_entries (location_id, joined_at);

-- ============================================================
-- OUTBOX
-- ============================================================

CREATE TABLE outbox_events (
    id              UUID PRIMARY KEY,
    event_type      TEXT NOT NULL,
    payload         JSONB NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at    TIMESTAMPTZ,
    error           TEXT,
    retry_count     INT NOT NULL DEFAULT 0
);

CREATE INDEX idx_outbox_pending ON outbox_events (created_at)
    WHERE processed_at IS NULL;
