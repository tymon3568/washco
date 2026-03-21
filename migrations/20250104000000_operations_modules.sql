-- ============================================================
-- PAYMENT & BILLING MODULE
-- Giải quyết: Thất thoát doanh thu, "lủng túi", không bắn bill
-- ============================================================

CREATE TABLE payments (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    queue_entry_id  UUID REFERENCES queue_entries(id),
    booking_id      UUID REFERENCES bookings(id),
    customer_name   TEXT NOT NULL,
    customer_phone  TEXT,

    -- Dịch vụ và giá
    service_id      UUID NOT NULL REFERENCES services(id),
    service_name    TEXT NOT NULL,
    base_price      BIGINT NOT NULL,           -- Giá gốc (VND)
    discount_amount BIGINT NOT NULL DEFAULT 0,  -- Giảm giá
    final_amount    BIGINT NOT NULL,           -- Thực thu (VND)
    promotion_id    UUID REFERENCES promotions(id),

    -- Thanh toán
    payment_method  TEXT NOT NULL DEFAULT 'cash',  -- cash | bank_transfer | qr | e_wallet
    payment_status  TEXT NOT NULL DEFAULT 'pending', -- pending | completed | refunded
    paid_at         TIMESTAMPTZ,

    -- Ai thu tiền (audit trail)
    collected_by    UUID NOT NULL REFERENCES users(id),
    verified_by     UUID REFERENCES users(id),  -- Xác nhận bởi manager/owner

    -- Thợ thực hiện (cho tính hoa hồng)
    staff_id        UUID REFERENCES users(id),
    assistant_id    UUID REFERENCES users(id),  -- Thợ phụ

    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_payments_location_date ON payments (tenant_id, location_id, created_at DESC);
CREATE INDEX idx_payments_queue ON payments (queue_entry_id) WHERE queue_entry_id IS NOT NULL;
CREATE INDEX idx_payments_booking ON payments (booking_id) WHERE booking_id IS NOT NULL;
CREATE INDEX idx_payments_staff ON payments (staff_id, created_at DESC) WHERE staff_id IS NOT NULL;
CREATE INDEX idx_payments_daily ON payments (location_id, paid_at::date) WHERE payment_status = 'completed';

-- Bảng chi tiết dịch vụ add-on trong 1 lần thanh toán
CREATE TABLE payment_line_items (
    id              UUID PRIMARY KEY,
    payment_id      UUID NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    service_id      UUID REFERENCES services(id),
    description     TEXT NOT NULL,
    quantity        INT NOT NULL DEFAULT 1,
    unit_price      BIGINT NOT NULL,
    amount          BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_payment_lines ON payment_line_items (payment_id);

-- ============================================================
-- STAFF & COMMISSION MODULE
-- Giải quyết: Quản lý thợ, chia ca, tính hoa hồng
-- ============================================================

CREATE TABLE staff_profiles (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    user_id         UUID NOT NULL REFERENCES users(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    display_name    TEXT NOT NULL,
    skill_level     TEXT NOT NULL DEFAULT 'junior',  -- junior | senior | lead | detailer
    hourly_rate     BIGINT NOT NULL DEFAULT 0,        -- Lương theo giờ (VND)
    is_active       BOOLEAN NOT NULL DEFAULT true,
    joined_date     DATE NOT NULL DEFAULT CURRENT_DATE,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_staff_user_location ON staff_profiles (user_id, location_id);
CREATE INDEX idx_staff_location ON staff_profiles (tenant_id, location_id) WHERE is_active = true;

-- Ca làm việc
CREATE TABLE shifts (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    staff_id        UUID NOT NULL REFERENCES staff_profiles(id),
    shift_date      DATE NOT NULL,
    start_time      TIME NOT NULL,
    end_time        TIME NOT NULL,
    actual_start    TIMESTAMPTZ,
    actual_end      TIMESTAMPTZ,
    status          TEXT NOT NULL DEFAULT 'scheduled',  -- scheduled | active | completed | absent
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_shifts_schedule ON shifts (tenant_id, location_id, shift_date, start_time);
CREATE INDEX idx_shifts_staff ON shifts (staff_id, shift_date);

-- Rule tính hoa hồng
CREATE TABLE commission_rules (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    name            TEXT NOT NULL,
    service_id      UUID REFERENCES services(id),         -- NULL = áp dụng cho tất cả dịch vụ
    skill_level     TEXT,                                   -- NULL = áp dụng cho tất cả level
    role_in_job     TEXT NOT NULL DEFAULT 'primary',        -- primary | assistant
    commission_type TEXT NOT NULL DEFAULT 'percentage',     -- percentage | fixed
    commission_value BIGINT NOT NULL,                       -- % * 100 hoặc VND cố định
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_commission_rules_location ON commission_rules (tenant_id, location_id)
    WHERE is_active = true;

-- Ghi nhận hoa hồng thực tế cho từng job
CREATE TABLE commission_entries (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    payment_id      UUID NOT NULL REFERENCES payments(id),
    staff_id        UUID NOT NULL REFERENCES staff_profiles(id),
    rule_id         UUID REFERENCES commission_rules(id),
    role_in_job     TEXT NOT NULL DEFAULT 'primary',
    payment_amount  BIGINT NOT NULL,    -- Giá trị đơn hàng
    commission_amount BIGINT NOT NULL,  -- Hoa hồng thực nhận (VND)
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_commission_staff_date ON commission_entries (staff_id, created_at DESC);
CREATE INDEX idx_commission_payment ON commission_entries (payment_id);

-- ============================================================
-- CUSTOMER & VEHICLE MODULE
-- Giải quyết: Vòng đời khách hàng, LTV, upsell
-- ============================================================

CREATE TABLE customers (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    phone           TEXT NOT NULL,
    name            TEXT NOT NULL,
    email           TEXT,
    segment         TEXT NOT NULL DEFAULT 'regular',  -- new | regular | vip | dormant
    total_visits    INT NOT NULL DEFAULT 0,
    total_spent     BIGINT NOT NULL DEFAULT 0,         -- Tổng chi tiêu (VND)
    last_visit_at   TIMESTAMPTZ,
    loyalty_points  INT NOT NULL DEFAULT 0,
    notes           TEXT,
    tags            TEXT[] DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_customers_tenant_phone ON customers (tenant_id, phone);
CREATE INDEX idx_customers_segment ON customers (tenant_id, segment);
CREATE INDEX idx_customers_last_visit ON customers (tenant_id, last_visit_at DESC);

CREATE TABLE vehicles (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    customer_id     UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    plate_number    TEXT,
    vehicle_type    TEXT NOT NULL,        -- car | suv | truck | motorbike
    brand           TEXT,
    model           TEXT,
    color           TEXT,
    year            INT,
    notes           TEXT,                 -- VD: "Có vết xước cánh cửa trái"
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_vehicles_customer ON vehicles (customer_id);
CREATE UNIQUE INDEX idx_vehicles_plate ON vehicles (tenant_id, plate_number)
    WHERE plate_number IS NOT NULL;

-- Lịch sử chăm sóc từng xe
CREATE TABLE service_history (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    vehicle_id      UUID NOT NULL REFERENCES vehicles(id),
    customer_id     UUID NOT NULL REFERENCES customers(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    payment_id      UUID REFERENCES payments(id),
    service_id      UUID NOT NULL REFERENCES services(id),
    service_name    TEXT NOT NULL,
    amount_paid     BIGINT NOT NULL,
    staff_name      TEXT,
    notes           TEXT,
    next_recommended_date DATE,           -- Ngày khuyến nghị quay lại
    next_recommended_service TEXT,         -- VD: "Bảo dưỡng ceramic"
    serviced_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_service_history_vehicle ON service_history (vehicle_id, serviced_at DESC);
CREATE INDEX idx_service_history_customer ON service_history (customer_id, serviced_at DESC);
CREATE INDEX idx_service_history_reminder ON service_history (tenant_id, next_recommended_date)
    WHERE next_recommended_date IS NOT NULL;

-- Membership / Gói hội viên
CREATE TABLE memberships (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    customer_id     UUID NOT NULL REFERENCES customers(id),
    plan_name       TEXT NOT NULL,
    plan_type       TEXT NOT NULL DEFAULT 'wash_count',  -- wash_count | monthly | yearly
    total_uses      INT,                    -- NULL = unlimited
    used_count      INT NOT NULL DEFAULT 0,
    price_paid      BIGINT NOT NULL,
    valid_from      DATE NOT NULL,
    valid_to        DATE,
    status          TEXT NOT NULL DEFAULT 'active',  -- active | expired | cancelled
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_memberships_customer ON memberships (customer_id) WHERE status = 'active';
CREATE INDEX idx_memberships_expiring ON memberships (tenant_id, valid_to)
    WHERE status = 'active' AND valid_to IS NOT NULL;

-- ============================================================
-- INVENTORY MODULE
-- Giải quyết: Hao hụt hóa chất, kiểm soát vật tư
-- ============================================================

-- Danh mục vật tư
CREATE TABLE materials (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    location_id     UUID NOT NULL REFERENCES locations(id),
    name            TEXT NOT NULL,
    category        TEXT NOT NULL DEFAULT 'chemical',  -- chemical | accessory | consumable | equipment
    unit            TEXT NOT NULL DEFAULT 'ml',         -- ml | liter | piece | kg | gram
    unit_cost       BIGINT NOT NULL DEFAULT 0,          -- Giá nhập per unit (VND)
    current_stock   BIGINT NOT NULL DEFAULT 0,          -- Tồn kho hiện tại (theo unit)
    min_stock       BIGINT NOT NULL DEFAULT 0,          -- Ngưỡng cảnh báo
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_materials_location ON materials (tenant_id, location_id) WHERE is_active = true;
CREATE INDEX idx_materials_low_stock ON materials (tenant_id, location_id)
    WHERE is_active = true AND current_stock <= min_stock;

-- Định mức vật tư cho từng dịch vụ
CREATE TABLE material_norms (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    service_id      UUID NOT NULL REFERENCES services(id),
    material_id     UUID NOT NULL REFERENCES materials(id),
    quantity_per_job BIGINT NOT NULL,     -- Lượng tiêu thụ chuẩn per job
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (service_id, material_id)
);

-- Xuất nhập tồn
CREATE TABLE inventory_transactions (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    material_id     UUID NOT NULL REFERENCES materials(id),
    transaction_type TEXT NOT NULL,       -- purchase | usage | adjustment | return | waste
    quantity        BIGINT NOT NULL,      -- Dương = nhập, âm = xuất
    unit_cost       BIGINT,              -- Giá đơn vị (cho nhập hàng)
    reference_id    UUID,                -- FK tới payment_id hoặc purchase order
    reference_type  TEXT,                -- 'payment' | 'purchase' | 'adjustment'
    notes           TEXT,
    performed_by    UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_inventory_tx_material ON inventory_transactions (material_id, created_at DESC);
CREATE INDEX idx_inventory_tx_date ON inventory_transactions (tenant_id, created_at DESC);

-- View: So sánh tiêu thụ thực tế vs định mức
-- (Dùng cho báo cáo hao hụt)
CREATE OR REPLACE VIEW v_material_variance AS
SELECT
    p.tenant_id,
    p.location_id,
    p.service_id,
    mn.material_id,
    m.name AS material_name,
    m.unit,
    COUNT(p.id) AS job_count,
    COUNT(p.id) * mn.quantity_per_job AS expected_usage,
    COALESCE(SUM(ABS(it.quantity)), 0) AS actual_usage,
    COALESCE(SUM(ABS(it.quantity)), 0) - (COUNT(p.id) * mn.quantity_per_job) AS variance
FROM payments p
JOIN material_norms mn ON mn.service_id = p.service_id AND mn.tenant_id = p.tenant_id
JOIN materials m ON m.id = mn.material_id
LEFT JOIN inventory_transactions it ON it.reference_id = p.id
    AND it.material_id = mn.material_id
    AND it.transaction_type = 'usage'
WHERE p.payment_status = 'completed'
GROUP BY p.tenant_id, p.location_id, p.service_id, mn.material_id, m.name, m.unit, mn.quantity_per_job;
