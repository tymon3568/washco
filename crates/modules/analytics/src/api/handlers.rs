use axum::Json;
use axum::extract::{Path, Query, State};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::AnalyticsState;
use super::dto::*;

pub async fn daily_summary(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<DailySummaryResponse>, AppError> {
    let date = params.date_or_today();
    let summary = svc.daily_summary(ctx.tenant_id, location_id, date).await?;
    Ok(Json(summary.into()))
}

pub async fn utilization(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<UtilizationResponse>, AppError> {
    let date = params.date_or_today();
    // Default: 4 bays, 12 hours (720 minutes) operating day
    let bay_count = 4;
    let operating_hours_minutes = 720;
    let utilization = svc
        .bay_utilization(
            ctx.tenant_id,
            location_id,
            date,
            bay_count,
            operating_hours_minutes,
        )
        .await?;
    Ok(Json(utilization.into()))
}

pub async fn service_breakdown(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<Vec<ServiceMetricResponse>>, AppError> {
    let date = params.date_or_today();
    let metrics = svc
        .service_breakdown(ctx.tenant_id, location_id, date)
        .await?;
    Ok(Json(metrics.into_iter().map(Into::into).collect()))
}
