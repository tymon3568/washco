use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use washco_shared::money::Money;

use crate::application::ServiceRepository;
use crate::domain::{Service, VehicleType};

pub struct PgServiceRepository {
    pool: PgPool,
}

impl PgServiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct ServiceRow {
    id: Uuid,
    tenant_id: Uuid,
    location_id: Uuid,
    name: String,
    description: Option<String>,
    vehicle_type: String,
    base_price: i64,
    duration_minutes: i32,
    is_active: bool,
    sort_order: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ServiceRow {
    fn into_service(self) -> Service {
        Service {
            id: self.id,
            tenant_id: self.tenant_id,
            location_id: self.location_id,
            name: self.name,
            description: self.description,
            vehicle_type: VehicleType::from_str(&self.vehicle_type)
                .unwrap_or(VehicleType::Sedan),
            base_price: Money::new(self.base_price),
            duration_minutes: self.duration_minutes,
            is_active: self.is_active,
            sort_order: self.sort_order,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

impl ServiceRepository for PgServiceRepository {
    async fn find_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<Service>> {
        let rows = sqlx::query_as::<_, ServiceRow>(
            r#"SELECT id, tenant_id, location_id, name, description,
                      vehicle_type, base_price, duration_minutes,
                      is_active, sort_order, created_at, updated_at, deleted_at
               FROM services
               WHERE tenant_id = $1
                 AND location_id = $2
                 AND deleted_at IS NULL
                 AND is_active = true
               ORDER BY sort_order ASC"#,
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(ServiceRow::into_service).collect())
    }

    async fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> anyhow::Result<Option<Service>> {
        let row = sqlx::query_as::<_, ServiceRow>(
            r#"SELECT id, tenant_id, location_id, name, description,
                      vehicle_type, base_price, duration_minutes,
                      is_active, sort_order, created_at, updated_at, deleted_at
               FROM services
               WHERE id = $1
                 AND tenant_id = $2
                 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(ServiceRow::into_service))
    }

    async fn create(&self, service: &Service) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO services
               (id, tenant_id, location_id, name, description, vehicle_type,
                base_price, duration_minutes, is_active, sort_order,
                created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
        )
        .bind(service.id)
        .bind(service.tenant_id)
        .bind(service.location_id)
        .bind(&service.name)
        .bind(&service.description)
        .bind(service.vehicle_type.as_str())
        .bind(service.base_price.amount())
        .bind(service.duration_minutes)
        .bind(service.is_active)
        .bind(service.sort_order)
        .bind(service.created_at)
        .bind(service.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, service: &Service) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE services
               SET name = $1, description = $2, vehicle_type = $3,
                   base_price = $4, duration_minutes = $5, is_active = $6,
                   sort_order = $7, updated_at = $8
               WHERE id = $9
                 AND tenant_id = $10
                 AND deleted_at IS NULL"#,
        )
        .bind(&service.name)
        .bind(&service.description)
        .bind(service.vehicle_type.as_str())
        .bind(service.base_price.amount())
        .bind(service.duration_minutes)
        .bind(service.is_active)
        .bind(service.sort_order)
        .bind(service.updated_at)
        .bind(service.id)
        .bind(service.tenant_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn soft_delete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE services
               SET deleted_at = NOW(), updated_at = NOW()
               WHERE id = $1
                 AND tenant_id = $2
                 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
