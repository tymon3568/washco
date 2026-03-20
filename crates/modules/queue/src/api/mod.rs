use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::QueueService;
use crate::infra::PgQueueRepository;

pub mod dto;
mod handlers;

type Service = QueueService<PgQueueRepository>;

#[derive(Clone)]
pub struct QueueState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for QueueState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for QueueState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgQueueRepository::new(pool);
    let service = Arc::new(QueueService::new(repo));

    let state = QueueState { service, jwt };

    Router::new()
        .route("/locations/{location_id}", get(handlers::get_queue))
        .route("/locations/{location_id}/join", post(handlers::join))
        .route("/{id}/advance", put(handlers::advance))
        .route("/{id}/complete", put(handlers::complete))
        .route("/{id}/cancel", put(handlers::cancel))
        .with_state(state)
}
