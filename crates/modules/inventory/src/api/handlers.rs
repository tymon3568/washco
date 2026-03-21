use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::InventoryState;
use super::dto::*;
use crate::application::{
    CreateMaterialInput, RecordTransactionInput, SetNormInput, UpdateMaterialInput,
};

// --- Materials ---

pub async fn list_materials(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<MaterialResponse>>, AppError> {
    let materials = svc.list_materials(ctx.tenant_id, location_id).await?;
    let responses: Vec<MaterialResponse> = materials.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_material(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateMaterialRequest>,
) -> Result<(StatusCode, Json<MaterialResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let material = svc
        .create_material(
            ctx.tenant_id,
            CreateMaterialInput {
                location_id,
                name: body.name,
                category: body.category,
                unit: body.unit,
                unit_cost: body.unit_cost,
                current_stock: body.current_stock,
                min_stock: body.min_stock,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(material.into())))
}

pub async fn get_material(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<MaterialResponse>, AppError> {
    let material = svc.get_material(ctx.tenant_id, id).await?;
    Ok(Json(material.into()))
}

pub async fn update_material(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateMaterialRequest>,
) -> Result<Json<MaterialResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let material = svc
        .update_material(
            ctx.tenant_id,
            id,
            UpdateMaterialInput {
                name: body.name,
                category: body.category,
                unit: body.unit,
                unit_cost: body.unit_cost,
                min_stock: body.min_stock,
                is_active: body.is_active,
            },
        )
        .await?;

    Ok(Json(material.into()))
}

// --- Norms ---

pub async fn set_norm(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Json(body): Json<SetNormRequest>,
) -> Result<(StatusCode, Json<MaterialNormResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let norm = svc
        .set_norm(
            ctx.tenant_id,
            SetNormInput {
                service_id: body.service_id,
                material_id: body.material_id,
                quantity_per_job: body.quantity_per_job,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(norm.into())))
}

pub async fn list_norms(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(service_id): Path<Uuid>,
) -> Result<Json<Vec<MaterialNormResponse>>, AppError> {
    let norms = svc.list_norms(ctx.tenant_id, service_id).await?;
    let responses: Vec<MaterialNormResponse> = norms.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn delete_norm(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    ctx.require_manager_or_above()?;
    svc.delete_norm(ctx.tenant_id, id).await?;
    Ok(Json(MessageResponse {
        message: "Norm deleted".to_string(),
    }))
}

// --- Transactions ---

pub async fn record_transaction(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Json(body): Json<RecordTransactionRequest>,
) -> Result<(StatusCode, Json<InventoryTransactionResponse>), AppError> {
    ctx.require_role(&[
        washco_shared::Role::Owner,
        washco_shared::Role::Admin,
        washco_shared::Role::Manager,
        washco_shared::Role::Staff,
    ])?;

    let transaction = svc
        .record_transaction(
            ctx.tenant_id,
            RecordTransactionInput {
                material_id: body.material_id,
                transaction_type: body.transaction_type,
                quantity: body.quantity,
                unit_cost: body.unit_cost,
                reference_id: body.reference_id,
                reference_type: body.reference_type,
                notes: body.notes,
                performed_by: ctx.user_id,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(transaction.into())))
}

pub async fn list_transactions(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(material_id): Path<Uuid>,
) -> Result<Json<Vec<InventoryTransactionResponse>>, AppError> {
    let transactions = svc
        .list_transactions(ctx.tenant_id, material_id, 100)
        .await?;
    let responses: Vec<InventoryTransactionResponse> =
        transactions.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

// --- Reports ---

pub async fn low_stock_alerts(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<LowStockAlertResponse>>, AppError> {
    ctx.require_manager_or_above()?;
    let alerts = svc.low_stock_alerts(ctx.tenant_id, location_id).await?;
    let responses: Vec<LowStockAlertResponse> = alerts.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn material_variance(
    State(svc): State<InventoryState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<VarianceQuery>,
) -> Result<Json<Vec<MaterialVarianceResponse>>, AppError> {
    ctx.require_manager_or_above()?;
    let variances = svc
        .material_variance(ctx.tenant_id, location_id, query.from, query.to)
        .await?;
    let responses: Vec<MaterialVarianceResponse> = variances.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}
