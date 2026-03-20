use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::Location;

#[derive(Debug, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub address: String,
    pub district: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub phone: Option<String>,
    pub bay_count: i16,
    pub queue_mode: String,
    pub amenities: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub bay_count: Option<i16>,
    pub queue_mode: Option<String>,
    pub status: Option<String>,
    pub amenities: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NearbyQuery {
    pub lat: f64,
    pub lng: f64,
    #[serde(default = "default_radius")]
    pub radius_meters: f64,
}

fn default_radius() -> f64 {
    5000.0
}

#[derive(Debug, Serialize)]
pub struct LocationResponse {
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
    pub queue_mode: String,
    pub status: String,
    pub amenities: serde_json::Value,
}

impl From<Location> for LocationResponse {
    fn from(l: Location) -> Self {
        Self {
            id: l.id,
            tenant_id: l.tenant_id,
            name: l.name,
            slug: l.slug,
            phone: l.phone,
            address: l.address,
            district: l.district,
            city: l.city,
            latitude: l.latitude,
            longitude: l.longitude,
            bay_count: l.bay_count,
            queue_mode: l.queue_mode.to_string(),
            status: l.status.to_string(),
            amenities: l.amenities,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NearbyLocationResponse {
    #[serde(flatten)]
    pub location: LocationResponse,
    pub distance_meters: f64,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
