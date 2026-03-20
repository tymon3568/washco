use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::QueueError;

#[derive(Debug, Clone, PartialEq)]
pub enum QueueStatus {
    Waiting,
    InProgress,
    Completed,
    Cancelled,
}

impl QueueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Waiting => "waiting",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "waiting" => Self::Waiting,
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            _ => Self::Waiting,
        }
    }
}

impl std::fmt::Display for QueueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub queue_number: i32,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub vehicle_type: String,
    pub service_id: Uuid,
    pub service_name: String,
    pub bay_id: Option<Uuid>,
    pub status: QueueStatus,
    pub joined_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl QueueEntry {
    /// Transition from waiting -> in_progress, optionally assigning a bay.
    pub fn advance(&mut self, bay_id: Option<Uuid>) -> Result<(), QueueError> {
        if self.status != QueueStatus::Waiting {
            return Err(QueueError::InvalidTransition {
                from: self.status.to_string(),
                to: "in_progress".to_string(),
            });
        }
        self.status = QueueStatus::InProgress;
        self.started_at = Some(Utc::now());
        self.bay_id = bay_id;
        Ok(())
    }

    /// Transition from in_progress -> completed.
    pub fn complete(&mut self) -> Result<(), QueueError> {
        if self.status != QueueStatus::InProgress {
            return Err(QueueError::InvalidTransition {
                from: self.status.to_string(),
                to: "completed".to_string(),
            });
        }
        self.status = QueueStatus::Completed;
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    /// Transition from waiting -> cancelled.
    pub fn cancel(&mut self) -> Result<(), QueueError> {
        if self.status != QueueStatus::Waiting {
            return Err(QueueError::InvalidTransition {
                from: self.status.to_string(),
                to: "cancelled".to_string(),
            });
        }
        self.status = QueueStatus::Cancelled;
        self.completed_at = Some(Utc::now());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WaitEstimate {
    pub position: i32,
    pub estimated_minutes: i32,
}
