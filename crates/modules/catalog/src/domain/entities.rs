use chrono::{DateTime, Utc};
use uuid::Uuid;
use washco_shared::money::Money;

#[derive(Debug, Clone)]
pub struct Service {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub vehicle_type: VehicleType,
    pub base_price: Money,
    pub duration_minutes: i32,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Service {
    pub fn new(
        tenant_id: Uuid,
        location_id: Uuid,
        name: String,
        description: Option<String>,
        vehicle_type: VehicleType,
        base_price: Money,
        duration_minutes: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            tenant_id,
            location_id,
            name,
            description,
            vehicle_type,
            base_price,
            duration_minutes,
            is_active: true,
            sort_order: 0,
            created_at: now,
            updated_at: now,
            deleted_at: None,
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
