use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{DiscountResult, DiscountType, Promotion, PromotionError};

use super::ports::PromotionRepository;

pub struct PromotionService<R> {
    repo: R,
}

pub struct CreatePromotionInput {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: i64,
    pub min_order: i64,
    pub max_uses: Option<i32>,
    pub valid_from: chrono::DateTime<Utc>,
    pub valid_to: chrono::DateTime<Utc>,
    pub location_ids: Vec<Uuid>,
    pub is_active: bool,
}

pub struct UpdatePromotionInput {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: i64,
    pub min_order: i64,
    pub max_uses: Option<i32>,
    pub valid_from: chrono::DateTime<Utc>,
    pub valid_to: chrono::DateTime<Utc>,
    pub location_ids: Vec<Uuid>,
    pub is_active: bool,
}

impl<R: PromotionRepository> PromotionService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_promotion(
        &self,
        tenant_id: Uuid,
        input: CreatePromotionInput,
    ) -> Result<Promotion, AppError> {
        // Check for duplicate code
        let existing = self
            .repo
            .find_by_code(tenant_id, &input.code)
            .await
            .map_err(AppError::Internal)?;

        if existing.is_some() {
            return Err(PromotionError::CodeAlreadyExists.into());
        }

        let now = Utc::now();
        let promo = Promotion {
            id: Uuid::now_v7(),
            tenant_id,
            code: input.code,
            name: input.name,
            description: input.description,
            discount_type: DiscountType::from_str(&input.discount_type),
            discount_value: input.discount_value,
            min_order: input.min_order,
            max_uses: input.max_uses,
            used_count: 0,
            valid_from: input.valid_from,
            valid_to: input.valid_to,
            location_ids: input.location_ids,
            is_active: input.is_active,
            created_at: now,
            updated_at: now,
        };

        self.repo
            .create(&promo)
            .await
            .map_err(AppError::Internal)?;

        Ok(promo)
    }

    pub async fn update_promotion(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdatePromotionInput,
    ) -> Result<Promotion, AppError> {
        let mut promo = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PromotionError::NotFound)?;

        // Check for duplicate code (if code changed)
        if promo.code != input.code {
            let existing = self
                .repo
                .find_by_code(tenant_id, &input.code)
                .await
                .map_err(AppError::Internal)?;

            if existing.is_some() {
                return Err(PromotionError::CodeAlreadyExists.into());
            }
        }

        promo.code = input.code;
        promo.name = input.name;
        promo.description = input.description;
        promo.discount_type = DiscountType::from_str(&input.discount_type);
        promo.discount_value = input.discount_value;
        promo.min_order = input.min_order;
        promo.max_uses = input.max_uses;
        promo.valid_from = input.valid_from;
        promo.valid_to = input.valid_to;
        promo.location_ids = input.location_ids;
        promo.is_active = input.is_active;
        promo.updated_at = Utc::now();

        self.repo
            .update(&promo)
            .await
            .map_err(AppError::Internal)?;

        Ok(promo)
    }

    pub async fn delete_promotion(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<(), AppError> {
        // Verify it exists
        self.repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PromotionError::NotFound)?;

        self.repo
            .delete(tenant_id, id)
            .await
            .map_err(AppError::Internal)?;

        Ok(())
    }

    pub async fn list_promotions(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<Promotion>, AppError> {
        self.repo
            .list(tenant_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn validate_code(
        &self,
        tenant_id: Uuid,
        code: &str,
        location_id: Option<Uuid>,
        order_amount: i64,
    ) -> Result<DiscountResult, AppError> {
        let promo = self
            .repo
            .find_by_code(tenant_id, code)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PromotionError::InvalidCode)?;

        self.validate_promotion(&promo, location_id, order_amount)?;

        let discount_amount = self.calculate_discount(&promo, order_amount);
        let final_price = order_amount - discount_amount;

        Ok(DiscountResult {
            original_price: order_amount,
            discount_amount,
            final_price: final_price.max(0),
            promotion_code: promo.code,
        })
    }

    pub async fn redeem(
        &self,
        tenant_id: Uuid,
        code: &str,
        location_id: Option<Uuid>,
        order_amount: i64,
    ) -> Result<DiscountResult, AppError> {
        let promo = self
            .repo
            .find_by_code(tenant_id, code)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PromotionError::InvalidCode)?;

        self.validate_promotion(&promo, location_id, order_amount)?;

        let discount_amount = self.calculate_discount(&promo, order_amount);
        let final_price = order_amount - discount_amount;

        self.repo
            .increment_used_count(tenant_id, promo.id)
            .await
            .map_err(AppError::Internal)?;

        Ok(DiscountResult {
            original_price: order_amount,
            discount_amount,
            final_price: final_price.max(0),
            promotion_code: promo.code,
        })
    }

    fn validate_promotion(
        &self,
        promo: &Promotion,
        location_id: Option<Uuid>,
        order_amount: i64,
    ) -> Result<(), AppError> {
        if !promo.is_active {
            return Err(PromotionError::InvalidCode.into());
        }

        let now = Utc::now();
        if now < promo.valid_from || now > promo.valid_to {
            return Err(PromotionError::Expired.into());
        }

        if let Some(max_uses) = promo.max_uses {
            if promo.used_count >= max_uses {
                return Err(PromotionError::MaxUsesReached.into());
            }
        }

        if order_amount < promo.min_order {
            return Err(PromotionError::MinOrderNotMet.into());
        }

        if let Some(loc_id) = location_id {
            if !promo.location_ids.is_empty() && !promo.location_ids.contains(&loc_id) {
                return Err(PromotionError::NotApplicableToLocation.into());
            }
        }

        Ok(())
    }

    fn calculate_discount(&self, promo: &Promotion, order_amount: i64) -> i64 {
        match promo.discount_type {
            DiscountType::Percentage => {
                // discount_value is percentage (e.g. 10 = 10%)
                (order_amount * promo.discount_value) / 100
            }
            DiscountType::Fixed => {
                // discount_value is fixed amount in minor units
                promo.discount_value
            }
        }
    }
}
