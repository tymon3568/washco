use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::AnalyticsService;
use crate::infra::PgAnalyticsRepository;

pub mod dto;
mod handlers;

type Service = AnalyticsService<PgAnalyticsRepository>;

#[derive(Clone)]
pub struct AnalyticsState {
    service: Arc<Service>,
    jwt: JwtConfig,
    pub pool: PgPool,
}

impl std::ops::Deref for AnalyticsState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for AnalyticsState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgAnalyticsRepository::new(pool.clone());
    let service = Arc::new(AnalyticsService::new(repo));

    let state = AnalyticsState { service, jwt, pool };

    Router::new()
        .route(
            "/locations/{location_id}/daily",
            get(handlers::daily_summary),
        )
        .route(
            "/locations/{location_id}/utilization",
            get(handlers::utilization),
        )
        .route(
            "/locations/{location_id}/services",
            get(handlers::service_breakdown),
        )
        .route("/locations/{location_id}/trend", get(handlers::trend))
        .route(
            "/locations/{location_id}/period",
            get(handlers::period_summary),
        )
        .route("/compare", get(handlers::compare_locations))
        .with_state(state)
}
