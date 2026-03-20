use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum DiscountType {
    Percentage,
    Fixed,
}

impl DiscountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Percentage => "percentage",
            Self::Fixed => "fixed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "percentage" => Self::Percentage,
            "fixed" => Self::Fixed,
            _ => Self::Percentage,
        }
    }
}

impl std::fmt::Display for DiscountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Promotion {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub min_order: i64,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub location_ids: Vec<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscountResult {
    pub original_price: i64,
    pub discount_amount: i64,
    pub final_price: i64,
    pub promotion_code: String,
}
