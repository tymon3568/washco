use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::PromotionState;
use super::dto::*;
use crate::application::{CreatePromotionInput, UpdatePromotionInput};

pub async fn create_promotion(
    State(svc): State<PromotionState>,
    ctx: TenantContext,
    Json(body): Json<CreatePromotionRequest>,
) -> Result<(StatusCode, Json<PromotionResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let promo = svc
        .create_promotion(
            ctx.tenant_id,
            CreatePromotionInput {
                code: body.code,
                name: body.name,
                description: body.description,
                discount_type: body.discount_type,
                discount_value: body.discount_value,
                min_order: body.min_order.unwrap_or(0),
                max_uses: body.max_uses,
                valid_from: body.valid_from,
                valid_to: body.valid_to,
                location_ids: body.location_ids.unwrap_or_default(),
                is_active: body.is_active.unwrap_or(true),
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(promo.into())))
}

pub async fn list_promotions(
    State(svc): State<PromotionState>,
    ctx: TenantContext,
) -> Result<Json<Vec<PromotionResponse>>, AppError> {
    let promos = svc.list_promotions(ctx.tenant_id).await?;
    Ok(Json(promos.into_iter().map(Into::into).collect()))
}

pub async fn update_promotion(
    State(svc): State<PromotionState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePromotionRequest>,
) -> Result<Json<PromotionResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let promo = svc
        .update_promotion(
            ctx.tenant_id,
            id,
            UpdatePromotionInput {
                code: body.code,
                name: body.name,
                description: body.description,
                discount_type: body.discount_type,
                discount_value: body.discount_value,
                min_order: body.min_order.unwrap_or(0),
                max_uses: body.max_uses,
                valid_from: body.valid_from,
                valid_to: body.valid_to,
                location_ids: body.location_ids.unwrap_or_default(),
                is_active: body.is_active.unwrap_or(true),
            },
        )
        .await?;

    Ok(Json(promo.into()))
}

pub async fn delete_promotion(
    State(svc): State<PromotionState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_owner_or_admin()?;
    svc.delete_promotion(ctx.tenant_id, id).await?;
    Ok(Json(serde_json::json!({ "message": "Deleted" })))
}

pub async fn validate_code(
    State(svc): State<PromotionState>,
    ctx: TenantContext,
    Json(body): Json<ValidateCodeRequest>,
) -> Result<Json<DiscountResultResponse>, AppError> {
    let result = svc
        .validate_code(
            ctx.tenant_id,
            &body.code,
            body.location_id,
            body.order_amount,
        )
        .await?;

    Ok(Json(DiscountResultResponse {
        original_price: result.original_price,
        discount_amount: result.discount_amount,
        final_price: result.final_price,
        promotion_code: result.promotion_code,
    }))
}

pub async fn redeem(
    State(svc): State<PromotionState>,
    ctx: TenantContext,
    Json(body): Json<RedeemRequest>,
) -> Result<Json<DiscountResultResponse>, AppError> {
    let result = svc
        .redeem(
            ctx.tenant_id,
            &body.code,
            body.location_id,
            body.order_amount,
        )
        .await?;

    Ok(Json(DiscountResultResponse {
        original_price: result.original_price,
        discount_amount: result.discount_amount,
        final_price: result.final_price,
        promotion_code: result.promotion_code,
    }))
}
