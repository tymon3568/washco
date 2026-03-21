use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::PaymentService;
use crate::infra::PgPaymentRepository;

pub mod dto;
mod handlers;

type Service = PaymentService<PgPaymentRepository>;

#[derive(Clone)]
pub struct PaymentState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for PaymentState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for PaymentState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgPaymentRepository::new(pool);
    let service = Arc::new(PaymentService::new(repo));

    let state = PaymentState { service, jwt };

    Router::new()
        .route("/", post(handlers::create))
        .route("/{id}", get(handlers::get_by_id))
        .route("/{id}/complete", put(handlers::complete))
        .route("/locations/{location_id}", get(handlers::list_by_location))
        .route(
            "/locations/{location_id}/revenue",
            get(handlers::daily_revenue),
        )
        .route("/staff/{staff_id}/earnings", get(handlers::staff_earnings))
        .with_state(state)
}
