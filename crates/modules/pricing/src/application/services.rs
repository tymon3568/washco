use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{
    AppliedRule, PriceCalculation, PricingError, PricingRule, RuleType,
};

use super::ports::PricingRepository;

pub struct PricingService<R> {
    repo: R,
}

pub struct CreateRuleInput {
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
}

pub struct UpdateRuleInput {
    pub name: String,
    pub rule_type: String,
    pub multiplier: f64,
    pub fixed_adjustment: i64,
    pub conditions: serde_json::Value,
    pub priority: i32,
    pub is_active: bool,
    pub service_id: Option<Uuid>,
    pub valid_from: Option<chrono::DateTime<chrono::Utc>>,
    pub valid_to: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct CalculatePriceInput {
    pub location_id: Uuid,
    pub service_id: Option<Uuid>,
    pub base_price: i64,
}

impl<R: PricingRepository> PricingService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_rule(
        &self,
        tenant_id: Uuid,
        input: CreateRuleInput,
    ) -> Result<PricingRule, AppError> {
        let rule_type = RuleType::from_str(&input.rule_type)
            .ok_or(PricingError::InvalidRuleType(input.rule_type.clone()))?;

        let now = Utc::now();
        let rule = PricingRule {
            id: Uuid::now_v7(),
            tenant_id,
            location_id: input.location_id,
            service_id: input.service_id,
            name: input.name,
            rule_type,
            multiplier: input.multiplier,
            fixed_adjustment: input.fixed_adjustment,
            conditions: input.conditions,
            priority: input.priority,
            is_active: input.is_active,
            valid_from: input.valid_from,
            valid_to: input.valid_to,
            created_at: now,
            updated_at: now,
        };

        self.repo
            .create(&rule)
            .await
            .map_err(AppError::Internal)?;

        Ok(rule)
    }

    pub async fn update_rule(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateRuleInput,
    ) -> Result<PricingRule, AppError> {
        let mut rule = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PricingError::RuleNotFound)?;

        let rule_type = RuleType::from_str(&input.rule_type)
            .ok_or(PricingError::InvalidRuleType(input.rule_type.clone()))?;

        rule.name = input.name;
        rule.rule_type = rule_type;
        rule.multiplier = input.multiplier;
        rule.fixed_adjustment = input.fixed_adjustment;
        rule.conditions = input.conditions;
        rule.priority = input.priority;
        rule.is_active = input.is_active;
        rule.service_id = input.service_id;
        rule.valid_from = input.valid_from;
        rule.valid_to = input.valid_to;
        rule.updated_at = Utc::now();

        self.repo
            .update(&rule)
            .await
            .map_err(AppError::Internal)?;

        Ok(rule)
    }

    pub async fn delete_rule(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<(), AppError> {
        // Verify rule exists
        self.repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PricingError::RuleNotFound)?;

        self.repo
            .delete(tenant_id, id)
            .await
            .map_err(AppError::Internal)?;

        Ok(())
    }

    pub async fn list_rules(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<PricingRule>, AppError> {
        self.repo
            .list_by_location(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn calculate_price(
        &self,
        tenant_id: Uuid,
        input: CalculatePriceInput,
    ) -> Result<PriceCalculation, AppError> {
        let now = Utc::now();
        let mut rules = self
            .repo
            .find_active_rules(tenant_id, input.location_id, input.service_id, now)
            .await
            .map_err(AppError::Internal)?;

        // Sort by priority descending (higher priority first)
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut final_price = input.base_price;
        let mut applied_rules = Vec::new();

        for rule in &rules {
            let price_before = final_price;

            // Apply multiplier
            if (rule.multiplier - 1.0).abs() > f64::EPSILON {
                final_price = (final_price as f64 * rule.multiplier).round() as i64;
            }

            // Apply fixed adjustment
            if rule.fixed_adjustment != 0 {
                final_price += rule.fixed_adjustment;
            }

            let adjustment = final_price - price_before;
            if adjustment != 0 {
                applied_rules.push(AppliedRule {
                    rule_id: rule.id,
                    rule_name: rule.name.clone(),
                    adjustment,
                });
            }
        }

        // Ensure final price is non-negative
        if final_price < 0 {
            final_price = 0;
        }

        Ok(PriceCalculation {
            base_price: input.base_price,
            final_price,
            applied_rules,
        })
    }
}
