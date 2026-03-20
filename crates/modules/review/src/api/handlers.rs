use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::ReviewState;
use crate::application::SubmitReviewInput;

/// POST / — submit a review (public, tenant_id from body)
pub async fn submit_review(
    State(svc): State<ReviewState>,
    Json(body): Json<SubmitReviewRequest>,
) -> Result<(StatusCode, Json<ReviewResponse>), AppError> {
    let review = svc
        .submit_review(
            body.tenant_id,
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

/// GET /locations/{location_id} — list reviews (public, tenant_id from query)
pub async fn list_reviews(
    State(svc): State<ReviewState>,
    Path(location_id): Path<Uuid>,
    Query(query): Query<ListReviewsQuery>,
) -> Result<Json<Vec<ReviewResponse>>, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let reviews = svc
        .list_by_location(query.tenant_id, location_id, limit, offset)
        .await?;

    Ok(Json(reviews.into_iter().map(Into::into).collect()))
}

/// GET /locations/{location_id}/summary — get summary (public, tenant_id from query)
pub async fn get_summary(
    State(svc): State<ReviewState>,
    Path(location_id): Path<Uuid>,
    Query(query): Query<SummaryQuery>,
) -> Result<Json<ReviewSummaryResponse>, AppError> {
    let summary = svc.get_summary(query.tenant_id, location_id).await?;

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
    svc.reply_to_review(ctx.tenant_id, id, &body.reply).await?;

    Ok(Json(serde_json::json!({ "message": "Reply added" })))
}
