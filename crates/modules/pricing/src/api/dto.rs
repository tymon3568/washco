use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{AppliedRule, PriceCalculation, PricingRule};

#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub location_id: Uuid,
    pub service_id: Option<Uuid>,
    pub name: String,
    pub rule_type: String,
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
    #[serde(default)]
    pub fixed_adjustment: i64,
    #[serde(default = "default_conditions")]
    pub conditions: serde_json::Value,
    #[serde(default)]
    pub priority: i32,
    #[serde(default = "default_true")]
    pub is_active: bool,
    pub valid_from: Option<chrono::DateTime<chrono::Utc>>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
}

fn default_multiplier() -> f64 {
    1.0
}

fn default_conditions() -> serde_json::Value {
    serde_json::json!({})
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct UpdateRuleRequest {
    pub name: String,
    pub rule_type: String,
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
    #[serde(default)]
    pub fixed_adjustment: i64,
    #[serde(default = "default_conditions")]
    pub conditions: serde_json::Value,
    #[serde(default)]
    pub priority: i32,
    #[serde(default = "default_true")]
    pub is_active: bool,
    pub service_id: Option<Uuid>,
    pub valid_from: Option<chrono::DateTime<chrono::Utc>>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CalculatePriceRequest {
    pub location_id: Uuid,
    pub service_id: Option<Uuid>,
    pub base_price: i64,
}

#[derive(Debug, Serialize)]
pub struct PricingRuleResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub service_id: Option<Uuid>,
    pub name: String,
    pub rule_type: String,
    pub multiplier: f64,
    pub fixed_adjustment: i64,
    pub conditions: serde_json::Value,
    pub priority: i32,
    pub is_active: bool,
    pub valid_from: Option<chrono::DateTime<chrono::Utc>>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<PricingRule> for PricingRuleResponse {
    fn from(r: PricingRule) -> Self {
        Self {
            id: r.id,
            location_id: r.location_id,
            service_id: r.service_id,
            name: r.name,
            rule_type: r.rule_type.to_string(),
            multiplier: r.multiplier,
            fixed_adjustment: r.fixed_adjustment,
            conditions: r.conditions,
            priority: r.priority,
            is_active: r.is_active,
            valid_from: r.valid_from,
            valid_to: r.valid_to,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppliedRuleResponse {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub adjustment: i64,
}

impl From<AppliedRule> for AppliedRuleResponse {
    fn from(r: AppliedRule) -> Self {
        Self {
            rule_id: r.rule_id,
            rule_name: r.rule_name,
            adjustment: r.adjustment,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PriceCalculationResponse {
    pub base_price: i64,
    pub final_price: i64,
    pub applied_rules: Vec<AppliedRuleResponse>,
}

impl From<PriceCalculation> for PriceCalculationResponse {
    fn from(c: PriceCalculation) -> Self {
        Self {
            base_price: c.base_price,
            final_price: c.final_price,
            applied_rules: c.applied_rules.into_iter().map(Into::into).collect(),
        }
    }
}
