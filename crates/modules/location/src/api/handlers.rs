use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::LocationState;
use super::dto::*;
use crate::application::{CreateLocationInput, OperatingHoursInput, UpdateLocationInput};

pub async fn create(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Json(body): Json<CreateLocationRequest>,
) -> Result<(StatusCode, Json<LocationResponse>), AppError> {
    ctx.require_owner_or_admin()?;
    let location = svc
        .create(
            ctx.tenant_id,
            CreateLocationInput {
                name: body.name,
                phone: body.phone,
                address: body.address,
                district: body.district,
                city: body.city,
                latitude: body.latitude,
                longitude: body.longitude,
                bay_count: body.bay_count,
                queue_mode: body.queue_mode,
                amenities: body.amenities,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(location.into())))
}

pub async fn list(
    State(svc): State<LocationState>,
    ctx: TenantContext,
) -> Result<Json<Vec<LocationResponse>>, AppError> {
    let locations = svc.list_by_tenant(ctx.tenant_id).await?;
    Ok(Json(locations.into_iter().map(Into::into).collect()))
}

pub async fn get_by_id(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<LocationResponse>, AppError> {
    let location = svc.get_by_id(ctx.tenant_id, id).await?;
    Ok(Json(location.into()))
}

pub async fn update(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateLocationRequest>,
) -> Result<Json<LocationResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let location = svc
        .update(
            ctx.tenant_id,
            id,
            UpdateLocationInput {
                name: body.name,
                phone: body.phone,
                address: body.address,
                district: body.district,
                city: body.city,
                latitude: body.latitude,
                longitude: body.longitude,
                bay_count: body.bay_count,
                queue_mode: body.queue_mode,
                status: body.status,
                amenities: body.amenities,
            },
        )
        .await?;

    Ok(Json(location.into()))
}

pub async fn delete(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    ctx.require_owner_or_admin()?;
    svc.delete(ctx.tenant_id, id).await?;
    Ok(Json(MessageResponse {
        message: "Location deleted".to_string(),
    }))
}

pub async fn nearby(
    State(svc): State<LocationState>,
    Query(query): Query<NearbyQuery>,
) -> Result<Json<Vec<NearbyLocationResponse>>, AppError> {
    let results = svc
        .find_nearby(query.lat, query.lng, query.radius_meters)
        .await?;

    Ok(Json(
        results
            .into_iter()
            .map(|(loc, distance)| NearbyLocationResponse {
                location: loc.into(),
                distance_meters: distance,
            })
            .collect(),
    ))
}

pub async fn get_operating_hours(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<OperatingHoursResponse>>, AppError> {
    let hours = svc.get_operating_hours(ctx.tenant_id, id).await?;
    Ok(Json(hours.into_iter().map(Into::into).collect()))
}

pub async fn set_operating_hours(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<SetOperatingHoursRequest>,
) -> Result<Json<Vec<OperatingHoursResponse>>, AppError> {
    ctx.require_manager_or_above()?;
    let inputs: Vec<OperatingHoursInput> = body
        .hours
        .into_iter()
        .map(|e| OperatingHoursInput {
            day_of_week: e.day_of_week,
            open_time: e.open_time,
            close_time: e.close_time,
            is_closed: e.is_closed,
        })
        .collect();

    let hours = svc.set_operating_hours(ctx.tenant_id, id, inputs).await?;

    Ok(Json(hours.into_iter().map(Into::into).collect()))
}

pub async fn list_bays(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<BayResponse>>, AppError> {
    let bays = svc.list_bays(ctx.tenant_id, location_id).await?;
    Ok(Json(bays.into_iter().map(Into::into).collect()))
}

pub async fn create_bay(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<BayRequest>,
) -> Result<(StatusCode, Json<BayResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let bay = svc
        .create_bay(ctx.tenant_id, location_id, body.name)
        .await?;
    Ok((StatusCode::CREATED, Json(bay.into())))
}

pub async fn update_bay(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(bay_id): Path<Uuid>,
    Json(body): Json<UpdateBayRequest>,
) -> Result<Json<BayResponse>, AppError> {
    let bay = svc
        .update_bay(ctx.tenant_id, bay_id, body.name, body.is_active)
        .await?;
    Ok(Json(bay.into()))
}

pub async fn delete_bay(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Path(bay_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    ctx.require_manager_or_above()?;
    svc.delete_bay(ctx.tenant_id, bay_id).await?;
    Ok(Json(MessageResponse {
        message: "Bay deleted".to_string(),
    }))
}
