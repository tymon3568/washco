use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::Service;

#[derive(Debug, Deserialize)]
pub struct CreateServiceRequest {
    pub name: String,
    pub description: Option<String>,
    pub vehicle_type: String,
    pub base_price: i64,
    pub duration_minutes: i32,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServiceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_price: Option<i64>,
    pub duration_minutes: Option<i32>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ServiceResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub vehicle_type: String,
    pub base_price: i64,
    pub duration_minutes: i32,
    pub is_active: bool,
    pub sort_order: i32,
}

impl From<Service> for ServiceResponse {
    fn from(s: Service) -> Self {
        Self {
            id: s.id,
            location_id: s.location_id,
            name: s.name,
            description: s.description,
            vehicle_type: s.vehicle_type.to_string(),
            base_price: s.base_price.amount(),
            duration_minutes: s.duration_minutes,
            is_active: s.is_active,
            sort_order: s.sort_order,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
