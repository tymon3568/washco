use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

pub fn build(state: AppState) -> Router {
    let api = Router::new()
        .nest(
            "/api/v1/auth",
            washco_identity::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/locations",
            washco_location::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/catalog",
            washco_catalog::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/queue",
            washco_queue::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/analytics",
            washco_analytics::api::routes(state.db.clone(), state.jwt.clone()),
        );

    Router::new()
        .merge(api)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
