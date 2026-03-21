use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerCondition {
    Rain,
    HeavyRain,
    SunnyHot,
    Cloudy,
}

impl TriggerCondition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rain => "rain",
            Self::HeavyRain => "heavy_rain",
            Self::SunnyHot => "sunny_hot",
            Self::Cloudy => "cloudy",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rain" => Some(Self::Rain),
            "heavy_rain" => Some(Self::HeavyRain),
            "sunny_hot" => Some(Self::SunnyHot),
            "cloudy" => Some(Self::Cloudy),
            _ => None,
        }
    }
}

impl std::fmt::Display for TriggerCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct WeatherTrigger {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub promotion_id: Uuid,
    pub location_id: Uuid,
    pub trigger_condition: TriggerCondition,
    pub auto_activate: bool,
    pub is_active: bool,
    pub last_triggered: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WeatherData {
    pub id: Uuid,
    pub city: String,
    pub temperature_c: Option<f64>,
    pub condition: String,
    pub humidity: Option<f64>,
    pub fetched_at: DateTime<Utc>,
    pub forecast_for: DateTime<Utc>,
}
