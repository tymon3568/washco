use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::QueueEntry;

#[derive(Debug, Deserialize)]
pub struct JoinQueueRequest {
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub vehicle_type: String,
    pub service_id: Uuid,
    pub service_name: String,
}

#[derive(Debug, Deserialize)]
pub struct AdvanceRequest {
    pub bay_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct QueueEntryResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub queue_number: i32,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub vehicle_type: String,
    pub service_id: Uuid,
    pub service_name: String,
    pub bay_id: Option<Uuid>,
    pub status: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<QueueEntry> for QueueEntryResponse {
    fn from(e: QueueEntry) -> Self {
        Self {
            id: e.id,
            location_id: e.location_id,
            queue_number: e.queue_number,
            customer_name: e.customer_name,
            customer_phone: e.customer_phone,
            vehicle_type: e.vehicle_type,
            service_id: e.service_id,
            service_name: e.service_name,
            bay_id: e.bay_id,
            status: e.status.to_string(),
            joined_at: e.joined_at,
            started_at: e.started_at,
            completed_at: e.completed_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct QueueStateResponse {
    pub location_id: Uuid,
    pub waiting: Vec<QueueEntryResponse>,
    pub in_progress: Vec<QueueEntryResponse>,
    pub completed_today: i64,
    pub estimated_wait_minutes: i32,
}
