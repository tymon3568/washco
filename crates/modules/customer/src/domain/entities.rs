use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Customer {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub phone: String,
    pub name: String,
    pub email: Option<String>,
    pub segment: CustomerSegment,
    pub total_visits: i32,
    pub total_spent: i64,
    pub last_visit_at: Option<DateTime<Utc>>,
    pub loyalty_points: i32,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Customer {
    pub fn new(tenant_id: Uuid, phone: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            tenant_id,
            phone,
            name,
            email: None,
            segment: CustomerSegment::New,
            total_visits: 0,
            total_spent: 0,
            last_visit_at: None,
            loyalty_points: 0,
            notes: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Auto-segment based on visit count and recency.
    /// 0-1 visits: New, 2-9: Regular, 10+: Vip, >90 days no visit: Dormant.
    pub fn compute_segment(&mut self) {
        if let Some(last_visit) = self.last_visit_at {
            let days_since = (Utc::now() - last_visit).num_days();
            if days_since > 90 {
                self.segment = CustomerSegment::Dormant;
                return;
            }
        }
        self.segment = match self.total_visits {
            0..=1 => CustomerSegment::New,
            2..=9 => CustomerSegment::Regular,
            _ => CustomerSegment::Vip,
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CustomerSegment {
    New,
    Regular,
    Vip,
    Dormant,
}

impl CustomerSegment {
    pub fn as_str(&self) -> &'static str {
        match self {
            CustomerSegment::New => "new",
            CustomerSegment::Regular => "regular",
            CustomerSegment::Vip => "vip",
            CustomerSegment::Dormant => "dormant",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "new" => Some(CustomerSegment::New),
            "regular" => Some(CustomerSegment::Regular),
            "vip" => Some(CustomerSegment::Vip),
            "dormant" => Some(CustomerSegment::Dormant),
            _ => None,
        }
    }
}

impl std::fmt::Display for CustomerSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub plate_number: Option<String>,
    pub vehicle_type: VehicleType,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub year: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Vehicle {
    pub fn new(tenant_id: Uuid, customer_id: Uuid, vehicle_type: VehicleType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            tenant_id,
            customer_id,
            plate_number: None,
            vehicle_type,
            brand: None,
            model: None,
            color: None,
            year: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VehicleType {
    Motorbike,
    Sedan,
    Suv,
    Truck,
    Van,
}

impl VehicleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VehicleType::Motorbike => "motorbike",
            VehicleType::Sedan => "sedan",
            VehicleType::Suv => "suv",
            VehicleType::Truck => "truck",
            VehicleType::Van => "van",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "motorbike" => Some(VehicleType::Motorbike),
            "sedan" => Some(VehicleType::Sedan),
            "suv" => Some(VehicleType::Suv),
            "truck" => Some(VehicleType::Truck),
            "van" => Some(VehicleType::Van),
            _ => None,
        }
    }
}

impl std::fmt::Display for VehicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceHistoryEntry {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub vehicle_id: Uuid,
    pub customer_id: Uuid,
    pub location_id: Uuid,
    pub payment_id: Option<Uuid>,
    pub service_id: Uuid,
    pub service_name: String,
    pub amount_paid: i64,
    pub staff_name: Option<String>,
    pub notes: Option<String>,
    pub next_recommended_date: Option<NaiveDate>,
    pub next_recommended_service: Option<String>,
    pub serviced_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Membership {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub plan_name: String,
    pub plan_type: MembershipType,
    pub total_uses: Option<i32>,
    pub used_count: i32,
    pub price_paid: i64,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub status: MembershipStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MembershipType {
    WashCount,
    Monthly,
    Yearly,
}

impl MembershipType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MembershipType::WashCount => "wash_count",
            MembershipType::Monthly => "monthly",
            MembershipType::Yearly => "yearly",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "wash_count" => Some(MembershipType::WashCount),
            "monthly" => Some(MembershipType::Monthly),
            "yearly" => Some(MembershipType::Yearly),
            _ => None,
        }
    }
}

impl std::fmt::Display for MembershipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MembershipStatus {
    Active,
    Expired,
    Cancelled,
}

impl MembershipStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MembershipStatus::Active => "active",
            MembershipStatus::Expired => "expired",
            MembershipStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(MembershipStatus::Active),
            "expired" => Some(MembershipStatus::Expired),
            "cancelled" => Some(MembershipStatus::Cancelled),
            _ => None,
        }
    }
}

impl std::fmt::Display for MembershipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
