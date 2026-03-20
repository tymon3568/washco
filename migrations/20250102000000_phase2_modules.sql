-- ============================================================
-- BOOKING MODULE
-- ============================================================

CREATE TABLE bookings (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    service_id      UUID NOT NULL REFERENCES services(id),
    customer_name   TEXT NOT NULL,
    customer_phone  TEXT NOT NULL,
    vehicle_type    TEXT NOT NULL,
    booking_date    DATE NOT NULL,
    time_slot       TIME NOT NULL,
    status          TEXT NOT NULL DEFAULT 'pending',
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    cancelled_at    TIMESTAMPTZ
);

CREATE INDEX idx_bookings_location_date ON bookings (tenant_id, location_id, booking_date, time_slot)
    WHERE status IN ('pending', 'confirmed');
CREATE INDEX idx_bookings_phone ON bookings (customer_phone, booking_date);

-- ============================================================
-- REVIEW MODULE
-- ============================================================

CREATE TABLE reviews (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    queue_entry_id  UUID REFERENCES queue_entries(id),
    customer_name   TEXT NOT NULL,
    customer_phone  TEXT,
    rating          SMALLINT NOT NULL CHECK (rating BETWEEN 1 AND 5),
    comment         TEXT,
    reply           TEXT,
    replied_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reviews_location ON reviews (tenant_id, location_id, created_at DESC);
CREATE INDEX idx_reviews_rating ON reviews (location_id, rating);

-- ============================================================
-- NOTIFICATION MODULE
-- ============================================================

CREATE TABLE notification_templates (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    template_type   TEXT NOT NULL,
    channel         TEXT NOT NULL DEFAULT 'sms',
    subject         TEXT,
    body_template   TEXT NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, template_type, channel)
);

CREATE TABLE notifications (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    recipient_phone TEXT NOT NULL,
    channel         TEXT NOT NULL DEFAULT 'sms',
    template_type   TEXT NOT NULL,
    payload         JSONB NOT NULL DEFAULT '{}',
    rendered_body   TEXT,
    status          TEXT NOT NULL DEFAULT 'pending',
    sent_at         TIMESTAMPTZ,
    error           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notifications_pending ON notifications (created_at)
    WHERE status = 'pending';
CREATE INDEX idx_notifications_tenant ON notifications (tenant_id, created_at DESC);

-- ============================================================
-- PRICING MODULE
-- ============================================================

CREATE TABLE pricing_rules (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    service_id      UUID REFERENCES services(id),
    name            TEXT NOT NULL,
    rule_type       TEXT NOT NULL,
    multiplier      FLOAT8 NOT NULL DEFAULT 1.0,
    fixed_adjustment BIGINT NOT NULL DEFAULT 0,
    conditions      JSONB NOT NULL DEFAULT '{}',
    priority        INT NOT NULL DEFAULT 0,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    valid_from      TIMESTAMPTZ,
    valid_to        TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_pricing_rules_location ON pricing_rules (tenant_id, location_id)
    WHERE is_active = true;

-- ============================================================
-- PROMOTION MODULE
-- ============================================================

CREATE TABLE promotions (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    code            TEXT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT,
    discount_type   TEXT NOT NULL,
    discount_value  BIGINT NOT NULL,
    min_order       BIGINT NOT NULL DEFAULT 0,
    max_uses        INT,
    used_count      INT NOT NULL DEFAULT 0,
    valid_from      TIMESTAMPTZ NOT NULL,
    valid_to        TIMESTAMPTZ NOT NULL,
    location_ids    UUID[] DEFAULT '{}',
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_promotions_code ON promotions (tenant_id, code) WHERE is_active = true;
CREATE INDEX idx_promotions_active ON promotions (tenant_id, valid_from, valid_to) WHERE is_active = true;
