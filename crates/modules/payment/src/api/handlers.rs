use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, Role, TenantContext};

use super::PaymentState;
use super::dto::*;
use crate::application::CreatePaymentInput;

pub async fn create(
    State(svc): State<PaymentState>,
    ctx: TenantContext,
    Json(body): Json<CreatePaymentRequest>,
) -> Result<(StatusCode, Json<PaymentResponse>), AppError> {
    ctx.require_role(&[Role::Owner, Role::Manager, Role::Cashier, Role::Staff])?;

    let input = CreatePaymentInput {
        location_id: body.location_id,
        queue_entry_id: body.queue_entry_id,
        booking_id: body.booking_id,
        customer_name: body.customer_name,
        customer_phone: body.customer_phone,
        service_id: body.service_id,
        service_name: body.service_name,
        base_price: body.base_price,
        discount_amount: body.discount_amount,
        final_amount: body.final_amount,
        promotion_id: body.promotion_id,
        payment_method: body.payment_method,
        collected_by: ctx.user_id,
        staff_id: body.staff_id,
        assistant_id: body.assistant_id,
        notes: body.notes,
    };

    let payment = svc.create_payment(ctx.tenant_id, input).await?;

    Ok((StatusCode::CREATED, Json(payment.into())))
}

pub async fn get_by_id(
    State(svc): State<PaymentState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<PaymentResponse>, AppError> {
    let payment = svc.get_payment(ctx.tenant_id, id).await?;
    Ok(Json(payment.into()))
}

pub async fn complete(
    State(svc): State<PaymentState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<PaymentResponse>, AppError> {
    ctx.require_role(&[Role::Owner, Role::Manager])?;

    let payment = svc
        .complete_payment(ctx.tenant_id, id, Some(ctx.user_id))
        .await?;

    Ok(Json(payment.into()))
}

pub async fn list_by_location(
    State(svc): State<PaymentState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<PaymentResponse>>, AppError> {
    let date = query.date_or_today();
    let payments = svc
        .list_by_location(ctx.tenant_id, location_id, date)
        .await?;

    Ok(Json(payments.into_iter().map(Into::into).collect()))
}

pub async fn daily_revenue(
    State(svc): State<PaymentState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<DateQuery>,
) -> Result<Json<DailyRevenueResponse>, AppError> {
    let date = query.date_or_today();
    let summary = svc.daily_revenue(ctx.tenant_id, location_id, date).await?;

    Ok(Json(summary.into()))
}

pub async fn staff_earnings(
    State(svc): State<PaymentState>,
    ctx: TenantContext,
    Path(staff_id): Path<Uuid>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Vec<StaffEarningResponse>>, AppError> {
    let from = query.from_or_today();
    let to = query.to_or_today();
    let earnings = svc
        .staff_earnings(ctx.tenant_id, staff_id, from, to)
        .await?;

    Ok(Json(earnings.into_iter().map(Into::into).collect()))
}
