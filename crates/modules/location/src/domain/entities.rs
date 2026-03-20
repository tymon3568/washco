use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub phone: Option<String>,
    pub address: String,
    pub district: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub bay_count: i16,
    pub queue_mode: QueueMode,
    pub status: LocationStatus,
    pub amenities: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Location {
    pub fn new(
        tenant_id: Uuid,
        name: String,
        phone: Option<String>,
        address: String,
        district: String,
        city: String,
        latitude: f64,
        longitude: f64,
        bay_count: i16,
        queue_mode: QueueMode,
        amenities: serde_json::Value,
    ) -> Self {
        let now = Utc::now();
        let slug = Self::slugify(&name);
        Self {
            id: Uuid::now_v7(),
            tenant_id,
            name,
            slug,
            phone,
            address,
            district,
            city,
            latitude,
            longitude,
            bay_count,
            queue_mode,
            status: LocationStatus::Pending,
            amenities,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn slugify(name: &str) -> String {
        name.to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueueMode {
    BookingOnly,
    WalkinOnly,
    Hybrid,
}

impl QueueMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BookingOnly => "booking_only",
            Self::WalkinOnly => "walkin_only",
            Self::Hybrid => "hybrid",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "booking_only" => Some(Self::BookingOnly),
            "walkin_only" => Some(Self::WalkinOnly),
            "hybrid" => Some(Self::Hybrid),
            _ => None,
        }
    }
}

impl std::fmt::Display for QueueMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationStatus {
    Pending,
    Active,
    Suspended,
}

impl LocationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Suspended => "suspended",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "active" => Some(Self::Active),
            "suspended" => Some(Self::Suspended),
            _ => None,
        }
    }
}

impl std::fmt::Display for LocationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct OperatingHours {
    pub id: Uuid,
    pub location_id: Uuid,
    pub tenant_id: Uuid,
    pub day_of_week: i16,
    pub open_time: chrono::NaiveTime,
    pub close_time: chrono::NaiveTime,
    pub is_closed: bool,
}

impl OperatingHours {
    pub fn new(
        location_id: Uuid,
        tenant_id: Uuid,
        day_of_week: i16,
        open_time: chrono::NaiveTime,
        close_time: chrono::NaiveTime,
        is_closed: bool,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            location_id,
            tenant_id,
            day_of_week,
            open_time,
            close_time,
            is_closed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bay {
    pub id: Uuid,
    pub location_id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub is_active: bool,
}

impl Bay {
    pub fn new(location_id: Uuid, tenant_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            location_id,
            tenant_id,
            name,
            is_active: true,
        }
    }
}
