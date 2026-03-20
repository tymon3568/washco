use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{QueueEntry, QueueError, QueueStatus, WaitEstimate};

use super::ports::QueueRepository;

pub struct QueueService<R> {
    repo: R,
}

/// Input data for joining the queue.
pub struct JoinInput {
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub vehicle_type: String,
    pub service_id: Uuid,
    pub service_name: String,
}

/// Represents the full queue state for a location.
pub struct QueueView {
    pub waiting: Vec<QueueEntry>,
    pub in_progress: Vec<QueueEntry>,
    pub completed_today: i64,
    pub estimated_wait: WaitEstimate,
}

impl<R: QueueRepository> QueueService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn join(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        input: JoinInput,
    ) -> Result<QueueEntry, AppError> {
        let queue_number = self
            .repo
            .next_queue_number(tenant_id, location_id)
            .await
            .map_err(|e| AppError::Internal(e))?;

        let entry = QueueEntry {
            id: Uuid::now_v7(),
            tenant_id,
            location_id,
            queue_number,
            customer_name: input.customer_name,
            customer_phone: input.customer_phone,
            vehicle_type: input.vehicle_type,
            service_id: input.service_id,
            service_name: input.service_name,
            bay_id: None,
            status: QueueStatus::Waiting,
            joined_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };

        self.repo
            .create(&entry)
            .await
            .map_err(|e| AppError::Internal(e))?;

        Ok(entry)
    }

    pub async fn advance(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        bay_id: Option<Uuid>,
    ) -> Result<QueueEntry, AppError> {
        let mut entry = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(|e| AppError::Internal(e))?
            .ok_or(QueueError::EntryNotFound)?;

        entry.advance(bay_id)?;

        self.repo
            .update_status(&entry)
            .await
            .map_err(|e| AppError::Internal(e))?;

        Ok(entry)
    }

    pub async fn complete(&self, tenant_id: Uuid, id: Uuid) -> Result<QueueEntry, AppError> {
        let mut entry = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(|e| AppError::Internal(e))?
            .ok_or(QueueError::EntryNotFound)?;

        entry.complete()?;

        self.repo
            .update_status(&entry)
            .await
            .map_err(|e| AppError::Internal(e))?;

        Ok(entry)
    }

    pub async fn cancel(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
        let mut entry = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(|e| AppError::Internal(e))?
            .ok_or(QueueError::EntryNotFound)?;

        entry.cancel()?;

        self.repo
            .update_status(&entry)
            .await
            .map_err(|e| AppError::Internal(e))?;

        Ok(())
    }

    pub async fn get_queue(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<QueueView, AppError> {
        let entries = self
            .repo
            .find_active_by_location(tenant_id, location_id)
            .await
            .map_err(|e| AppError::Internal(e))?;

        let waiting: Vec<QueueEntry> = entries
            .iter()
            .filter(|e| e.status == QueueStatus::Waiting)
            .cloned()
            .collect();

        let in_progress: Vec<QueueEntry> = entries
            .iter()
            .filter(|e| e.status == QueueStatus::InProgress)
            .cloned()
            .collect();

        let completed_today = self
            .repo
            .completed_today_count(tenant_id, location_id)
            .await
            .map_err(|e| AppError::Internal(e))?;

        // For the overall location wait estimate, use current time as reference.
        let estimated_wait = if waiting.is_empty() {
            WaitEstimate {
                position: 0,
                estimated_minutes: 0,
            }
        } else {
            // Estimate for a new joiner: position = total waiting count
            WaitEstimate {
                position: waiting.len() as i32,
                estimated_minutes: (waiting.len() as i32) * 15, // default 15 min per entry
            }
        };

        Ok(QueueView {
            waiting,
            in_progress,
            completed_today,
            estimated_wait,
        })
    }

    pub async fn estimate_wait(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        entry: &QueueEntry,
    ) -> Result<WaitEstimate, AppError> {
        self.repo
            .estimate_wait(tenant_id, location_id, entry.joined_at)
            .await
            .map_err(|e| AppError::Internal(e))
    }
}
