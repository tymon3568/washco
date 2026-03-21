use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::WeatherState;
use super::dto::*;
use crate::application::CreateWeatherTriggerInput;

pub async fn create_trigger(
    State(svc): State<WeatherState>,
    ctx: TenantContext,
    Json(body): Json<CreateTriggerRequest>,
) -> Result<(StatusCode, Json<WeatherTriggerResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let trigger = svc
        .create_trigger(
            ctx.tenant_id,
            CreateWeatherTriggerInput {
                promotion_id: body.promotion_id,
                location_id: body.location_id,
                trigger_condition: body.trigger_condition,
                auto_activate: body.auto_activate.unwrap_or(false),
            },
        )
        .await?;
    Ok((StatusCode::CREATED, Json(trigger.into())))
}

pub async fn list_triggers(
    State(svc): State<WeatherState>,
    ctx: TenantContext,
    Query(params): Query<ListTriggersQuery>,
) -> Result<Json<Vec<WeatherTriggerResponse>>, AppError> {
    let triggers = svc.list_triggers(ctx.tenant_id, params.location_id).await?;
    Ok(Json(triggers.into_iter().map(Into::into).collect()))
}

pub async fn update_trigger(
    State(svc): State<WeatherState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTriggerRequest>,
) -> Result<Json<WeatherTriggerResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let trigger = svc
        .update_trigger(
            ctx.tenant_id,
            id,
            body.trigger_condition,
            body.auto_activate,
            body.is_active,
        )
        .await?;
    Ok(Json(trigger.into()))
}

pub async fn delete_trigger(
    State(svc): State<WeatherState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_owner_or_admin()?;
    svc.delete_trigger(ctx.tenant_id, id).await?;
    Ok(Json(serde_json::json!({ "message": "Deleted" })))
}

pub async fn report_weather(
    State(svc): State<WeatherState>,
    _ctx: TenantContext,
    Json(body): Json<ReportWeatherRequest>,
) -> Result<Json<WeatherDataResponse>, AppError> {
    let data = svc
        .report_weather(&body.city, &body.condition, body.temperature_c, body.humidity)
        .await?;
    Ok(Json(data.into()))
}

pub async fn latest_weather(
    State(svc): State<WeatherState>,
    _ctx: TenantContext,
    Path(city): Path<String>,
) -> Result<Json<Option<WeatherDataResponse>>, AppError> {
    let data = svc.latest_weather(&city).await?;
    Ok(Json(data.map(Into::into)))
}

pub async fn evaluate(
    State(svc): State<WeatherState>,
    ctx: TenantContext,
    Json(body): Json<EvaluateRequest>,
) -> Result<Json<Vec<WeatherTriggerResponse>>, AppError> {
    let matched = svc
        .evaluate_triggers(ctx.tenant_id, body.location_id, &body.condition)
        .await?;
    Ok(Json(matched.into_iter().map(Into::into).collect()))
}
