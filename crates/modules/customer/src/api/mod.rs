use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::application::CustomerService;
use crate::infra::PgCustomerRepository;

pub mod dto;
mod handlers;

type Service = CustomerService<PgCustomerRepository>;

#[derive(Clone)]
pub struct CustomerState {
    service: Arc<Service>,
    jwt: JwtConfig,
}

impl std::ops::Deref for CustomerState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for CustomerState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgCustomerRepository::new(pool);
    let service = Arc::new(CustomerService::new(repo));

    let state = CustomerState { service, jwt };

    Router::new()
        // Customers
        .route(
            "/customers",
            post(handlers::create_customer).get(handlers::list_customers),
        )
        .route(
            "/customers/{id}",
            get(handlers::get_customer).put(handlers::update_customer),
        )
        .route("/customers/phone/{phone}", get(handlers::find_by_phone))
        // Vehicles
        .route(
            "/customers/{customer_id}/vehicles",
            post(handlers::add_vehicle).get(handlers::list_vehicles),
        )
        .route("/vehicles/plate/{plate}", get(handlers::find_by_plate))
        // Service history
        .route(
            "/customers/{customer_id}/service-history",
            post(handlers::add_service_record),
        )
        .route(
            "/vehicles/{vehicle_id}/history",
            get(handlers::vehicle_history),
        )
        .route("/reminders/due", get(handlers::due_reminders))
        // Memberships
        .route(
            "/customers/{customer_id}/memberships",
            post(handlers::create_membership).get(handlers::list_memberships),
        )
        .route("/memberships/{id}/use", post(handlers::use_membership))
        .with_state(state)
}
