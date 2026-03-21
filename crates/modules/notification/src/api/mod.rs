use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::NotificationService;
use crate::infra::PgNotificationRepository;

pub mod dto;
mod handlers;

type Service = NotificationService<PgNotificationRepository>;

#[derive(Clone)]
pub struct NotificationState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for NotificationState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for NotificationState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgNotificationRepository::new(pool);
    let service = Arc::new(NotificationService::new(repo));

    let state = NotificationState { service, jwt };

    Router::new()
        .route("/templates", post(handlers::create_template))
        .route("/templates", get(handlers::list_templates))
        .route("/templates/{id}", put(handlers::update_template))
        .route("/templates/{id}", delete(handlers::delete_template))
        .route("/send", post(handlers::send_notification))
        .route("/", get(handlers::list_notifications))
        .with_state(state)
}
