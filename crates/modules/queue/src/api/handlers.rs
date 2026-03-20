use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::QueueState;
use crate::application::JoinInput;

pub async fn get_queue(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<QueueStateResponse>, AppError> {
    let view = svc.get_queue(ctx.tenant_id, location_id).await?;

    Ok(Json(QueueStateResponse {
        location_id,
        waiting: view.waiting.into_iter().map(Into::into).collect(),
        in_progress: view.in_progress.into_iter().map(Into::into).collect(),
        completed_today: view.completed_today,
        estimated_wait_minutes: view.estimated_wait.estimated_minutes,
    }))
}

pub async fn join(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<JoinQueueRequest>,
) -> Result<Json<QueueEntryResponse>, AppError> {
    let entry = svc
        .join(
            ctx.tenant_id,
            location_id,
            JoinInput {
                customer_name: body.customer_name,
                customer_phone: body.customer_phone,
                vehicle_type: body.vehicle_type,
                service_id: body.service_id,
                service_name: body.service_name,
            },
        )
        .await?;

    Ok(Json(entry.into()))
}

pub async fn advance(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<AdvanceRequest>,
) -> Result<Json<QueueEntryResponse>, AppError> {
    let entry = svc.advance(ctx.tenant_id, id, body.bay_id).await?;
    Ok(Json(entry.into()))
}

pub async fn complete(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<QueueEntryResponse>, AppError> {
    let entry = svc.complete(ctx.tenant_id, id).await?;
    Ok(Json(entry.into()))
}

pub async fn cancel(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    svc.cancel(ctx.tenant_id, id).await?;
    Ok(Json(serde_json::json!({ "message": "Cancelled" })))
}
