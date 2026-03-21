use axum::Json;
use axum::extract::{Path, Query, State};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::AnalyticsState;
use super::dto::*;

/// Verify location belongs to the caller's tenant.
async fn verify_location_ownership(
    pool: &sqlx::PgPool,
    tenant_id: Uuid,
    location_id: Uuid,
) -> Result<(), AppError> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM locations WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL)",
    )
    .bind(location_id)
    .bind(tenant_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("DB: {e}")))?;

    if !exists {
        return Err(AppError::NotFound { entity: "Location" });
    }
    Ok(())
}

pub async fn daily_summary(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<DailySummaryResponse>, AppError> {
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
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
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
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
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
    let date = params.date_or_today();
    let metrics = svc
        .service_breakdown(ctx.tenant_id, location_id, date)
        .await?;
    Ok(Json(metrics.into_iter().map(Into::into).collect()))
}

pub async fn trend(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(params): Query<TrendQuery>,
) -> Result<Json<Vec<TrendDataPointResponse>>, AppError> {
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
    let (from, to) = params.resolve();
    let points = svc.trend(ctx.tenant_id, location_id, from, to).await?;
    Ok(Json(points.into_iter().map(Into::into).collect()))
}

pub async fn period_summary(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(params): Query<TrendQuery>,
) -> Result<Json<PeriodSummaryResponse>, AppError> {
    verify_location_ownership(&svc.pool, ctx.tenant_id, location_id).await?;
    let (from, to) = params.resolve();
    let summary = svc
        .period_summary(ctx.tenant_id, location_id, from, to)
        .await?;
    Ok(Json(summary.into()))
}

pub async fn compare_locations(
    State(svc): State<AnalyticsState>,
    ctx: TenantContext,
    Query(params): Query<CompareQuery>,
) -> Result<Json<Vec<LocationComparisonResponse>>, AppError> {
    let ids = params.parse_location_ids();
    if ids.is_empty() {
        return Err(AppError::Validation {
            message: "location_ids is required".into(),
        });
    }
    let (from, to) = params.resolve_dates();
    let comparisons = svc.compare_locations(ctx.tenant_id, &ids, from, to).await?;
    Ok(Json(comparisons.into_iter().map(Into::into).collect()))
}
