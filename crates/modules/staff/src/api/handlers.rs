use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::StaffState;
use super::dto::*;
use crate::application::{
    CreateCommissionRuleInput, CreateShiftInput, CreateStaffInput, RecordCommissionInput,
    UpdateStaffInput,
};

// --- Staff profiles ---

pub async fn create_staff(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateStaffRequest>,
) -> Result<(StatusCode, Json<StaffResponse>), AppError> {
    ctx.require_owner_or_admin()?;

    let profile = svc
        .create_profile(
            ctx.tenant_id,
            CreateStaffInput {
                user_id: body.user_id,
                location_id,
                display_name: body.display_name,
                skill_level: body.skill_level,
                hourly_rate: body.hourly_rate,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(profile.into())))
}

pub async fn list_staff(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<StaffResponse>>, AppError> {
    let profiles = svc.list_by_location(ctx.tenant_id, location_id).await?;
    let responses: Vec<StaffResponse> = profiles.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_staff(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<StaffResponse>, AppError> {
    let profile = svc.get_profile(ctx.tenant_id, id).await?;
    Ok(Json(profile.into()))
}

pub async fn update_staff(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateStaffRequest>,
) -> Result<Json<StaffResponse>, AppError> {
    ctx.require_manager_or_above()?;

    let profile = svc
        .update_profile(
            ctx.tenant_id,
            id,
            UpdateStaffInput {
                display_name: body.display_name,
                skill_level: body.skill_level,
                hourly_rate: body.hourly_rate,
                is_active: body.is_active,
            },
        )
        .await?;

    Ok(Json(profile.into()))
}

// --- Shifts ---

pub async fn create_shift(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateShiftRequest>,
) -> Result<(StatusCode, Json<ShiftResponse>), AppError> {
    ctx.require_manager_or_above()?;

    let shift = svc
        .create_shift(
            ctx.tenant_id,
            CreateShiftInput {
                location_id,
                staff_id: body.staff_id,
                shift_date: body.shift_date,
                start_time: body.start_time,
                end_time: body.end_time,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(shift.into())))
}

pub async fn list_shifts(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Vec<ShiftResponse>>, AppError> {
    let shifts = svc
        .list_shifts(ctx.tenant_id, location_id, query.date)
        .await?;
    let responses: Vec<ShiftResponse> = shifts.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn update_shift_status(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateShiftStatusRequest>,
) -> Result<Json<ShiftResponse>, AppError> {
    ctx.require_manager_or_above()?;

    let shift = svc
        .update_shift_status(
            ctx.tenant_id,
            id,
            body.status,
            body.actual_start,
            body.actual_end,
        )
        .await?;

    Ok(Json(shift.into()))
}

// --- Commission rules ---

pub async fn create_commission_rule(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Json(body): Json<CreateCommissionRuleRequest>,
) -> Result<(StatusCode, Json<CommissionRuleResponse>), AppError> {
    ctx.require_owner_or_admin()?;

    let rule = svc
        .create_commission_rule(
            ctx.tenant_id,
            CreateCommissionRuleInput {
                location_id,
                name: body.name,
                service_id: body.service_id,
                skill_level: body.skill_level,
                role_in_job: body.role_in_job,
                commission_type: body.commission_type,
                commission_value: body.commission_value,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(rule.into())))
}

pub async fn list_commission_rules(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<Vec<CommissionRuleResponse>>, AppError> {
    let rules = svc
        .list_commission_rules(ctx.tenant_id, location_id)
        .await?;
    let responses: Vec<CommissionRuleResponse> = rules.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

// --- Commission entries ---

pub async fn record_commission(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Json(body): Json<RecordCommissionRequest>,
) -> Result<(StatusCode, Json<CommissionEntryResponse>), AppError> {
    ctx.require_manager_or_above()?;

    let entry = svc
        .record_commission(
            ctx.tenant_id,
            RecordCommissionInput {
                payment_id: body.payment_id,
                staff_id: body.staff_id,
                rule_id: body.rule_id,
                role_in_job: body.role_in_job,
                payment_amount: body.payment_amount,
                commission_amount: body.commission_amount,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(entry.into())))
}

pub async fn commission_summary(
    State(svc): State<StaffState>,
    ctx: TenantContext,
    Path(staff_id): Path<Uuid>,
    Query(query): Query<CommissionSummaryQuery>,
) -> Result<Json<CommissionSummaryResponse>, AppError> {
    let summary = svc
        .staff_commission_summary(ctx.tenant_id, staff_id, query.from, query.to)
        .await?;

    Ok(Json(summary.into()))
}
