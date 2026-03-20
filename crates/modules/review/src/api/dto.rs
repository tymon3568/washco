use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Review, ReviewSummary};

#[derive(Debug, Deserialize)]
pub struct SubmitReviewRequest {
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub rating: i16,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListReviewsQuery {
    pub tenant_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SummaryQuery {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ReplyRequest {
    pub reply: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub rating: i16,
    pub comment: Option<String>,
    pub reply: Option<String>,
    pub replied_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Review> for ReviewResponse {
    fn from(r: Review) -> Self {
        Self {
            id: r.id,
            location_id: r.location_id,
            queue_entry_id: r.queue_entry_id,
            customer_name: r.customer_name,
            customer_phone: r.customer_phone,
            rating: r.rating,
            comment: r.comment,
            reply: r.reply,
            replied_at: r.replied_at,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ReviewSummaryResponse {
    pub location_id: Uuid,
    pub average_rating: f64,
    pub total_count: i64,
    pub distribution: [i64; 5],
}

impl ReviewSummaryResponse {
    pub fn from_summary(location_id: Uuid, summary: ReviewSummary) -> Self {
        Self {
            location_id,
            average_rating: summary.average_rating,
            total_count: summary.total_count,
            distribution: summary.distribution,
        }
    }
}
