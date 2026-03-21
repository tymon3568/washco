use std::sync::Arc;

use axum::{
    Router,
    routing::{get, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::LocationService;
use crate::infra::PgLocationRepository;

pub mod dto;
mod handlers;

type Service = LocationService<PgLocationRepository>;

#[derive(Clone)]
pub struct LocationState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for LocationState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for LocationState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgLocationRepository::new(pool);
    let service = Arc::new(LocationService::new(repo));

    let state = LocationState { service, jwt };

    Router::new()
        .route("/", get(handlers::list).post(handlers::create))
        .route(
            "/{id}",
            get(handlers::get_by_id)
                .put(handlers::update)
                .delete(handlers::delete),
        )
        .route("/nearby", get(handlers::nearby))
        .route(
            "/{id}/hours",
            get(handlers::get_operating_hours).put(handlers::set_operating_hours),
        )
        .route(
            "/{id}/bays",
            get(handlers::list_bays).post(handlers::create_bay),
        )
        .route(
            "/bays/{bay_id}",
            put(handlers::update_bay).delete(handlers::delete_bay),
        )
        .with_state(state)
}
