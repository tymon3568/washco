use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::ReviewState;
use super::dto::*;
use crate::application::SubmitReviewInput;

/// POST / — submit a review (auth required, tenant_id from JWT)
pub async fn submit_review(
    State(svc): State<ReviewState>,
    ctx: TenantContext,
    Json(body): Json<SubmitReviewRequest>,
) -> Result<(StatusCode, Json<ReviewResponse>), AppError> {
    let review = svc
        .submit_review(
            ctx.tenant_id,
            SubmitReviewInput {
                location_id: body.location_id,
                queue_entry_id: body.queue_entry_id,
                customer_name: body.customer_name,
                customer_phone: body.customer_phone,
                rating: body.rating,
                comment: body.comment,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(review.into())))
}

/// GET /locations/{location_id} — list reviews (auth required, tenant_id from JWT)
pub async fn list_reviews(
    State(svc): State<ReviewState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
    Query(query): Query<ListReviewsQuery>,
) -> Result<Json<Vec<ReviewResponse>>, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let reviews = svc
        .list_by_location(ctx.tenant_id, location_id, limit, offset)
        .await?;

    Ok(Json(reviews.into_iter().map(Into::into).collect()))
}

/// GET /locations/{location_id}/summary — get summary (auth required, tenant_id from JWT)
pub async fn get_summary(
    State(svc): State<ReviewState>,
    ctx: TenantContext,
    Path(location_id): Path<Uuid>,
) -> Result<Json<ReviewSummaryResponse>, AppError> {
    let summary = svc.get_summary(ctx.tenant_id, location_id).await?;

    Ok(Json(ReviewSummaryResponse::from_summary(
        location_id,
        summary,
    )))
}

/// PUT /{id}/reply — owner replies to a review (auth required)
pub async fn reply_to_review(
    State(svc): State<ReviewState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<ReplyRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_manager_or_above()?;
    svc.reply_to_review(ctx.tenant_id, id, &body.reply).await?;

    Ok(Json(serde_json::json!({ "message": "Reply added" })))
}
