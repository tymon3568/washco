use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::WeatherRepository;
use crate::domain::{TriggerCondition, WeatherData, WeatherTrigger};

pub struct PgWeatherRepository {
    pool: PgPool,
}

impl PgWeatherRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_trigger(row: &sqlx::postgres::PgRow) -> WeatherTrigger {
    let cond_str: String = row.get("trigger_condition");
    WeatherTrigger {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        promotion_id: row.get("promotion_id"),
        location_id: row.get("location_id"),
        trigger_condition: TriggerCondition::from_str(&cond_str).unwrap_or(TriggerCondition::Rain),
        auto_activate: row.get("auto_activate"),
        is_active: row.get("is_active"),
        last_triggered: row.get("last_triggered"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_weather(row: &sqlx::postgres::PgRow) -> WeatherData {
    WeatherData {
        id: row.get("id"),
        city: row.get("city"),
        temperature_c: row.get("temperature_c"),
        condition: row.get("condition"),
        humidity: row.get("humidity"),
        fetched_at: row.get("fetched_at"),
        forecast_for: row.get("forecast_for"),
    }
}

impl WeatherRepository for PgWeatherRepository {
    async fn create_trigger(&self, trigger: &WeatherTrigger) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO weather_triggers
               (id, tenant_id, promotion_id, location_id, trigger_condition,
                auto_activate, is_active, last_triggered, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        )
        .bind(trigger.id)
        .bind(trigger.tenant_id)
        .bind(trigger.promotion_id)
        .bind(trigger.location_id)
        .bind(trigger.trigger_condition.as_str())
        .bind(trigger.auto_activate)
        .bind(trigger.is_active)
        .bind(trigger.last_triggered)
        .bind(trigger.created_at)
        .bind(trigger.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_trigger_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> anyhow::Result<Option<WeatherTrigger>> {
        let row = sqlx::query(
            "SELECT * FROM weather_triggers WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.as_ref().map(row_to_trigger))
    }

    async fn list_triggers(
        &self,
        tenant_id: Uuid,
        location_id: Option<Uuid>,
    ) -> anyhow::Result<Vec<WeatherTrigger>> {
        let rows = match location_id {
            Some(loc_id) => {
                sqlx::query(
                    "SELECT * FROM weather_triggers WHERE tenant_id = $1 AND location_id = $2 ORDER BY created_at DESC",
                )
                .bind(tenant_id)
                .bind(loc_id)
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query(
                    "SELECT * FROM weather_triggers WHERE tenant_id = $1 ORDER BY created_at DESC",
                )
                .bind(tenant_id)
                .fetch_all(&self.pool)
                .await?
            }
        };
        Ok(rows.iter().map(row_to_trigger).collect())
    }

    async fn update_trigger(&self, trigger: &WeatherTrigger) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE weather_triggers SET
               trigger_condition = $1, auto_activate = $2, is_active = $3, updated_at = $4
               WHERE id = $5 AND tenant_id = $6"#,
        )
        .bind(trigger.trigger_condition.as_str())
        .bind(trigger.auto_activate)
        .bind(trigger.is_active)
        .bind(trigger.updated_at)
        .bind(trigger.id)
        .bind(trigger.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_trigger(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM weather_triggers WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_active_triggers_for_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<WeatherTrigger>> {
        let rows = sqlx::query(
            "SELECT * FROM weather_triggers WHERE tenant_id = $1 AND location_id = $2 AND is_active = true",
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(row_to_trigger).collect())
    }

    async fn mark_triggered(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE weather_triggers SET last_triggered = now(), updated_at = now() WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_weather_data(&self, data: &WeatherData) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO weather_data (id, city, temperature_c, condition, humidity, fetched_at, forecast_for)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(data.id)
        .bind(&data.city)
        .bind(data.temperature_c)
        .bind(&data.condition)
        .bind(data.humidity)
        .bind(data.fetched_at)
        .bind(data.forecast_for)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn latest_weather(&self, city: &str) -> anyhow::Result<Option<WeatherData>> {
        let row = sqlx::query(
            "SELECT * FROM weather_data WHERE city = $1 ORDER BY fetched_at DESC LIMIT 1",
        )
        .bind(city)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.as_ref().map(row_to_weather))
    }
}
