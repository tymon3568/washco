use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::BookingService;
use crate::infra::PgBookingRepository;

pub mod dto;
mod handlers;

type Service = BookingService<PgBookingRepository>;

#[derive(Clone)]
pub struct BookingState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for BookingState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for BookingState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgBookingRepository::new(pool);
    let service = Arc::new(BookingService::new(repo));

    let state = BookingState { service, jwt };

    Router::new()
        .route(
            "/locations/{location_id}",
            post(handlers::create_booking).get(handlers::list_by_location),
        )
        .route("/phone/{phone}", get(handlers::list_by_phone))
        .route("/{id}/confirm", put(handlers::confirm))
        .route("/{id}/complete", put(handlers::complete))
        .route("/{id}/cancel", put(handlers::cancel))
        .with_state(state)
}
