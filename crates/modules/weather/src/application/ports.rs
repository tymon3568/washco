use uuid::Uuid;

use crate::domain::{WeatherData, WeatherTrigger};

pub trait WeatherRepository: Send + Sync {
    fn create_trigger(
        &self,
        trigger: &WeatherTrigger,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_trigger_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<WeatherTrigger>>> + Send;

    fn list_triggers(
        &self,
        tenant_id: Uuid,
        location_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<WeatherTrigger>>> + Send;

    fn update_trigger(
        &self,
        trigger: &WeatherTrigger,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn delete_trigger(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn list_active_triggers_for_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<WeatherTrigger>>> + Send;

    fn mark_triggered(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn upsert_weather_data(
        &self,
        data: &WeatherData,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn latest_weather(
        &self,
        city: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<WeatherData>>> + Send;
}
