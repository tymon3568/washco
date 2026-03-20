use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

impl NotificationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Sent => "sent",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "sent" => Self::Sent,
            "failed" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

impl std::fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationChannel {
    Sms,
    Push,
}

impl NotificationChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sms => "sms",
            Self::Push => "push",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "push" => Self::Push,
            _ => Self::Sms,
        }
    }
}

impl std::fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub template_type: String,
    pub channel: NotificationChannel,
    pub subject: Option<String>,
    pub body_template: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub recipient_phone: String,
    pub channel: NotificationChannel,
    pub template_type: String,
    pub payload: serde_json::Value,
    pub rendered_body: Option<String>,
    pub status: NotificationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
}
