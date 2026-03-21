use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{WeatherData, WeatherTrigger};

#[derive(Debug, Deserialize)]
pub struct CreateTriggerRequest {
    pub promotion_id: Uuid,
    pub location_id: Uuid,
    pub trigger_condition: String,
    pub auto_activate: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTriggerRequest {
    pub trigger_condition: Option<String>,
    pub auto_activate: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListTriggersQuery {
    pub location_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ReportWeatherRequest {
    pub city: String,
    pub condition: String,
    pub temperature_c: Option<f64>,
    pub humidity: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub location_id: Uuid,
    pub condition: String,
}

#[derive(Debug, Serialize)]
pub struct WeatherTriggerResponse {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub location_id: Uuid,
    pub trigger_condition: String,
    pub auto_activate: bool,
    pub is_active: bool,
    pub last_triggered: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<WeatherTrigger> for WeatherTriggerResponse {
    fn from(t: WeatherTrigger) -> Self {
        Self {
            id: t.id,
            promotion_id: t.promotion_id,
            location_id: t.location_id,
            trigger_condition: t.trigger_condition.to_string(),
            auto_activate: t.auto_activate,
            is_active: t.is_active,
            last_triggered: t.last_triggered,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WeatherDataResponse {
    pub id: Uuid,
    pub city: String,
    pub temperature_c: Option<f64>,
    pub condition: String,
    pub humidity: Option<f64>,
    pub fetched_at: DateTime<Utc>,
    pub forecast_for: DateTime<Utc>,
}

impl From<WeatherData> for WeatherDataResponse {
    fn from(d: WeatherData) -> Self {
        Self {
            id: d.id,
            city: d.city,
            temperature_c: d.temperature_c,
            condition: d.condition,
            humidity: d.humidity,
            fetched_at: d.fetched_at,
            forecast_for: d.forecast_for,
        }
    }
}
