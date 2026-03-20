use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::LocationRepository;
use crate::domain::{Bay, Location, LocationStatus, OperatingHours, QueueMode};

pub struct PgLocationRepository {
    pool: PgPool,
}

impl PgLocationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_location(row: &sqlx::postgres::PgRow) -> Location {
    Location {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        name: row.get("name"),
        slug: row.get("slug"),
        phone: row.get("phone"),
        address: row.get("address"),
        district: row.get("district"),
        city: row.get("city"),
        latitude: row.get("latitude"),
        longitude: row.get("longitude"),
        bay_count: row.get("bay_count"),
        queue_mode: QueueMode::from_str(row.get::<String, _>("queue_mode").as_str()).unwrap_or(QueueMode::Hybrid),
        status: LocationStatus::from_str(row.get::<String, _>("status").as_str()).unwrap_or(LocationStatus::Pending),
        amenities: row.get("amenities"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str = r#"id, tenant_id, name, slug, phone, address, district, city,
    ST_Y(coordinates::geometry) as latitude,
    ST_X(coordinates::geometry) as longitude,
    bay_count, queue_mode, status, amenities, created_at, updated_at, deleted_at"#;

impl LocationRepository for PgLocationRepository {
    async fn find_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Location>, sqlx::Error> {
        let q = format!(
            "SELECT {SELECT_COLS} FROM locations WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL"
        );
        sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await
            .map(|r| r.map(|row| row_to_location(&row)))
    }

    async fn find_by_slug(&self, tenant_id: Uuid, slug: &str) -> Result<Option<Location>, sqlx::Error> {
        let q = format!(
            "SELECT {SELECT_COLS} FROM locations WHERE slug = $1 AND tenant_id = $2 AND deleted_at IS NULL"
        );
        sqlx::query(&q)
            .bind(slug)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await
            .map(|r| r.map(|row| row_to_location(&row)))
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Location>, sqlx::Error> {
        let q = format!(
            "SELECT {SELECT_COLS} FROM locations WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY name"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_location).collect())
    }

    async fn create(&self, location: &Location) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"INSERT INTO locations
               (id, tenant_id, name, slug, phone, address, district, city,
                coordinates, bay_count, queue_mode, status, amenities, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
                       ST_MakePoint($9, $10)::geography, $11, $12, $13, $14, $15, $16)"#,
        )
        .bind(location.id)
        .bind(location.tenant_id)
        .bind(&location.name)
        .bind(&location.slug)
        .bind(&location.phone)
        .bind(&location.address)
        .bind(&location.district)
        .bind(&location.city)
        .bind(location.longitude)
        .bind(location.latitude)
        .bind(location.bay_count)
        .bind(location.queue_mode.as_str())
        .bind(location.status.as_str())
        .bind(&location.amenities)
        .bind(location.created_at)
        .bind(location.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, location: &Location) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE locations
               SET name = $1, slug = $2, phone = $3, address = $4, district = $5, city = $6,
                   coordinates = ST_MakePoint($7, $8)::geography,
                   bay_count = $9, queue_mode = $10, status = $11, amenities = $12, updated_at = $13
               WHERE id = $14 AND tenant_id = $15 AND deleted_at IS NULL"#,
        )
        .bind(&location.name)
        .bind(&location.slug)
        .bind(&location.phone)
        .bind(&location.address)
        .bind(&location.district)
        .bind(&location.city)
        .bind(location.longitude)
        .bind(location.latitude)
        .bind(location.bay_count)
        .bind(location.queue_mode.as_str())
        .bind(location.status.as_str())
        .bind(&location.amenities)
        .bind(location.updated_at)
        .bind(location.id)
        .bind(location.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn soft_delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE locations SET deleted_at = $1 WHERE id = $2 AND tenant_id = $3 AND deleted_at IS NULL",
        )
        .bind(Utc::now())
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_nearby(&self, lat: f64, lng: f64, radius_meters: f64) -> Result<Vec<(Location, f64)>, sqlx::Error> {
        let q = format!(
            "SELECT {SELECT_COLS}, ST_Distance(coordinates, ST_MakePoint($1, $2)::geography) as distance
             FROM locations
             WHERE ST_DWithin(coordinates, ST_MakePoint($1, $2)::geography, $3)
                   AND deleted_at IS NULL AND status = 'active'
             ORDER BY distance"
        );
        let rows = sqlx::query(&q)
            .bind(lng)
            .bind(lat)
            .bind(radius_meters)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .iter()
            .map(|row| {
                let distance: f64 = row.get("distance");
                (row_to_location(row), distance)
            })
            .collect())
    }

    async fn get_operating_hours(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<OperatingHours>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT oh.id, oh.location_id, l.tenant_id, oh.day_of_week, oh.open_time, oh.close_time, oh.is_closed
               FROM operating_hours oh
               JOIN locations l ON l.id = oh.location_id
               WHERE oh.location_id = $1 AND l.tenant_id = $2 AND l.deleted_at IS NULL
               ORDER BY oh.day_of_week"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| OperatingHours {
                id: row.get("id"),
                location_id: row.get("location_id"),
                tenant_id: row.get("tenant_id"),
                day_of_week: row.get("day_of_week"),
                open_time: row.get("open_time"),
                close_time: row.get("close_time"),
                is_closed: row.get("is_closed"),
            })
            .collect())
    }

    async fn set_operating_hours(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        hours: &[OperatingHours],
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Delete existing hours for this location (tenant-safe via join check)
        sqlx::query(
            r#"DELETE FROM operating_hours
               WHERE location_id = $1
                 AND location_id IN (SELECT id FROM locations WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL)"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        // Insert new hours
        for h in hours {
            sqlx::query(
                r#"INSERT INTO operating_hours (id, location_id, day_of_week, open_time, close_time, is_closed)
                   VALUES ($1, $2, $3, $4, $5, $6)"#,
            )
            .bind(h.id)
            .bind(location_id)
            .bind(h.day_of_week)
            .bind(h.open_time)
            .bind(h.close_time)
            .bind(h.is_closed)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn list_bays(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<Bay>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, location_id, tenant_id, name, is_active FROM bays WHERE tenant_id = $1 AND location_id = $2 ORDER BY name",
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| Bay {
                id: row.get("id"),
                location_id: row.get("location_id"),
                tenant_id: row.get("tenant_id"),
                name: row.get("name"),
                is_active: row.get("is_active"),
            })
            .collect())
    }

    async fn create_bay(&self, bay: &Bay) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO bays (id, location_id, tenant_id, name, is_active) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(bay.id)
        .bind(bay.location_id)
        .bind(bay.tenant_id)
        .bind(&bay.name)
        .bind(bay.is_active)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_bay(&self, bay: &Bay) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE bays SET name = $1, is_active = $2 WHERE id = $3 AND tenant_id = $4",
        )
        .bind(&bay.name)
        .bind(bay.is_active)
        .bind(bay.id)
        .bind(bay.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_bay(&self, tenant_id: Uuid, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM bays WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
