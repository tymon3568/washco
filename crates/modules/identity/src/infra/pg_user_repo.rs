use sqlx::{PgPool, Row};
use uuid::Uuid;
use washco_shared::Role;

use crate::application::UserRepository;
use crate::domain::{Tenant, User};

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query(
            "SELECT id, tenant_id, phone, name, role, is_verified, created_at, updated_at, deleted_at
             FROM users WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map(|row| row.map(|r| row_to_user(&r)))
    }

    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query(
            "SELECT id, tenant_id, phone, name, role, is_verified, created_at, updated_at, deleted_at
             FROM users WHERE phone = $1 AND deleted_at IS NULL",
        )
        .bind(phone)
        .fetch_optional(&self.pool)
        .await
        .map(|row| row.map(|r| row_to_user(&r)))
    }

    async fn create_tenant(&self, tenant: &Tenant) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO tenants (id, business_name, owner_name, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(tenant.id)
        .bind(&tenant.business_name)
        .bind(&tenant.owner_name)
        .bind(tenant.created_at)
        .bind(tenant.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, tenant_id, phone, name, role, is_verified, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(user.id)
        .bind(user.tenant_id)
        .bind(&user.phone)
        .bind(&user.name)
        .bind(user.role.to_string())
        .bind(user.is_verified)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET name = $1, role = $2, is_verified = $3, updated_at = $4
             WHERE id = $5 AND deleted_at IS NULL",
        )
        .bind(&user.name)
        .bind(user.role.to_string())
        .bind(user.is_verified)
        .bind(user.updated_at)
        .bind(user.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_tenant_tier_features(
        &self,
        tenant_id: Uuid,
    ) -> Result<(Option<String>, Vec<String>), sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT st.name, st.features
               FROM tenants t
               JOIN subscription_tiers st ON st.id = t.tier_id
               WHERE t.id = $1"#,
        )
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let name: String = r.get("name");
                let features_json: serde_json::Value = r.get("features");
                let features: Vec<String> = features_json
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                Ok((Some(name), features))
            }
            None => Ok((None, vec![])),
        }
    }
}

fn row_to_user(row: &sqlx::postgres::PgRow) -> User {
    let role_str: String = row.get("role");
    User {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        phone: row.get("phone"),
        name: row.get("name"),
        role: parse_role(&role_str),
        is_verified: row.get("is_verified"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

fn parse_role(s: &str) -> Role {
    match s {
        "owner" => Role::Owner,
        "manager" => Role::Manager,
        "cashier" => Role::Cashier,
        "staff" => Role::Staff,
        "driver" => Role::Driver,
        "admin" => Role::Admin,
        _ => Role::Staff,
    }
}
