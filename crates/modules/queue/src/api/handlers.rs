use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, Role, TenantContext};

use super::QueueState;
use super::dto::*;
use crate::application::JoinInput;

/// Verify location belongs to the caller's tenant.
async fn verify_location_ownership(
    pool: &sqlx::PgPool,
    tenant_id: Uuid,
    location_id: Uuid,
) -> Result<(), AppError> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM locations WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL)",
    )
    .bind(location_id)
    .bind(tenant_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("DB: {e}")))?;

    if !exists {
        return Err(AppError::NotFound { entity: "Location" });
    }
    Ok(())
}

pub async fn get_queue(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<QueueStateResponse>, AppError> {
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
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
) -> Result<(StatusCode, Json<QueueEntryResponse>), AppError> {
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
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

    svc.broadcast.notify(location_id, "queue_updated").await;
    Ok((StatusCode::CREATED, Json(entry.into())))
}

pub async fn advance(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<AdvanceRequest>,
) -> Result<Json<QueueEntryResponse>, AppError> {
    ctx.require_role(&[Role::Owner, Role::Manager, Role::Cashier, Role::Staff])?;
    let entry = svc.advance(ctx.tenant_id, id, body.bay_id).await?;
    svc.broadcast
        .notify(entry.location_id, "queue_updated")
        .await;
    Ok(Json(entry.into()))
}

pub async fn complete(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<QueueEntryResponse>, AppError> {
    ctx.require_role(&[Role::Owner, Role::Manager, Role::Cashier, Role::Staff])?;
    let entry = svc.complete(ctx.tenant_id, id).await?;
    svc.broadcast
        .notify(entry.location_id, "queue_updated")
        .await;
    Ok(Json(entry.into()))
}

pub async fn cancel(
    State(svc): State<QueueState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_role(&[Role::Owner, Role::Manager, Role::Cashier, Role::Staff])?;
    let entry = svc.cancel(ctx.tenant_id, id).await?;
    svc.broadcast
        .notify(entry.location_id, "queue_updated")
        .await;
    Ok(Json(serde_json::json!({ "message": "Cancelled" })))
}
