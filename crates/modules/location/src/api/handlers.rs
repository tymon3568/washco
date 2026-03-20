use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::LocationState;
use crate::application::{CreateLocationInput, UpdateLocationInput};

pub async fn create(
    State(svc): State<LocationState>,
    ctx: TenantContext,
    Json(body): Json<CreateLocationRequest>,
) -> Result<Json<LocationResponse>, AppError> {
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

    Ok(Json(location.into()))
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
