use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::ReviewService;
use crate::infra::PgReviewRepository;

pub mod dto;
mod handlers;

type Service = ReviewService<PgReviewRepository>;

#[derive(Clone)]
pub struct ReviewState {
    service: Arc<Service>,
    jwt: JwtConfig,
    pub pool: PgPool,
}

impl std::ops::Deref for ReviewState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for ReviewState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgReviewRepository::new(pool.clone());
    let service = Arc::new(ReviewService::new(repo));

    let state = ReviewState {
        service,
        jwt,
        pool,
    };

    Router::new()
        .route("/", post(handlers::submit_review))
        .route("/public", post(handlers::public_submit_review))
        .route("/locations/{location_id}", get(handlers::list_reviews))
        .route(
            "/public/locations/{location_id}",
            get(handlers::public_list_reviews),
        )
        .route(
            "/locations/{location_id}/summary",
            get(handlers::get_summary),
        )
        .route("/{id}/reply", put(handlers::reply_to_review))
        .with_state(state)
}
