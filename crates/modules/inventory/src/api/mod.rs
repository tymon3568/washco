use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::InventoryService;
use crate::infra::PgInventoryRepository;

pub mod dto;
mod handlers;

type Service = InventoryService<PgInventoryRepository>;

#[derive(Clone)]
pub struct InventoryState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for InventoryState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for InventoryState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgInventoryRepository::new(pool);
    let service = Arc::new(InventoryService::new(repo));

    let state = InventoryState { service, jwt };

    Router::new()
        // Materials
        .route(
            "/locations/{location_id}/materials",
            get(handlers::list_materials).post(handlers::create_material),
        )
        .route(
            "/materials/{id}",
            get(handlers::get_material).put(handlers::update_material),
        )
        // Norms
        .route("/norms", axum::routing::post(handlers::set_norm))
        .route("/services/{service_id}/norms", get(handlers::list_norms))
        .route("/norms/{id}", axum::routing::delete(handlers::delete_norm))
        // Transactions
        .route(
            "/transactions",
            axum::routing::post(handlers::record_transaction),
        )
        .route(
            "/materials/{material_id}/transactions",
            get(handlers::list_transactions),
        )
        // Reports
        .route(
            "/locations/{location_id}/low-stock",
            get(handlers::low_stock_alerts),
        )
        .route(
            "/locations/{location_id}/variance",
            get(handlers::material_variance),
        )
        .with_state(state)
}
