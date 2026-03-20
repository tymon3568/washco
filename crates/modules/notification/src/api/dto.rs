use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Notification, NotificationTemplate};

// ── Template DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub template_type: String,
    pub channel: Option<String>,
    pub subject: Option<String>,
    pub body_template: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub template_type: String,
    pub channel: Option<String>,
    pub subject: Option<String>,
    pub body_template: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: Uuid,
    pub template_type: String,
    pub channel: String,
    pub subject: Option<String>,
    pub body_template: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<NotificationTemplate> for TemplateResponse {
    fn from(t: NotificationTemplate) -> Self {
        Self {
            id: t.id,
            template_type: t.template_type,
            channel: t.channel.to_string(),
            subject: t.subject,
            body_template: t.body_template,
            is_active: t.is_active,
            created_at: t.created_at,
        }
    }
}

// ── Notification DTOs ──

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub recipient_phone: String,
    pub template_type: String,
    pub channel: Option<String>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ListNotificationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub recipient_phone: String,
    pub channel: String,
    pub template_type: String,
    pub payload: serde_json::Value,
    pub rendered_body: Option<String>,
    pub status: String,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id,
            recipient_phone: n.recipient_phone,
            channel: n.channel.to_string(),
            template_type: n.template_type,
            payload: n.payload,
            rendered_body: n.rendered_body,
            status: n.status.to_string(),
            sent_at: n.sent_at,
            error: n.error,
            created_at: n.created_at,
        }
    }
}
