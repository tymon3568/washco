use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::Booking;

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub service_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub vehicle_type: String,
    pub booking_date: chrono::NaiveDate,
    pub time_slot: chrono::NaiveTime,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub service_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub vehicle_type: String,
    pub booking_date: chrono::NaiveDate,
    pub time_slot: chrono::NaiveTime,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Booking> for BookingResponse {
    fn from(b: Booking) -> Self {
        Self {
            id: b.id,
            location_id: b.location_id,
            service_id: b.service_id,
            customer_name: b.customer_name,
            customer_phone: b.customer_phone,
            vehicle_type: b.vehicle_type,
            booking_date: b.booking_date,
            time_slot: b.time_slot,
            status: b.status.to_string(),
            notes: b.notes,
            created_at: b.created_at,
            updated_at: b.updated_at,
            cancelled_at: b.cancelled_at,
        }
    }
}
