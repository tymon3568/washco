use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{AdminAction, AdminLocationView, PlatformMetrics, SubscriptionTier};

use super::ports::AdminRepository;

pub struct AdminService<R> {
    repo: R,
}

impl<R: AdminRepository> AdminService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list_locations(
        &self,
        status_filter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AdminLocationView>, AppError> {
        self.repo
            .list_locations(status_filter, limit, offset)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn get_location(&self, id: Uuid) -> Result<Option<AdminLocationView>, AppError> {
        self.repo.get_location(id).await.map_err(AppError::Internal)
    }

    pub async fn approve_location(&self, id: Uuid, admin_user_id: Uuid) -> Result<(), AppError> {
        self.repo
            .update_location_status(id, "active")
            .await
            .map_err(AppError::Internal)?;

        let action = AdminAction {
            id: Uuid::now_v7(),
            admin_user_id,
            action_type: "approve_location".to_string(),
            target_type: "location".to_string(),
            target_id: id,
            reason: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        self.repo
            .log_action(&action)
            .await
            .map_err(AppError::Internal)?;

        Ok(())
    }

    pub async fn suspend_location(
        &self,
        id: Uuid,
        admin_user_id: Uuid,
        reason: Option<String>,
    ) -> Result<(), AppError> {
        self.repo
            .update_location_status(id, "suspended")
            .await
            .map_err(AppError::Internal)?;

        let action = AdminAction {
            id: Uuid::now_v7(),
            admin_user_id,
            action_type: "suspend_location".to_string(),
            target_type: "location".to_string(),
            target_id: id,
            reason: reason.clone(),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        self.repo
            .log_action(&action)
            .await
            .map_err(AppError::Internal)?;

        Ok(())
    }

    pub async fn platform_metrics(&self) -> Result<PlatformMetrics, AppError> {
        self.repo
            .platform_metrics()
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_actions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AdminAction>, AppError> {
        self.repo
            .list_actions(limit, offset)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_tiers(&self) -> Result<Vec<SubscriptionTier>, AppError> {
        self.repo.list_tiers().await.map_err(AppError::Internal)
    }
}
