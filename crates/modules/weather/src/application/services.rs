use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{TriggerCondition, WeatherData, WeatherTrigger};

use super::ports::WeatherRepository;

pub struct WeatherService<R> {
    repo: R,
}

pub struct CreateWeatherTriggerInput {
    pub promotion_id: Uuid,
    pub location_id: Uuid,
    pub trigger_condition: String,
    pub auto_activate: bool,
}

impl<R: WeatherRepository> WeatherService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_trigger(
        &self,
        tenant_id: Uuid,
        input: CreateWeatherTriggerInput,
    ) -> Result<WeatherTrigger, AppError> {
        let condition = TriggerCondition::from_str(&input.trigger_condition)
            .ok_or_else(|| AppError::Validation {
                message: format!("Invalid trigger condition: {}", input.trigger_condition),
            })?;

        let now = Utc::now();
        let trigger = WeatherTrigger {
            id: Uuid::now_v7(),
            tenant_id,
            promotion_id: input.promotion_id,
            location_id: input.location_id,
            trigger_condition: condition,
            auto_activate: input.auto_activate,
            is_active: true,
            last_triggered: None,
            created_at: now,
            updated_at: now,
        };

        self.repo
            .create_trigger(&trigger)
            .await
            .map_err(AppError::Internal)?;

        Ok(trigger)
    }

    pub async fn list_triggers(
        &self,
        tenant_id: Uuid,
        location_id: Option<Uuid>,
    ) -> Result<Vec<WeatherTrigger>, AppError> {
        self.repo
            .list_triggers(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_trigger(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        trigger_condition: Option<String>,
        auto_activate: Option<bool>,
        is_active: Option<bool>,
    ) -> Result<WeatherTrigger, AppError> {
        let mut trigger = self
            .repo
            .find_trigger_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(AppError::NotFound {
                entity: "WeatherTrigger",
            })?;

        if let Some(cond) = trigger_condition {
            trigger.trigger_condition =
                TriggerCondition::from_str(&cond).ok_or_else(|| AppError::Validation {
                    message: format!("Invalid trigger condition: {cond}"),
                })?;
        }
        if let Some(auto) = auto_activate {
            trigger.auto_activate = auto;
        }
        if let Some(active) = is_active {
            trigger.is_active = active;
        }
        trigger.updated_at = Utc::now();

        self.repo
            .update_trigger(&trigger)
            .await
            .map_err(AppError::Internal)?;

        Ok(trigger)
    }

    pub async fn delete_trigger(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
        self.repo
            .find_trigger_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(AppError::NotFound {
                entity: "WeatherTrigger",
            })?;

        self.repo
            .delete_trigger(tenant_id, id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn report_weather(
        &self,
        city: &str,
        condition: &str,
        temperature_c: Option<f64>,
        humidity: Option<f64>,
    ) -> Result<WeatherData, AppError> {
        let now = Utc::now();
        let data = WeatherData {
            id: Uuid::now_v7(),
            city: city.to_string(),
            temperature_c,
            condition: condition.to_string(),
            humidity,
            fetched_at: now,
            forecast_for: now,
        };

        self.repo
            .upsert_weather_data(&data)
            .await
            .map_err(AppError::Internal)?;

        Ok(data)
    }

    pub async fn latest_weather(&self, city: &str) -> Result<Option<WeatherData>, AppError> {
        self.repo
            .latest_weather(city)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn evaluate_triggers(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        current_condition: &str,
    ) -> Result<Vec<WeatherTrigger>, AppError> {
        let triggers = self
            .repo
            .list_active_triggers_for_location(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)?;

        let mut matched = Vec::new();
        for trigger in triggers {
            if self.condition_matches(&trigger.trigger_condition, current_condition) {
                self.repo
                    .mark_triggered(tenant_id, trigger.id)
                    .await
                    .map_err(AppError::Internal)?;
                matched.push(trigger);
            }
        }

        Ok(matched)
    }

    fn condition_matches(&self, trigger_cond: &TriggerCondition, weather: &str) -> bool {
        match trigger_cond {
            TriggerCondition::Rain => weather == "rain" || weather == "heavy_rain",
            TriggerCondition::HeavyRain => weather == "heavy_rain" || weather == "storm",
            TriggerCondition::SunnyHot => weather == "sunny",
            TriggerCondition::Cloudy => weather == "cloudy",
        }
    }
}
