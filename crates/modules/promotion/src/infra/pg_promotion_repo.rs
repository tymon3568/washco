use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::PromotionRepository;
use crate::domain::{DiscountType, Promotion};

pub struct PgPromotionRepository {
    pool: PgPool,
}

impl PgPromotionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_promotion(row: &sqlx::postgres::PgRow) -> Promotion {
    Promotion {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        code: row.get("code"),
        name: row.get("name"),
        description: row.get("description"),
        discount_type: DiscountType::from_str(row.get::<String, _>("discount_type").as_str()),
        discount_value: row.get("discount_value"),
        min_order: row.get("min_order"),
        max_uses: row.get("max_uses"),
        used_count: row.get("used_count"),
        valid_from: row.get("valid_from"),
        valid_to: row.get("valid_to"),
        location_ids: row.get::<Vec<Uuid>, _>("location_ids"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

const PROMO_COLS: &str = "id, tenant_id, code, name, description, discount_type, discount_value, min_order, max_uses, used_count, valid_from, valid_to, location_ids, is_active, created_at, updated_at";

impl PromotionRepository for PgPromotionRepository {
    async fn create(&self, promo: &Promotion) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO promotions
               (id, tenant_id, code, name, description, discount_type, discount_value,
                min_order, max_uses, used_count, valid_from, valid_to, location_ids,
                is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"#,
        )
        .bind(promo.id)
        .bind(promo.tenant_id)
        .bind(&promo.code)
        .bind(&promo.name)
        .bind(&promo.description)
        .bind(promo.discount_type.as_str())
        .bind(promo.discount_value)
        .bind(promo.min_order)
        .bind(promo.max_uses)
        .bind(promo.used_count)
        .bind(promo.valid_from)
        .bind(promo.valid_to)
        .bind(&promo.location_ids)
        .bind(promo.is_active)
        .bind(promo.created_at)
        .bind(promo.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<Promotion>> {
        let q = format!("SELECT {PROMO_COLS} FROM promotions WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_promotion))
    }

    async fn find_by_code(&self, tenant_id: Uuid, code: &str) -> anyhow::Result<Option<Promotion>> {
        let q = format!("SELECT {PROMO_COLS} FROM promotions WHERE tenant_id = $1 AND code = $2");
        let row = sqlx::query(&q)
            .bind(tenant_id)
            .bind(code)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_promotion))
    }

    async fn update(&self, promo: &Promotion) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE promotions SET
               code = $1, name = $2, description = $3, discount_type = $4,
               discount_value = $5, min_order = $6, max_uses = $7,
               valid_from = $8, valid_to = $9, location_ids = $10,
               is_active = $11, updated_at = $12
               WHERE id = $13 AND tenant_id = $14"#,
        )
        .bind(&promo.code)
        .bind(&promo.name)
        .bind(&promo.description)
        .bind(promo.discount_type.as_str())
        .bind(promo.discount_value)
        .bind(promo.min_order)
        .bind(promo.max_uses)
        .bind(promo.valid_from)
        .bind(promo.valid_to)
        .bind(&promo.location_ids)
        .bind(promo.is_active)
        .bind(promo.updated_at)
        .bind(promo.id)
        .bind(promo.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM promotions WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list(&self, tenant_id: Uuid) -> anyhow::Result<Vec<Promotion>> {
        let q = format!(
            "SELECT {PROMO_COLS} FROM promotions WHERE tenant_id = $1 ORDER BY created_at DESC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_promotion).collect())
    }

    async fn increment_used_count(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE promotions SET used_count = used_count + 1, updated_at = now() WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
