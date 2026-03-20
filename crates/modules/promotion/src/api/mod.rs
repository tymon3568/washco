use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::PromotionService;
use crate::infra::PgPromotionRepository;

pub mod dto;
mod handlers;

type Service = PromotionService<PgPromotionRepository>;

#[derive(Clone)]
pub struct PromotionState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for PromotionState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for PromotionState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgPromotionRepository::new(pool);
    let service = Arc::new(PromotionService::new(repo));

    let state = PromotionState { service, jwt };

    Router::new()
        .route("/", post(handlers::create_promotion))
        .route("/", get(handlers::list_promotions))
        .route("/{id}", put(handlers::update_promotion))
        .route("/{id}", delete(handlers::delete_promotion))
        .route("/validate", post(handlers::validate_code))
        .route("/redeem", post(handlers::redeem))
        .with_state(state)
}
