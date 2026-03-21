use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config::AppConfig;
use crate::state::AppState;
use crate::uploads;

/// Health check response
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

/// Readiness probe - checks database connectivity
async fn ready(state: axum::extract::State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "status": "ready" }))),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "status": "unavailable", "reason": "database" })),
        ),
    }
}

pub fn build(state: AppState, config: &AppConfig) -> Router {
    let module_routes = Router::new()
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
        )
        .nest(
            "/api/v1/bookings",
            washco_booking::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/reviews",
            washco_review::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/notifications",
            washco_notification::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/pricing",
            washco_pricing::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/promotions",
            washco_promotion::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/payments",
            washco_payment::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/staff",
            washco_staff::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/customers",
            washco_customer::api::routes(state.db.clone(), state.jwt.clone()),
        )
        .nest(
            "/api/v1/inventory",
            washco_inventory::api::routes(state.db.clone(), state.jwt.clone()),
        );

    let upload_routes = Router::new()
        .route("/api/v1/uploads/presign", post(uploads::presign_upload))
        .route("/api/v1/files/{*key}", get(uploads::get_file))
        .with_state(state.clone());

    let health_routes = Router::new()
        .route("/health", get(health))
        .route("/live", get(health))
        .route("/ready", get(ready))
        .with_state(state);

    // CORS - restrict to configured origins
    let cors = build_cors(config);

    // Security headers
    let security_headers = tower_http::set_header::SetResponseHeaderLayer::overriding(
        axum::http::header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    let x_frame = tower_http::set_header::SetResponseHeaderLayer::overriding(
        axum::http::header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );

    Router::new()
        .merge(module_routes)
        .merge(upload_routes)
        .merge(health_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(security_headers)
        .layer(x_frame)
}

fn build_cors(config: &AppConfig) -> CorsLayer {
    if config.allowed_origins.is_empty()
        || config.allowed_origins.iter().any(|o| o == "*")
    {
        return CorsLayer::permissive();
    }

    let origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600))
}
