use serde::{Deserialize, Serialize};

use crate::domain::{AdminAction, AdminLocationView, PlatformMetrics, SubscriptionTier};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SuspendRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminLocationResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub district: String,
    pub status: String,
    pub bay_count: i32,
    pub queue_mode: String,
    pub created_at: String,
    pub review_count: i64,
    pub avg_rating: f64,
}

impl From<AdminLocationView> for AdminLocationResponse {
    fn from(v: AdminLocationView) -> Self {
        Self {
            id: v.id.to_string(),
            tenant_id: v.tenant_id.to_string(),
            name: v.name,
            address: v.address,
            city: v.city,
            district: v.district,
            status: v.status,
            bay_count: v.bay_count,
            queue_mode: v.queue_mode,
            created_at: v.created_at.to_rfc3339(),
            review_count: v.review_count,
            avg_rating: v.avg_rating,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlatformMetricsResponse {
    pub total_locations: i64,
    pub active_locations: i64,
    pub pending_locations: i64,
    pub suspended_locations: i64,
    pub total_tenants: i64,
    pub total_revenue_today: i64,
}

impl From<PlatformMetrics> for PlatformMetricsResponse {
    fn from(m: PlatformMetrics) -> Self {
        Self {
            total_locations: m.total_locations,
            active_locations: m.active_locations,
            pending_locations: m.pending_locations,
            suspended_locations: m.suspended_locations,
            total_tenants: m.total_tenants,
            total_revenue_today: m.total_revenue_today,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AdminActionResponse {
    pub id: String,
    pub admin_user_id: String,
    pub action_type: String,
    pub target_type: String,
    pub target_id: String,
    pub reason: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: String,
}

impl From<AdminAction> for AdminActionResponse {
    fn from(a: AdminAction) -> Self {
        Self {
            id: a.id.to_string(),
            admin_user_id: a.admin_user_id.to_string(),
            action_type: a.action_type,
            target_type: a.target_type,
            target_id: a.target_id.to_string(),
            reason: a.reason,
            metadata: a.metadata,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TierResponse {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub max_locations: i32,
    pub max_staff: i32,
    pub features: serde_json::Value,
}

impl From<SubscriptionTier> for TierResponse {
    fn from(t: SubscriptionTier) -> Self {
        Self {
            id: t.id.to_string(),
            name: t.name,
            display_name: t.display_name,
            max_locations: t.max_locations,
            max_staff: t.max_staff,
            features: t.features,
        }
    }
}
