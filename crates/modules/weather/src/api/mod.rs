use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::WeatherService;
use crate::infra::PgWeatherRepository;

pub mod dto;
mod handlers;

type Service = WeatherService<PgWeatherRepository>;

#[derive(Clone)]
pub struct WeatherState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for WeatherState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for WeatherState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgWeatherRepository::new(pool);
    let service = Arc::new(WeatherService::new(repo));

    let state = WeatherState { service, jwt };

    Router::new()
        .route("/triggers", post(handlers::create_trigger))
        .route("/triggers", get(handlers::list_triggers))
        .route("/triggers/{id}", put(handlers::update_trigger))
        .route("/triggers/{id}", delete(handlers::delete_trigger))
        .route("/data", post(handlers::report_weather))
        .route("/data/{city}", get(handlers::latest_weather))
        .route("/evaluate", post(handlers::evaluate))
        .with_state(state)
}
