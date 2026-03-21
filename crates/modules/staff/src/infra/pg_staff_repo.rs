use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::{
    CommissionSummary, CreateCommissionRuleInput, CreateShiftInput, CreateStaffInput,
    RecordCommissionInput, StaffRepository, UpdateStaffInput,
};
use crate::domain::{
    CommissionEntry, CommissionRule, CommissionType, JobRole, Shift, ShiftStatus, SkillLevel,
    StaffProfile,
};

pub struct PgStaffRepository {
    pool: PgPool,
}

impl PgStaffRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_profile(row: &sqlx::postgres::PgRow) -> StaffProfile {
    StaffProfile {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        user_id: row.get("user_id"),
        location_id: row.get("location_id"),
        display_name: row.get("display_name"),
        skill_level: SkillLevel::from_str(row.get::<String, _>("skill_level").as_str())
            .unwrap_or(SkillLevel::Junior),
        hourly_rate: row.get("hourly_rate"),
        is_active: row.get("is_active"),
        joined_date: row.get("joined_date"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_shift(row: &sqlx::postgres::PgRow) -> Shift {
    Shift {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        staff_id: row.get("staff_id"),
        shift_date: row.get("shift_date"),
        start_time: row.get("start_time"),
        end_time: row.get("end_time"),
        actual_start: row.get("actual_start"),
        actual_end: row.get("actual_end"),
        status: ShiftStatus::from_str(row.get::<String, _>("status").as_str())
            .unwrap_or(ShiftStatus::Scheduled),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
    }
}

fn row_to_commission_rule(row: &sqlx::postgres::PgRow) -> CommissionRule {
    let skill_level_str: Option<String> = row.get("skill_level");
    CommissionRule {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        name: row.get("name"),
        service_id: row.get("service_id"),
        skill_level: skill_level_str.and_then(|s| SkillLevel::from_str(&s)),
        role_in_job: JobRole::from_str(row.get::<String, _>("role_in_job").as_str())
            .unwrap_or(JobRole::Primary),
        commission_type: CommissionType::from_str(row.get::<String, _>("commission_type").as_str())
            .unwrap_or(CommissionType::Fixed),
        commission_value: row.get("commission_value"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_commission_entry(row: &sqlx::postgres::PgRow) -> CommissionEntry {
    CommissionEntry {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        payment_id: row.get("payment_id"),
        staff_id: row.get("staff_id"),
        rule_id: row.get("rule_id"),
        role_in_job: JobRole::from_str(row.get::<String, _>("role_in_job").as_str())
            .unwrap_or(JobRole::Primary),
        payment_amount: row.get("payment_amount"),
        commission_amount: row.get("commission_amount"),
        created_at: row.get("created_at"),
    }
}

const PROFILE_COLS: &str = "id, tenant_id, user_id, location_id, display_name, skill_level, hourly_rate, is_active, joined_date, notes, created_at, updated_at";
const SHIFT_COLS: &str = "id, tenant_id, location_id, staff_id, shift_date, start_time, end_time, actual_start, actual_end, status, notes, created_at";
const RULE_COLS: &str = "id, tenant_id, location_id, name, service_id, skill_level, role_in_job, commission_type, commission_value, is_active, created_at, updated_at";
const ENTRY_COLS: &str = "id, tenant_id, payment_id, staff_id, rule_id, role_in_job, payment_amount, commission_amount, created_at";

impl StaffRepository for PgStaffRepository {
    // --- Staff profiles ---

    async fn create_profile(
        &self,
        tenant_id: Uuid,
        input: &CreateStaffInput,
    ) -> anyhow::Result<StaffProfile> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let joined_date = now.date_naive();

        sqlx::query(
            r#"INSERT INTO staff_profiles
               (id, tenant_id, user_id, location_id, display_name, skill_level,
                hourly_rate, is_active, joined_date, notes, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, true, $8, NULL, $9, $9)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.user_id)
        .bind(input.location_id)
        .bind(&input.display_name)
        .bind(&input.skill_level)
        .bind(input.hourly_rate)
        .bind(joined_date)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let q =
            format!("SELECT {PROFILE_COLS} FROM staff_profiles WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_profile(&row))
    }

    async fn get_profile(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<StaffProfile>> {
        let q =
            format!("SELECT {PROFILE_COLS} FROM staff_profiles WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.as_ref().map(row_to_profile))
    }

    async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<StaffProfile>> {
        let q = format!(
            "SELECT {PROFILE_COLS} FROM staff_profiles
             WHERE tenant_id = $1 AND location_id = $2
             ORDER BY display_name ASC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.iter().map(row_to_profile).collect())
    }

    async fn update_profile(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: &UpdateStaffInput,
    ) -> anyhow::Result<StaffProfile> {
        // Fetch current profile
        let q =
            format!("SELECT {PROFILE_COLS} FROM staff_profiles WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;

        let current = row
            .as_ref()
            .map(row_to_profile)
            .ok_or_else(|| anyhow::anyhow!("Staff profile not found"))?;

        let display_name = input
            .display_name
            .as_deref()
            .unwrap_or(&current.display_name);
        let skill_level = input
            .skill_level
            .as_deref()
            .unwrap_or(current.skill_level.as_str());
        let hourly_rate = input.hourly_rate.unwrap_or(current.hourly_rate);
        let is_active = input.is_active.unwrap_or(current.is_active);
        let now = Utc::now();

        sqlx::query(
            r#"UPDATE staff_profiles
               SET display_name = $1, skill_level = $2, hourly_rate = $3,
                   is_active = $4, updated_at = $5
               WHERE id = $6 AND tenant_id = $7"#,
        )
        .bind(display_name)
        .bind(skill_level)
        .bind(hourly_rate)
        .bind(is_active)
        .bind(now)
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        let row = sqlx::query(&format!(
            "SELECT {PROFILE_COLS} FROM staff_profiles WHERE id = $1 AND tenant_id = $2"
        ))
        .bind(id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row_to_profile(&row))
    }

    // --- Shifts ---

    async fn create_shift(
        &self,
        tenant_id: Uuid,
        input: &CreateShiftInput,
    ) -> anyhow::Result<Shift> {
        let id = Uuid::now_v7();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO shifts
               (id, tenant_id, location_id, staff_id, shift_date, start_time, end_time,
                actual_start, actual_end, status, notes, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, NULL, 'scheduled', NULL, $8)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.location_id)
        .bind(input.staff_id)
        .bind(input.shift_date)
        .bind(input.start_time)
        .bind(input.end_time)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let q = format!("SELECT {SHIFT_COLS} FROM shifts WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_shift(&row))
    }

    async fn list_shifts(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<Shift>> {
        let q = format!(
            "SELECT {SHIFT_COLS} FROM shifts
             WHERE tenant_id = $1 AND location_id = $2 AND shift_date = $3
             ORDER BY start_time ASC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .bind(date)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.iter().map(row_to_shift).collect())
    }

    async fn update_shift_status(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        status: &str,
        actual_start: Option<DateTime<Utc>>,
        actual_end: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Shift> {
        sqlx::query(
            r#"UPDATE shifts
               SET status = $1, actual_start = COALESCE($2, actual_start),
                   actual_end = COALESCE($3, actual_end)
               WHERE id = $4 AND tenant_id = $5"#,
        )
        .bind(status)
        .bind(actual_start)
        .bind(actual_end)
        .bind(id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        let q = format!("SELECT {SHIFT_COLS} FROM shifts WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_shift(&row))
    }

    // --- Commission rules ---

    async fn create_commission_rule(
        &self,
        tenant_id: Uuid,
        input: &CreateCommissionRuleInput,
    ) -> anyhow::Result<CommissionRule> {
        let id = Uuid::now_v7();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO commission_rules
               (id, tenant_id, location_id, name, service_id, skill_level, role_in_job,
                commission_type, commission_value, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, true, $10, $10)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.location_id)
        .bind(&input.name)
        .bind(input.service_id)
        .bind(input.skill_level.as_deref())
        .bind(&input.role_in_job)
        .bind(&input.commission_type)
        .bind(input.commission_value)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let q =
            format!("SELECT {RULE_COLS} FROM commission_rules WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_commission_rule(&row))
    }

    async fn list_commission_rules(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<CommissionRule>> {
        let q = format!(
            "SELECT {RULE_COLS} FROM commission_rules
             WHERE tenant_id = $1 AND location_id = $2 AND is_active = true
             ORDER BY name ASC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.iter().map(row_to_commission_rule).collect())
    }

    // --- Commission entries ---

    async fn record_commission(
        &self,
        tenant_id: Uuid,
        input: &RecordCommissionInput,
    ) -> anyhow::Result<CommissionEntry> {
        let id = Uuid::now_v7();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO commission_entries
               (id, tenant_id, payment_id, staff_id, rule_id, role_in_job,
                payment_amount, commission_amount, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(input.payment_id)
        .bind(input.staff_id)
        .bind(input.rule_id)
        .bind(&input.role_in_job)
        .bind(input.payment_amount)
        .bind(input.commission_amount)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let q =
            format!("SELECT {ENTRY_COLS} FROM commission_entries WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_commission_entry(&row))
    }

    async fn staff_commission_summary(
        &self,
        tenant_id: Uuid,
        staff_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> anyhow::Result<CommissionSummary> {
        let row = sqlx::query(
            r#"SELECT
                 COUNT(*)::BIGINT AS total_jobs,
                 COALESCE(SUM(payment_amount), 0)::BIGINT AS total_revenue,
                 COALESCE(SUM(commission_amount), 0)::BIGINT AS total_commission
               FROM commission_entries
               WHERE tenant_id = $1
                 AND staff_id = $2
                 AND created_at >= $3::date
                 AND created_at < ($4::date + INTERVAL '1 day')"#,
        )
        .bind(tenant_id)
        .bind(staff_id)
        .bind(from)
        .bind(to)
        .fetch_one(&self.pool)
        .await?;

        Ok(CommissionSummary {
            staff_id,
            total_jobs: row.get("total_jobs"),
            total_revenue: row.get("total_revenue"),
            total_commission: row.get("total_commission"),
            period_from: from,
            period_to: to,
        })
    }
}
