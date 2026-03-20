use uuid::Uuid;

use crate::domain::{Notification, NotificationTemplate};

pub trait NotificationRepository: Send + Sync {
    fn create_template(
        &self,
        template: &NotificationTemplate,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_template(
        &self,
        tenant_id: Uuid,
        template_type: &str,
        channel: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<NotificationTemplate>>> + Send;

    fn list_templates(
        &self,
        tenant_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<NotificationTemplate>>> + Send;

    fn update_template(
        &self,
        template: &NotificationTemplate,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn delete_template(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn create_notification(
        &self,
        notif: &Notification,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn list_notifications(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Notification>>> + Send;

    fn mark_sent(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn mark_failed(
        &self,
        id: Uuid,
        error: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
