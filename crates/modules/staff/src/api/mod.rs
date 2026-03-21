use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::StaffService;
use crate::infra::PgStaffRepository;

pub mod dto;
mod handlers;

type Service = StaffService<PgStaffRepository>;

#[derive(Clone)]
pub struct StaffState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for StaffState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for StaffState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgStaffRepository::new(pool);
    let service = Arc::new(StaffService::new(repo));

    let state = StaffState { service, jwt };

    Router::new()
        // Staff profiles
        .route(
            "/locations/{location_id}/staff",
            get(handlers::list_staff).post(handlers::create_staff),
        )
        .route(
            "/staff/{id}",
            get(handlers::get_staff).put(handlers::update_staff),
        )
        // Shifts
        .route(
            "/locations/{location_id}/shifts",
            get(handlers::list_shifts).post(handlers::create_shift),
        )
        .route("/shifts/{id}/status", post(handlers::update_shift_status))
        // Commission rules
        .route(
            "/locations/{location_id}/commission-rules",
            get(handlers::list_commission_rules).post(handlers::create_commission_rule),
        )
        // Commission entries + summary
        .route("/commissions", post(handlers::record_commission))
        .route(
            "/staff/{staff_id}/commission-summary",
            get(handlers::commission_summary),
        )
        .with_state(state)
}
