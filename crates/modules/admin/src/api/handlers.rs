use axum::Json;
use axum::extract::{Path, Query, State};
use uuid::Uuid;
use washco_shared::{AppError, Role, TenantContext};

use super::AdminState;
use super::dto::*;

pub async fn list_locations(
    State(svc): State<AdminState>,
    ctx: TenantContext,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<AdminLocationResponse>>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    let locations = svc
        .list_locations(params.status.as_deref(), limit, offset)
        .await?;

    Ok(Json(locations.into_iter().map(Into::into).collect()))
}

pub async fn get_location(
    State(svc): State<AdminState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminLocationResponse>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    let location = svc
        .get_location(id)
        .await?
        .ok_or(AppError::NotFound { entity: "location" })?;

    Ok(Json(location.into()))
}

pub async fn approve_location(
    State(svc): State<AdminState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    svc.approve_location(id, ctx.user_id).await?;

    Ok(Json(serde_json::json!({ "status": "approved" })))
}

pub async fn suspend_location(
    State(svc): State<AdminState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<SuspendRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    svc.suspend_location(id, ctx.user_id, body.reason).await?;

    Ok(Json(serde_json::json!({ "status": "suspended" })))
}

pub async fn platform_metrics(
    State(svc): State<AdminState>,
    ctx: TenantContext,
) -> Result<Json<PlatformMetricsResponse>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    let metrics = svc.platform_metrics().await?;

    Ok(Json(metrics.into()))
}

pub async fn list_actions(
    State(svc): State<AdminState>,
    ctx: TenantContext,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<AdminActionResponse>>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    let actions = svc.list_actions(limit, offset).await?;

    Ok(Json(actions.into_iter().map(Into::into).collect()))
}

pub async fn list_tiers(
    State(svc): State<AdminState>,
    ctx: TenantContext,
) -> Result<Json<Vec<TierResponse>>, AppError> {
    ctx.require_role(&[Role::Admin])?;

    let tiers = svc.list_tiers().await?;

    Ok(Json(tiers.into_iter().map(Into::into).collect()))
}
