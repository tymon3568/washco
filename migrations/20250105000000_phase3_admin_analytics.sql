-- Phase 3: Admin, Weather Triggers, Premium Tiers
-- ============================================================

-- Admin audit log
CREATE TABLE admin_actions (
    id              UUID PRIMARY KEY,
    admin_user_id   UUID NOT NULL REFERENCES users(id),
    action_type     TEXT NOT NULL,       -- approve_location | suspend_location | unsuspend_location
    target_type     TEXT NOT NULL,       -- location | tenant
    target_id       UUID NOT NULL,
    reason          TEXT,
    metadata        JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_admin_actions_target ON admin_actions (target_type, target_id, created_at DESC);
CREATE INDEX idx_admin_actions_admin ON admin_actions (admin_user_id, created_at DESC);

-- Weather data cache
CREATE TABLE weather_data (
    id              UUID PRIMARY KEY,
    city            TEXT NOT NULL,
    temperature_c   FLOAT8,
    condition       TEXT NOT NULL,       -- sunny | cloudy | rain | heavy_rain | storm
    humidity        FLOAT8,
    fetched_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    forecast_for    TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_weather_city_time ON weather_data (city, forecast_for DESC);

-- Weather trigger rules linked to promotions
CREATE TABLE weather_triggers (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    promotion_id    UUID NOT NULL REFERENCES promotions(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    trigger_condition TEXT NOT NULL,     -- rain | heavy_rain | sunny_hot | cloudy
    auto_activate   BOOLEAN NOT NULL DEFAULT false,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    last_triggered  TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_weather_triggers_location ON weather_triggers (tenant_id, location_id)
    WHERE is_active = true;

-- Premium tiers
CREATE TABLE subscription_tiers (
    id              UUID PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE,
    display_name    TEXT NOT NULL,
    max_locations   INT NOT NULL DEFAULT 1,
    max_staff       INT NOT NULL DEFAULT 5,
    features        JSONB NOT NULL DEFAULT '[]',
    sort_order      INT NOT NULL DEFAULT 0,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO subscription_tiers (id, name, display_name, max_locations, max_staff, features, sort_order) VALUES
    ('019578a0-0000-7000-8000-000000000001', 'free', 'Mien phi', 1, 5, '["basic_analytics","queue","catalog"]', 0),
    ('019578a0-0000-7000-8000-000000000002', 'basic', 'Co ban', 3, 15, '["basic_analytics","advanced_analytics","queue","catalog","booking","promotions"]', 1),
    ('019578a0-0000-7000-8000-000000000003', 'pro', 'Chuyen nghiep', 10, 50, '["basic_analytics","advanced_analytics","weather_triggers","multi_location","queue","catalog","booking","promotions","priority_support"]', 2);

-- Link tenants to tiers
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS tier_id UUID REFERENCES subscription_tiers(id);
UPDATE tenants SET tier_id = '019578a0-0000-7000-8000-000000000001' WHERE tier_id IS NULL;
