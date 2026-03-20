use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Review {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub rating: i16,
    pub comment: Option<String>,
    pub reply: Option<String>,
    pub replied_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ReviewSummary {
    pub average_rating: f64,
    pub total_count: i64,
    pub distribution: [i64; 5],
}
