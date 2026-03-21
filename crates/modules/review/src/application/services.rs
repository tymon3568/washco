use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{Review, ReviewError, ReviewSummary};

use super::ports::ReviewRepository;

pub struct ReviewService<R> {
    repo: R,
}

pub struct SubmitReviewInput {
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub rating: i16,
    pub comment: Option<String>,
}

impl<R: ReviewRepository> ReviewService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn submit_review(
        &self,
        tenant_id: Uuid,
        input: SubmitReviewInput,
    ) -> Result<Review, AppError> {
        if input.rating < 1 || input.rating > 5 {
            return Err(ReviewError::InvalidRating.into());
        }

        if let Some(queue_entry_id) = input.queue_entry_id {
            let exists = self
                .repo
                .exists_for_queue_entry(tenant_id, queue_entry_id)
                .await
                .map_err(AppError::Internal)?;
            if exists {
                return Err(ReviewError::AlreadyReviewed.into());
            }
        }

        let review = Review {
            id: Uuid::now_v7(),
            tenant_id,
            location_id: input.location_id,
            queue_entry_id: input.queue_entry_id,
            customer_name: input.customer_name,
            customer_phone: input.customer_phone,
            rating: input.rating,
            comment: input.comment,
            reply: None,
            replied_at: None,
            created_at: Utc::now(),
        };

        self.repo
            .create(&review)
            .await
            .map_err(AppError::Internal)?;

        Ok(review)
    }

    pub async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Review>, AppError> {
        self.repo
            .list_by_location(tenant_id, location_id, limit, offset)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn get_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<ReviewSummary, AppError> {
        self.repo
            .summary_by_location(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn reply_to_review(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        reply: &str,
    ) -> Result<(), AppError> {
        let review = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(ReviewError::NotFound)?;

        let _ = review; // exists check done

        self.repo
            .set_reply(tenant_id, id, reply)
            .await
            .map_err(AppError::Internal)?;

        Ok(())
    }
}
