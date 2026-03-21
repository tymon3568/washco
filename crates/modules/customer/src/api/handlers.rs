use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, Role, TenantContext};

use super::CustomerState;
use super::dto::*;
use crate::application::{
    AddServiceRecordInput, AddVehicleInput, CreateCustomerInput, CreateMembershipInput,
    UpdateCustomerInput,
};

// -- Customers --

pub async fn create_customer(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Json(body): Json<CreateCustomerRequest>,
) -> Result<(StatusCode, Json<CustomerResponse>), AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;

    let customer = svc
        .create_customer(
            ctx.tenant_id,
            CreateCustomerInput {
                phone: body.phone,
                name: body.name,
                email: body.email,
                notes: body.notes,
                tags: body.tags,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(customer.into())))
}

pub async fn list_customers(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Query(query): Query<CustomerListQuery>,
) -> Result<Json<Vec<CustomerResponse>>, AppError> {
    let customers = svc
        .list_customers(
            ctx.tenant_id,
            query.segment.as_deref(),
            query.limit.min(100),
            query.offset,
        )
        .await?;

    let responses: Vec<CustomerResponse> = customers.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn get_customer(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<CustomerResponse>, AppError> {
    let customer = svc.get_customer(ctx.tenant_id, id).await?;
    Ok(Json(customer.into()))
}

pub async fn update_customer(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCustomerRequest>,
) -> Result<Json<CustomerResponse>, AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;

    let customer = svc
        .update_customer(
            ctx.tenant_id,
            id,
            UpdateCustomerInput {
                name: body.name,
                email: body.email,
                notes: body.notes,
                tags: body.tags,
                loyalty_points: body.loyalty_points,
            },
        )
        .await?;

    Ok(Json(customer.into()))
}

pub async fn find_by_phone(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(phone): Path<String>,
) -> Result<Json<Option<CustomerResponse>>, AppError> {
    let customer = svc.find_by_phone(ctx.tenant_id, &phone).await?;
    Ok(Json(customer.map(Into::into)))
}

// -- Vehicles --

pub async fn add_vehicle(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(customer_id): Path<Uuid>,
    Json(body): Json<AddVehicleRequest>,
) -> Result<(StatusCode, Json<VehicleResponse>), AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;

    let vehicle = svc
        .add_vehicle(
            ctx.tenant_id,
            AddVehicleInput {
                customer_id,
                plate_number: body.plate_number,
                vehicle_type: body.vehicle_type,
                brand: body.brand,
                model: body.model,
                color: body.color,
                year: body.year,
                notes: body.notes,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(vehicle.into())))
}

pub async fn list_vehicles(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<Vec<VehicleResponse>>, AppError> {
    let vehicles = svc.list_vehicles(ctx.tenant_id, customer_id).await?;
    let responses: Vec<VehicleResponse> = vehicles.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn find_by_plate(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(plate): Path<String>,
) -> Result<Json<Option<VehicleResponse>>, AppError> {
    let vehicle = svc.find_by_plate(ctx.tenant_id, &plate).await?;
    Ok(Json(vehicle.map(Into::into)))
}

// -- Service history --

pub async fn add_service_record(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(customer_id): Path<Uuid>,
    Json(body): Json<AddServiceRecordRequest>,
) -> Result<(StatusCode, Json<ServiceHistoryResponse>), AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;

    let record = svc
        .add_service_record(
            ctx.tenant_id,
            AddServiceRecordInput {
                vehicle_id: body.vehicle_id,
                customer_id,
                location_id: body.location_id,
                payment_id: body.payment_id,
                service_id: body.service_id,
                service_name: body.service_name,
                amount_paid: body.amount_paid,
                staff_name: body.staff_name,
                notes: body.notes,
                next_recommended_date: body.next_recommended_date,
                next_recommended_service: body.next_recommended_service,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(record.into())))
}

pub async fn vehicle_history(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(vehicle_id): Path<Uuid>,
) -> Result<Json<Vec<ServiceHistoryResponse>>, AppError> {
    let entries = svc.vehicle_history(ctx.tenant_id, vehicle_id).await?;
    let responses: Vec<ServiceHistoryResponse> = entries.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn due_reminders(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Query(query): Query<ReminderQuery>,
) -> Result<Json<Vec<ReminderResponse>>, AppError> {
    ctx.require_manager_or_above()?;

    let as_of = query
        .as_of
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let entries = svc.due_reminders(ctx.tenant_id, as_of).await?;
    let responses: Vec<ReminderResponse> = entries.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

// -- Memberships --

pub async fn create_membership(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(customer_id): Path<Uuid>,
    Json(body): Json<CreateMembershipRequest>,
) -> Result<(StatusCode, Json<MembershipResponse>), AppError> {
    ctx.require_manager_or_above()?;

    let membership = svc
        .create_membership(
            ctx.tenant_id,
            CreateMembershipInput {
                customer_id,
                plan_name: body.plan_name,
                plan_type: body.plan_type,
                total_uses: body.total_uses,
                price_paid: body.price_paid,
                valid_from: body.valid_from,
                valid_to: body.valid_to,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(membership.into())))
}

pub async fn list_memberships(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<Vec<MembershipResponse>>, AppError> {
    let memberships = svc.list_memberships(ctx.tenant_id, customer_id).await?;
    let responses: Vec<MembershipResponse> = memberships.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn use_membership(
    State(svc): State<CustomerState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<MembershipResponse>, AppError> {
    ctx.require_role(&[
        Role::Owner,
        Role::Admin,
        Role::Manager,
        Role::Cashier,
        Role::Staff,
    ])?;

    let membership = svc.use_membership(ctx.tenant_id, id).await?;
    Ok(Json(membership.into()))
}
