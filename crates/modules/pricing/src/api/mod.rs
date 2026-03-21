use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::PricingService;
use crate::infra::PgPricingRepository;

pub mod dto;
mod handlers;

type Service = PricingService<PgPricingRepository>;

#[derive(Clone)]
pub struct PricingState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for PricingState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for PricingState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgPricingRepository::new(pool);
    let service = Arc::new(PricingService::new(repo));

    let state = PricingState { service, jwt };

    Router::new()
        .route("/rules", post(handlers::create_rule))
        .route("/locations/{location_id}/rules", get(handlers::list_rules))
        .route("/rules/{id}", put(handlers::update_rule))
        .route("/rules/{id}", delete(handlers::delete_rule))
        .route("/calculate", post(handlers::calculate_price))
        .with_state(state)
}
