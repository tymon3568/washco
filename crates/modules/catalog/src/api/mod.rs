use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::CatalogService;
use crate::infra::PgServiceRepository;

pub mod dto;
mod handlers;

type Service = CatalogService<PgServiceRepository>;

#[derive(Clone)]
pub struct CatalogState {
    service: Arc<Service>,
    jwt: JwtConfig,
    pub pool: PgPool,
}

impl std::ops::Deref for CatalogState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for CatalogState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgServiceRepository::new(pool.clone());
    let service = Arc::new(CatalogService::new(repo));

    let state = CatalogState {
        service,
        jwt,
        pool,
    };

    Router::new()
        .route(
            "/locations/{location_id}/services",
            get(handlers::list).post(handlers::create),
        )
        .route(
            "/public/locations/{location_id}/services",
            get(handlers::public_list),
        )
        .route(
            "/services/{id}",
            get(handlers::get_by_id)
                .put(handlers::update)
                .delete(handlers::delete),
        )
        .with_state(state)
}
