use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::NotificationRepository;
use crate::domain::{
    Notification, NotificationChannel, NotificationStatus, NotificationTemplate,
};

pub struct PgNotificationRepository {
    pool: PgPool,
}

impl PgNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_template(row: &sqlx::postgres::PgRow) -> NotificationTemplate {
    NotificationTemplate {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        template_type: row.get("template_type"),
        channel: NotificationChannel::from_str(row.get::<String, _>("channel").as_str()),
        subject: row.get("subject"),
        body_template: row.get("body_template"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
    }
}

fn row_to_notification(row: &sqlx::postgres::PgRow) -> Notification {
    Notification {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        recipient_phone: row.get("recipient_phone"),
        channel: NotificationChannel::from_str(row.get::<String, _>("channel").as_str()),
        template_type: row.get("template_type"),
        payload: row.get("payload"),
        rendered_body: row.get("rendered_body"),
        status: NotificationStatus::from_str(row.get::<String, _>("status").as_str()),
        sent_at: row.get("sent_at"),
        error: row.get("error"),
        created_at: row.get("created_at"),
    }
}

const TEMPLATE_COLS: &str =
    "id, tenant_id, template_type, channel, subject, body_template, is_active, created_at";

const NOTIFICATION_COLS: &str =
    "id, tenant_id, recipient_phone, channel, template_type, payload, rendered_body, status, sent_at, error, created_at";

impl NotificationRepository for PgNotificationRepository {
    async fn create_template(&self, template: &NotificationTemplate) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO notification_templates
               (id, tenant_id, template_type, channel, subject, body_template, is_active, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        )
        .bind(template.id)
        .bind(template.tenant_id)
        .bind(&template.template_type)
        .bind(template.channel.as_str())
        .bind(&template.subject)
        .bind(&template.body_template)
        .bind(template.is_active)
        .bind(template.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_template(
        &self,
        tenant_id: Uuid,
        template_type: &str,
        channel: &str,
    ) -> anyhow::Result<Option<NotificationTemplate>> {
        let q = format!(
            "SELECT {TEMPLATE_COLS} FROM notification_templates
             WHERE tenant_id = $1 AND template_type = $2 AND channel = $3"
        );
        let row = sqlx::query(&q)
            .bind(tenant_id)
            .bind(template_type)
            .bind(channel)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_template))
    }

    async fn list_templates(
        &self,
        tenant_id: Uuid,
    ) -> anyhow::Result<Vec<NotificationTemplate>> {
        let q = format!(
            "SELECT {TEMPLATE_COLS} FROM notification_templates
             WHERE tenant_id = $1 ORDER BY created_at DESC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_template).collect())
    }

    async fn update_template(&self, template: &NotificationTemplate) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE notification_templates SET template_type = $1, channel = $2, subject = $3, body_template = $4, is_active = $5 WHERE id = $6 AND tenant_id = $7",
        )
        .bind(&template.template_type)
        .bind(template.channel.as_str())
        .bind(&template.subject)
        .bind(&template.body_template)
        .bind(template.is_active)
        .bind(template.id)
        .bind(template.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_template(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM notification_templates WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_notification(&self, notif: &Notification) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO notifications
               (id, tenant_id, recipient_phone, channel, template_type, payload, rendered_body, status, sent_at, error, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
        )
        .bind(notif.id)
        .bind(notif.tenant_id)
        .bind(&notif.recipient_phone)
        .bind(notif.channel.as_str())
        .bind(&notif.template_type)
        .bind(&notif.payload)
        .bind(&notif.rendered_body)
        .bind(notif.status.as_str())
        .bind(notif.sent_at)
        .bind(&notif.error)
        .bind(notif.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_notifications(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Notification>> {
        let q = format!(
            "SELECT {NOTIFICATION_COLS} FROM notifications
             WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_notification).collect())
    }

    async fn mark_sent(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE notifications SET status = 'sent', sent_at = now() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn mark_failed(&self, id: Uuid, error: &str) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE notifications SET status = 'failed', error = $1 WHERE id = $2",
        )
        .bind(error)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
