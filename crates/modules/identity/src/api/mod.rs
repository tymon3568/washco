use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::IdentityService;
use crate::infra::{InMemoryOtpStore, PgUserRepository};

pub mod dto;
mod handlers;

type Service = IdentityService<PgUserRepository, InMemoryOtpStore>;

#[derive(Clone)]
pub struct IdentityState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for IdentityState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for IdentityState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgUserRepository::new(pool);
    let otp_store = InMemoryOtpStore::new();
    let service = Arc::new(IdentityService::new(repo, otp_store, jwt.clone()));

    let state = IdentityState { service, jwt };

    Router::new()
        .route("/register", post(handlers::register))
        .route("/otp/request", post(handlers::request_otp))
        .route("/otp/verify", post(handlers::verify_otp))
        .route("/refresh", post(handlers::refresh))
        .route("/me", get(handlers::me))
        .with_state(state)
}
