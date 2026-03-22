use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, Role, TenantContext, resolve_tenant_for_location};

use super::BookingState;
use super::dto::*;
use crate::application::CreateBookingInput;

pub async fn create_booking(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateBookingRequest>,
) -> Result<(StatusCode, Json<BookingResponse>), AppError> {
    let booking = svc
        .create_booking(
            ctx.tenant_id,
            location_id,
            CreateBookingInput {
                service_id: body.service_id,
                customer_name: body.customer_name,
                customer_phone: body.customer_phone,
                vehicle_type: body.vehicle_type,
                booking_date: body.booking_date,
                time_slot: body.time_slot,
                notes: body.notes,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(booking.into())))
}

/// Public endpoint for driver app - create booking without auth
pub async fn public_create_booking(
    State(svc): State<BookingState>,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateBookingRequest>,
) -> Result<(StatusCode, Json<BookingResponse>), AppError> {
    let tenant_id = resolve_tenant_for_location(&svc.pool, location_id).await?;
    let booking = svc
        .create_booking(
            tenant_id,
            location_id,
            CreateBookingInput {
                service_id: body.service_id,
                customer_name: body.customer_name,
                customer_phone: body.customer_phone,
                vehicle_type: body.vehicle_type,
                booking_date: body.booking_date,
                time_slot: body.time_slot,
                notes: body.notes,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(booking.into())))
}

pub async fn list_by_location(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<BookingResponse>>, AppError> {
    let date = query.date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let bookings = svc
        .list_by_location(ctx.tenant_id, location_id, date)
        .await?;

    Ok(Json(bookings.into_iter().map(Into::into).collect()))
}

pub async fn list_by_phone(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(phone): Path<String>,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<BookingResponse>>, AppError> {
    let bookings = svc.list_by_phone(ctx.tenant_id, &phone, query.date.as_ref()).await?;

    Ok(Json(bookings.into_iter().map(Into::into).collect()))
}

pub async fn confirm(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;
    let booking = svc.confirm(ctx.tenant_id, id).await?;
    Ok(Json(booking.into()))
}

pub async fn complete(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;
    let booking = svc.complete(ctx.tenant_id, id).await?;
    Ok(Json(booking.into()))
}

pub async fn cancel(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;
    let booking = svc.cancel(ctx.tenant_id, id).await?;
    Ok(Json(booking.into()))
}
