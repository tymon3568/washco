use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::BookingState;
use crate::application::CreateBookingInput;

pub async fn create_booking(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateBookingRequest>,
) -> Result<Json<BookingResponse>, AppError> {
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

    Ok(Json(booking.into()))
}

pub async fn list_by_location(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<BookingResponse>>, AppError> {
    let bookings = svc
        .list_by_location(ctx.tenant_id, location_id, query.date)
        .await?;

    Ok(Json(bookings.into_iter().map(Into::into).collect()))
}

pub async fn list_by_phone(
    State(svc): State<BookingState>,
    _ctx: TenantContext,
    Path(phone): Path<String>,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<BookingResponse>>, AppError> {
    let bookings = svc.list_by_phone(&phone, query.date).await?;

    Ok(Json(bookings.into_iter().map(Into::into).collect()))
}

pub async fn confirm(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, AppError> {
    let booking = svc.confirm(ctx.tenant_id, id).await?;
    Ok(Json(booking.into()))
}

pub async fn complete(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, AppError> {
    let booking = svc.complete(ctx.tenant_id, id).await?;
    Ok(Json(booking.into()))
}

pub async fn cancel(
    State(svc): State<BookingState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<BookingResponse>, AppError> {
    let booking = svc.cancel(ctx.tenant_id, id).await?;
    Ok(Json(booking.into()))
}
