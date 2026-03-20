use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::Promotion;

#[derive(Debug, Deserialize)]
pub struct CreatePromotionRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: i64,
    pub min_order: Option<i64>,
    pub max_uses: Option<i32>,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: chrono::DateTime<chrono::Utc>,
    pub location_ids: Option<Vec<Uuid>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePromotionRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: i64,
    pub min_order: Option<i64>,
    pub max_uses: Option<i32>,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: chrono::DateTime<chrono::Utc>,
    pub location_ids: Option<Vec<Uuid>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateCodeRequest {
    pub code: String,
    pub location_id: Option<Uuid>,
    pub order_amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct RedeemRequest {
    pub code: String,
    pub location_id: Option<Uuid>,
    pub order_amount: i64,
}

#[derive(Debug, Serialize)]
pub struct PromotionResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: i64,
    pub min_order: i64,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: chrono::DateTime<chrono::Utc>,
    pub location_ids: Vec<Uuid>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Promotion> for PromotionResponse {
    fn from(p: Promotion) -> Self {
        Self {
            id: p.id,
            code: p.code,
            name: p.name,
            description: p.description,
            discount_type: p.discount_type.to_string(),
            discount_value: p.discount_value,
            min_order: p.min_order,
            max_uses: p.max_uses,
            used_count: p.used_count,
            valid_from: p.valid_from,
            valid_to: p.valid_to,
            location_ids: p.location_ids,
            is_active: p.is_active,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DiscountResultResponse {
    pub original_price: i64,
    pub discount_amount: i64,
    pub final_price: i64,
    pub promotion_code: String,
}
