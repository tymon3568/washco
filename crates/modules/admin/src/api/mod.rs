use std::sync::Arc;

use axum::{
    Router,
    routing::{get, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::AdminService;
use crate::infra::PgAdminRepository;

pub mod dto;
mod handlers;

type Service = AdminService<PgAdminRepository>;

#[derive(Clone)]
pub struct AdminState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for AdminState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for AdminState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgAdminRepository::new(pool);
    let service = Arc::new(AdminService::new(repo));
    let state = AdminState { service, jwt };

    Router::new()
        .route("/locations", get(handlers::list_locations))
        .route("/locations/{id}", get(handlers::get_location))
        .route("/locations/{id}/approve", put(handlers::approve_location))
        .route("/locations/{id}/suspend", put(handlers::suspend_location))
        .route("/metrics", get(handlers::platform_metrics))
        .route("/actions", get(handlers::list_actions))
        .route("/tiers", get(handlers::list_tiers))
        .with_state(state)
}
