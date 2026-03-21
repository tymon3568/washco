use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::PricingState;
use crate::application::{CalculatePriceInput, CreateRuleInput, UpdateRuleInput};

pub async fn create_rule(
    State(svc): State<PricingState>,
    ctx: TenantContext,
    Json(body): Json<CreateRuleRequest>,
) -> Result<(StatusCode, Json<PricingRuleResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let rule = svc
        .create_rule(
            ctx.tenant_id,
            CreateRuleInput {
                location_id: body.location_id,
                service_id: body.service_id,
                name: body.name,
                rule_type: body.rule_type,
                multiplier: body.multiplier,
                fixed_adjustment: body.fixed_adjustment,
                conditions: body.conditions,
                priority: body.priority,
                is_active: body.is_active,
                valid_from: body.valid_from,
                valid_to: body.valid_to,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(rule.into())))
}

pub async fn list_rules(
    State(svc): State<PricingState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<PricingRuleResponse>>, AppError> {
    let rules = svc.list_rules(ctx.tenant_id, location_id).await?;
    Ok(Json(rules.into_iter().map(Into::into).collect()))
}

pub async fn update_rule(
    State(svc): State<PricingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRuleRequest>,
) -> Result<Json<PricingRuleResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let rule = svc
        .update_rule(
            ctx.tenant_id,
            id,
            UpdateRuleInput {
                name: body.name,
                rule_type: body.rule_type,
                multiplier: body.multiplier,
                fixed_adjustment: body.fixed_adjustment,
                conditions: body.conditions,
                priority: body.priority,
                is_active: body.is_active,
                service_id: body.service_id,
                valid_from: body.valid_from,
                valid_to: body.valid_to,
            },
        )
        .await?;

    Ok(Json(rule.into()))
}

pub async fn delete_rule(
    State(svc): State<PricingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_manager_or_above()?;
    svc.delete_rule(ctx.tenant_id, id).await?;
    Ok(Json(serde_json::json!({ "message": "Deleted" })))
}

pub async fn calculate_price(
    State(svc): State<PricingState>,
    ctx: TenantContext,
    Json(body): Json<CalculatePriceRequest>,
) -> Result<Json<PriceCalculationResponse>, AppError> {
    let calculation = svc
        .calculate_price(
            ctx.tenant_id,
            CalculatePriceInput {
                location_id: body.location_id,
                service_id: body.service_id,
                base_price: body.base_price,
            },
        )
        .await?;

    Ok(Json(calculation.into()))
}
