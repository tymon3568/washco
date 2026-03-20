use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    Surge,
    TimeBased,
    DayOfWeek,
    Demand,
}

impl RuleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Surge => "surge",
            Self::TimeBased => "time_based",
            Self::DayOfWeek => "day_of_week",
            Self::Demand => "demand",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "surge" => Some(Self::Surge),
            "time_based" => Some(Self::TimeBased),
            "day_of_week" => Some(Self::DayOfWeek),
            "demand" => Some(Self::Demand),
            _ => None,
        }
    }
}

impl std::fmt::Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct PricingRule {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub service_id: Option<Uuid>,
    pub name: String,
    pub rule_type: RuleType,
    pub multiplier: f64,
    pub fixed_adjustment: i64,
    pub conditions: serde_json::Value,
    pub priority: i32,
    pub is_active: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PriceCalculation {
    pub base_price: i64,
    pub final_price: i64,
    pub applied_rules: Vec<AppliedRule>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AppliedRule {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub adjustment: i64,
}
