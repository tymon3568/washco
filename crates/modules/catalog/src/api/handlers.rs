use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext, resolve_tenant_for_location};

use super::CatalogState;
use super::dto::*;
use crate::application::{CreateServiceInput, UpdateServiceInput};

pub async fn list(
    State(svc): State<CatalogState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<ServiceResponse>>, AppError> {
    let services = svc.list_services(ctx.tenant_id, location_id).await?;
    let responses: Vec<ServiceResponse> = services.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

/// Public endpoint for driver app - no auth required
pub async fn public_list(
    State(svc): State<CatalogState>,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<ServiceResponse>>, AppError> {
    let tenant_id = resolve_tenant_for_location(&svc.pool, location_id).await?;
    let services = svc.list_services(tenant_id, location_id).await?;
    let responses: Vec<ServiceResponse> = services.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create(
    State(svc): State<CatalogState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateServiceRequest>,
) -> Result<(StatusCode, Json<ServiceResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let service = svc
        .create_service(CreateServiceInput {
            tenant_id: ctx.tenant_id,
            location_id,
            name: body.name,
            description: body.description,
            vehicle_type: body.vehicle_type,
            base_price: body.base_price,
            duration_minutes: body.duration_minutes,
        })
        .await?;

    Ok((StatusCode::CREATED, Json(service.into())))
}

pub async fn get_by_id(
    State(svc): State<CatalogState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, AppError> {
    let service = svc.get_service(ctx.tenant_id, id).await?;
    Ok(Json(service.into()))
}

pub async fn update(
    State(svc): State<CatalogState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateServiceRequest>,
) -> Result<Json<ServiceResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let service = svc
        .update_service(UpdateServiceInput {
            tenant_id: ctx.tenant_id,
            id,
            name: body.name,
            description: body.description,
            base_price: body.base_price,
            duration_minutes: body.duration_minutes,
            is_active: body.is_active,
            sort_order: body.sort_order,
        })
        .await?;

    Ok(Json(service.into()))
}

pub async fn delete(
    State(svc): State<CatalogState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    ctx.require_owner_or_admin()?;
    svc.delete_service(ctx.tenant_id, id).await?;
    Ok(Json(MessageResponse {
        message: "Service deleted".to_string(),
    }))
}
