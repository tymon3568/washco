use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{
    Notification, NotificationChannel, NotificationError, NotificationStatus, NotificationTemplate,
};

use super::ports::NotificationRepository;

pub struct NotificationService<R> {
    repo: R,
}

/// Input for sending a notification.
pub struct SendNotificationInput {
    pub recipient_phone: String,
    pub template_type: String,
    pub channel: String,
    pub payload: serde_json::Value,
}

/// Input for creating a template.
pub struct CreateTemplateInput {
    pub template_type: String,
    pub channel: String,
    pub subject: Option<String>,
    pub body_template: String,
}

/// Input for updating a template.
pub struct UpdateTemplateInput {
    pub id: Uuid,
    pub template_type: String,
    pub channel: String,
    pub subject: Option<String>,
    pub body_template: String,
    pub is_active: bool,
}

impl<R: NotificationRepository> NotificationService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_template(
        &self,
        tenant_id: Uuid,
        input: CreateTemplateInput,
    ) -> Result<NotificationTemplate, AppError> {
        let template = NotificationTemplate {
            id: Uuid::now_v7(),
            tenant_id,
            template_type: input.template_type,
            channel: NotificationChannel::from_str(&input.channel),
            subject: input.subject,
            body_template: input.body_template,
            is_active: true,
            created_at: Utc::now(),
        };

        self.repo
            .create_template(&template)
            .await
            .map_err(AppError::Internal)?;

        Ok(template)
    }

    pub async fn list_templates(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<NotificationTemplate>, AppError> {
        self.repo
            .list_templates(tenant_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_template(
        &self,
        tenant_id: Uuid,
        input: UpdateTemplateInput,
    ) -> Result<NotificationTemplate, AppError> {
        let template = NotificationTemplate {
            id: input.id,
            tenant_id,
            template_type: input.template_type,
            channel: NotificationChannel::from_str(&input.channel),
            subject: input.subject,
            body_template: input.body_template,
            is_active: input.is_active,
            created_at: Utc::now(), // not updated in DB, but needed for struct
        };

        self.repo
            .update_template(&template)
            .await
            .map_err(AppError::Internal)?;

        Ok(template)
    }

    pub async fn delete_template(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
        self.repo
            .delete_template(tenant_id, id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn send_notification(
        &self,
        tenant_id: Uuid,
        input: SendNotificationInput,
    ) -> Result<Notification, AppError> {
        let channel = NotificationChannel::from_str(&input.channel);

        // Find the active template
        let template = self
            .repo
            .find_template(tenant_id, &input.template_type, channel.as_str())
            .await
            .map_err(AppError::Internal)?
            .ok_or(NotificationError::TemplateNotFound)?;

        if !template.is_active {
            return Err(NotificationError::TemplateNotFound.into());
        }

        // Render the body by replacing {{key}} placeholders
        let rendered_body = render_template(&template.body_template, &input.payload)?;

        let notification = Notification {
            id: Uuid::now_v7(),
            tenant_id,
            recipient_phone: input.recipient_phone,
            channel,
            template_type: input.template_type,
            payload: input.payload,
            rendered_body: Some(rendered_body),
            status: NotificationStatus::Pending,
            sent_at: None,
            error: None,
            created_at: Utc::now(),
        };

        self.repo
            .create_notification(&notification)
            .await
            .map_err(AppError::Internal)?;

        Ok(notification)
    }

    pub async fn list_notifications(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, AppError> {
        self.repo
            .list_notifications(tenant_id, limit, offset)
            .await
            .map_err(AppError::Internal)
    }
}

/// Simple template rendering: replaces `{{key}}` with values from payload JSON object.
fn render_template(body_template: &str, payload: &serde_json::Value) -> Result<String, AppError> {
    let empty = serde_json::Map::new();
    let obj = payload.as_object().unwrap_or(&empty);

    let mut result = body_template.to_string();

    for (key, value) in obj {
        let placeholder = format!("{{{{{key}}}}}");
        let replacement = match value {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        result = result.replace(&placeholder, &replacement);
    }

    Ok(result)
}
