use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AdminAction {
    pub id: Uuid,
    pub admin_user_id: Uuid,
    pub action_type: String,
    pub target_type: String,
    pub target_id: Uuid,
    pub reason: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PlatformMetrics {
    pub total_locations: i64,
    pub active_locations: i64,
    pub pending_locations: i64,
    pub suspended_locations: i64,
    pub total_tenants: i64,
    pub total_revenue_today: i64,
}

#[derive(Debug, Clone)]
pub struct AdminLocationView {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub address: String,
    pub city: String,
    pub district: String,
    pub status: String,
    pub bay_count: i32,
    pub queue_mode: String,
    pub created_at: DateTime<Utc>,
    pub review_count: i64,
    pub avg_rating: f64,
}

#[derive(Debug, Clone)]
pub struct SubscriptionTier {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub max_locations: i32,
    pub max_staff: i32,
    pub features: serde_json::Value,
}
