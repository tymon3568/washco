use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::PricingRepository;
use crate::domain::{PricingRule, RuleType};

pub struct PgPricingRepository {
    pool: PgPool,
}

impl PgPricingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_rule(row: &sqlx::postgres::PgRow) -> PricingRule {
    let rule_type_str: String = row.get("rule_type");
    PricingRule {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        service_id: row.get("service_id"),
        name: row.get("name"),
        rule_type: RuleType::from_str(&rule_type_str).unwrap_or(RuleType::Surge),
        multiplier: row.get("multiplier"),
        fixed_adjustment: row.get("fixed_adjustment"),
        conditions: row.get("conditions"),
        priority: row.get("priority"),
        is_active: row.get("is_active"),
        valid_from: row.get("valid_from"),
        valid_to: row.get("valid_to"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

const RULE_COLS: &str = "id, tenant_id, location_id, service_id, name, rule_type, multiplier, fixed_adjustment, conditions, priority, is_active, valid_from, valid_to, created_at, updated_at";

impl PricingRepository for PgPricingRepository {
    async fn create(&self, rule: &PricingRule) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO pricing_rules
               (id, tenant_id, location_id, service_id, name, rule_type, multiplier,
                fixed_adjustment, conditions, priority, is_active, valid_from, valid_to,
                created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#,
        )
        .bind(rule.id)
        .bind(rule.tenant_id)
        .bind(rule.location_id)
        .bind(rule.service_id)
        .bind(&rule.name)
        .bind(rule.rule_type.as_str())
        .bind(rule.multiplier)
        .bind(rule.fixed_adjustment)
        .bind(&rule.conditions)
        .bind(rule.priority)
        .bind(rule.is_active)
        .bind(rule.valid_from)
        .bind(rule.valid_to)
        .bind(rule.created_at)
        .bind(rule.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<PricingRule>> {
        let q = format!(
            "SELECT {RULE_COLS} FROM pricing_rules WHERE id = $1 AND tenant_id = $2"
        );
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_rule))
    }

    async fn update(&self, rule: &PricingRule) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE pricing_rules
               SET name = $1, rule_type = $2, multiplier = $3, fixed_adjustment = $4,
                   conditions = $5, priority = $6, is_active = $7, service_id = $8,
                   valid_from = $9, valid_to = $10, updated_at = $11
               WHERE id = $12 AND tenant_id = $13"#,
        )
        .bind(&rule.name)
        .bind(rule.rule_type.as_str())
        .bind(rule.multiplier)
        .bind(rule.fixed_adjustment)
        .bind(&rule.conditions)
        .bind(rule.priority)
        .bind(rule.is_active)
        .bind(rule.service_id)
        .bind(rule.valid_from)
        .bind(rule.valid_to)
        .bind(rule.updated_at)
        .bind(rule.id)
        .bind(rule.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM pricing_rules WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<PricingRule>> {
        let q = format!(
            "SELECT {RULE_COLS} FROM pricing_rules
             WHERE tenant_id = $1 AND location_id = $2
             ORDER BY priority DESC, created_at ASC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_rule).collect())
    }

    async fn find_active_rules(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        service_id: Option<Uuid>,
        now: DateTime<Utc>,
    ) -> anyhow::Result<Vec<PricingRule>> {
        let q = format!(
            "SELECT {RULE_COLS} FROM pricing_rules
             WHERE tenant_id = $1
               AND location_id = $2
               AND is_active = true
               AND (service_id IS NULL OR service_id = $3)
               AND (valid_from IS NULL OR valid_from <= $4)
               AND (valid_to IS NULL OR valid_to >= $4)
             ORDER BY priority DESC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .bind(service_id)
            .bind(now)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_rule).collect())
    }
}
