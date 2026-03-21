use chrono::NaiveDate;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::application::{
    CreateMaterialInput, InventoryRepository, RecordTransactionInput, SetNormInput,
    UpdateMaterialInput,
};
use crate::domain::{
    InventoryTransaction, LowStockAlert, Material, MaterialCategory, MaterialNorm,
    MaterialVariance, TransactionType,
};

pub struct PgInventoryRepository {
    pool: PgPool,
}

impl PgInventoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct MaterialRow {
    id: Uuid,
    tenant_id: Uuid,
    location_id: Uuid,
    name: String,
    category: String,
    unit: String,
    unit_cost: i64,
    current_stock: i64,
    min_stock: i64,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl MaterialRow {
    fn into_material(self) -> Material {
        Material {
            id: self.id,
            tenant_id: self.tenant_id,
            location_id: self.location_id,
            name: self.name,
            category: MaterialCategory::from_str(&self.category)
                .unwrap_or(MaterialCategory::Consumable),
            unit: self.unit,
            unit_cost: self.unit_cost,
            current_stock: self.current_stock,
            min_stock: self.min_stock,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(FromRow)]
struct NormRow {
    id: Uuid,
    tenant_id: Uuid,
    service_id: Uuid,
    material_id: Uuid,
    quantity_per_job: i64,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl NormRow {
    fn into_norm(self) -> MaterialNorm {
        MaterialNorm {
            id: self.id,
            tenant_id: self.tenant_id,
            service_id: self.service_id,
            material_id: self.material_id,
            quantity_per_job: self.quantity_per_job,
            created_at: self.created_at,
        }
    }
}

#[derive(FromRow)]
struct TransactionRow {
    id: Uuid,
    tenant_id: Uuid,
    material_id: Uuid,
    transaction_type: String,
    quantity: i64,
    unit_cost: Option<i64>,
    reference_id: Option<Uuid>,
    reference_type: Option<String>,
    notes: Option<String>,
    performed_by: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl TransactionRow {
    fn into_transaction(self) -> InventoryTransaction {
        InventoryTransaction {
            id: self.id,
            tenant_id: self.tenant_id,
            material_id: self.material_id,
            transaction_type: TransactionType::from_str(&self.transaction_type)
                .unwrap_or(TransactionType::Adjustment),
            quantity: self.quantity,
            unit_cost: self.unit_cost,
            reference_id: self.reference_id,
            reference_type: self.reference_type,
            notes: self.notes,
            performed_by: self.performed_by,
            created_at: self.created_at,
        }
    }
}

impl InventoryRepository for PgInventoryRepository {
    async fn create_material(
        &self,
        tenant_id: Uuid,
        input: &CreateMaterialInput,
    ) -> anyhow::Result<Material> {
        let id = Uuid::now_v7();
        let now = chrono::Utc::now();

        let row = sqlx::query_as::<_, MaterialRow>(
            r#"INSERT INTO materials
               (id, tenant_id, location_id, name, category, unit, unit_cost,
                current_stock, min_stock, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, true, $10, $10)
               RETURNING id, tenant_id, location_id, name, category, unit, unit_cost,
                         current_stock, min_stock, is_active, created_at, updated_at"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.location_id)
        .bind(&input.name)
        .bind(&input.category)
        .bind(&input.unit)
        .bind(input.unit_cost)
        .bind(input.current_stock)
        .bind(input.min_stock)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into_material())
    }

    async fn get_material(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<Material>> {
        let row = sqlx::query_as::<_, MaterialRow>(
            r#"SELECT id, tenant_id, location_id, name, category, unit, unit_cost,
                      current_stock, min_stock, is_active, created_at, updated_at
               FROM materials
               WHERE id = $1
                 AND tenant_id = $2"#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(MaterialRow::into_material))
    }

    async fn list_materials(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<Material>> {
        let rows = sqlx::query_as::<_, MaterialRow>(
            r#"SELECT id, tenant_id, location_id, name, category, unit, unit_cost,
                      current_stock, min_stock, is_active, created_at, updated_at
               FROM materials
               WHERE tenant_id = $1
                 AND location_id = $2
                 AND is_active = true
               ORDER BY name ASC"#,
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(MaterialRow::into_material).collect())
    }

    async fn update_material(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: &UpdateMaterialInput,
    ) -> anyhow::Result<Material> {
        let row = sqlx::query_as::<_, MaterialRow>(
            r#"UPDATE materials
               SET name = COALESCE($1, name),
                   category = COALESCE($2, category),
                   unit = COALESCE($3, unit),
                   unit_cost = COALESCE($4, unit_cost),
                   min_stock = COALESCE($5, min_stock),
                   is_active = COALESCE($6, is_active),
                   updated_at = NOW()
               WHERE id = $7
                 AND tenant_id = $8
               RETURNING id, tenant_id, location_id, name, category, unit, unit_cost,
                         current_stock, min_stock, is_active, created_at, updated_at"#,
        )
        .bind(input.name.as_deref())
        .bind(input.category.as_deref())
        .bind(input.unit.as_deref())
        .bind(input.unit_cost)
        .bind(input.min_stock)
        .bind(input.is_active)
        .bind(id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into_material())
    }

    async fn set_norm(
        &self,
        tenant_id: Uuid,
        input: &SetNormInput,
    ) -> anyhow::Result<MaterialNorm> {
        let id = Uuid::now_v7();
        let now = chrono::Utc::now();

        let row = sqlx::query_as::<_, NormRow>(
            r#"INSERT INTO material_norms
               (id, tenant_id, service_id, material_id, quantity_per_job, created_at)
               VALUES ($1, $2, $3, $4, $5, $6)
               ON CONFLICT (tenant_id, service_id, material_id)
               DO UPDATE SET quantity_per_job = EXCLUDED.quantity_per_job
               RETURNING id, tenant_id, service_id, material_id, quantity_per_job, created_at"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.service_id)
        .bind(input.material_id)
        .bind(input.quantity_per_job)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into_norm())
    }

    async fn list_norms(
        &self,
        tenant_id: Uuid,
        service_id: Uuid,
    ) -> anyhow::Result<Vec<MaterialNorm>> {
        let rows = sqlx::query_as::<_, NormRow>(
            r#"SELECT id, tenant_id, service_id, material_id, quantity_per_job, created_at
               FROM material_norms
               WHERE tenant_id = $1
                 AND service_id = $2
               ORDER BY created_at ASC"#,
        )
        .bind(tenant_id)
        .bind(service_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(NormRow::into_norm).collect())
    }

    async fn delete_norm(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            r#"DELETE FROM material_norms
               WHERE id = $1
                 AND tenant_id = $2"#,
        )
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn record_transaction(
        &self,
        tenant_id: Uuid,
        input: &RecordTransactionInput,
    ) -> anyhow::Result<InventoryTransaction> {
        let mut tx = self.pool.begin().await?;

        let id = Uuid::now_v7();
        let now = chrono::Utc::now();

        let row = sqlx::query_as::<_, TransactionRow>(
            r#"INSERT INTO inventory_transactions
               (id, tenant_id, material_id, transaction_type, quantity, unit_cost,
                reference_id, reference_type, notes, performed_by, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING id, tenant_id, material_id, transaction_type, quantity, unit_cost,
                         reference_id, reference_type, notes, performed_by, created_at"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.material_id)
        .bind(&input.transaction_type)
        .bind(input.quantity)
        .bind(input.unit_cost)
        .bind(input.reference_id)
        .bind(input.reference_type.as_deref())
        .bind(input.notes.as_deref())
        .bind(input.performed_by)
        .bind(now)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"UPDATE materials
               SET current_stock = current_stock + $1,
                   updated_at = NOW()
               WHERE id = $2
                 AND tenant_id = $3"#,
        )
        .bind(input.quantity)
        .bind(input.material_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.into_transaction())
    }

    async fn list_transactions(
        &self,
        tenant_id: Uuid,
        material_id: Uuid,
        limit: i64,
    ) -> anyhow::Result<Vec<InventoryTransaction>> {
        let rows = sqlx::query_as::<_, TransactionRow>(
            r#"SELECT id, tenant_id, material_id, transaction_type, quantity, unit_cost,
                      reference_id, reference_type, notes, performed_by, created_at
               FROM inventory_transactions
               WHERE tenant_id = $1
                 AND material_id = $2
               ORDER BY created_at DESC
               LIMIT $3"#,
        )
        .bind(tenant_id)
        .bind(material_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(TransactionRow::into_transaction)
            .collect())
    }

    async fn low_stock_alerts(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<LowStockAlert>> {
        let rows = sqlx::query(
            r#"SELECT id, name, current_stock, min_stock, unit
               FROM materials
               WHERE tenant_id = $1
                 AND location_id = $2
                 AND is_active = true
                 AND current_stock <= min_stock
               ORDER BY (current_stock - min_stock) ASC"#,
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_all(&self.pool)
        .await?;

        let alerts = rows
            .into_iter()
            .map(|row| LowStockAlert {
                material_id: row.get("id"),
                name: row.get("name"),
                current_stock: row.get("current_stock"),
                min_stock: row.get("min_stock"),
                unit: row.get("unit"),
            })
            .collect();

        Ok(alerts)
    }

    async fn material_variance(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> anyhow::Result<Vec<MaterialVariance>> {
        let rows = sqlx::query(
            r#"SELECT
                 m.id AS material_id,
                 m.name AS material_name,
                 m.unit,
                 COALESCE(j.job_count, 0) AS job_count,
                 COALESCE(j.job_count, 0) * COALESCE(mn.quantity_per_job, 0) AS expected_usage,
                 COALESCE(ABS(t.actual_usage), 0) AS actual_usage,
                 COALESCE(ABS(t.actual_usage), 0) - (COALESCE(j.job_count, 0) * COALESCE(mn.quantity_per_job, 0)) AS variance
               FROM materials m
               LEFT JOIN material_norms mn ON mn.material_id = m.id AND mn.tenant_id = m.tenant_id
               LEFT JOIN LATERAL (
                 SELECT COUNT(*) AS job_count
                 FROM queue_entries qe
                 JOIN services s ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
                 WHERE qe.tenant_id = m.tenant_id
                   AND s.location_id = m.location_id
                   AND mn.service_id = s.id
                   AND qe.status = 'completed'
                   AND qe.created_at::date >= $3
                   AND qe.created_at::date <= $4
               ) j ON true
               LEFT JOIN LATERAL (
                 SELECT SUM(it.quantity) AS actual_usage
                 FROM inventory_transactions it
                 WHERE it.tenant_id = m.tenant_id
                   AND it.material_id = m.id
                   AND it.transaction_type = 'usage'
                   AND it.created_at::date >= $3
                   AND it.created_at::date <= $4
               ) t ON true
               WHERE m.tenant_id = $1
                 AND m.location_id = $2
                 AND m.is_active = true
               ORDER BY m.name ASC"#,
        )
        .bind(tenant_id)
        .bind(location_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        let variances = rows
            .into_iter()
            .map(|row| MaterialVariance {
                material_id: row.get("material_id"),
                material_name: row.get("material_name"),
                unit: row.get("unit"),
                job_count: row.get("job_count"),
                expected_usage: row.get("expected_usage"),
                actual_usage: row.get("actual_usage"),
                variance: row.get("variance"),
            })
            .collect();

        Ok(variances)
    }
}
