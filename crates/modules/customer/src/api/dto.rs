use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Customer, Membership, ServiceHistoryEntry, Vehicle};

// -- Query params --

#[derive(Debug, Deserialize)]
pub struct CustomerListQuery {
    pub segment: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Deserialize)]
pub struct ReminderQuery {
    pub as_of: Option<NaiveDate>,
}

// -- Customer DTOs --

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub phone: String,
    pub name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
    pub loyalty_points: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CustomerResponse {
    pub id: Uuid,
    pub phone: String,
    pub name: String,
    pub email: Option<String>,
    pub segment: String,
    pub total_visits: i32,
    pub total_spent: i64,
    pub last_visit_at: Option<DateTime<Utc>>,
    pub loyalty_points: i32,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Customer> for CustomerResponse {
    fn from(c: Customer) -> Self {
        Self {
            id: c.id,
            phone: c.phone,
            name: c.name,
            email: c.email,
            segment: c.segment.to_string(),
            total_visits: c.total_visits,
            total_spent: c.total_spent,
            last_visit_at: c.last_visit_at,
            loyalty_points: c.loyalty_points,
            notes: c.notes,
            tags: c.tags,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

// -- Vehicle DTOs --

#[derive(Debug, Deserialize)]
pub struct AddVehicleRequest {
    pub plate_number: Option<String>,
    pub vehicle_type: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub year: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VehicleResponse {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub plate_number: Option<String>,
    pub vehicle_type: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub year: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Vehicle> for VehicleResponse {
    fn from(v: Vehicle) -> Self {
        Self {
            id: v.id,
            customer_id: v.customer_id,
            plate_number: v.plate_number,
            vehicle_type: v.vehicle_type.to_string(),
            brand: v.brand,
            model: v.model,
            color: v.color,
            year: v.year,
            notes: v.notes,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

// -- Service history DTOs --

#[derive(Debug, Deserialize)]
pub struct AddServiceRecordRequest {
    pub vehicle_id: Uuid,
    pub location_id: Uuid,
    pub payment_id: Option<Uuid>,
    pub service_id: Uuid,
    pub service_name: String,
    pub amount_paid: i64,
    pub staff_name: Option<String>,
    pub notes: Option<String>,
    pub next_recommended_date: Option<NaiveDate>,
    pub next_recommended_service: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServiceHistoryResponse {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub customer_id: Uuid,
    pub location_id: Uuid,
    pub payment_id: Option<Uuid>,
    pub service_id: Uuid,
    pub service_name: String,
    pub amount_paid: i64,
    pub staff_name: Option<String>,
    pub notes: Option<String>,
    pub next_recommended_date: Option<NaiveDate>,
    pub next_recommended_service: Option<String>,
    pub serviced_at: DateTime<Utc>,
}

impl From<ServiceHistoryEntry> for ServiceHistoryResponse {
    fn from(e: ServiceHistoryEntry) -> Self {
        Self {
            id: e.id,
            vehicle_id: e.vehicle_id,
            customer_id: e.customer_id,
            location_id: e.location_id,
            payment_id: e.payment_id,
            service_id: e.service_id,
            service_name: e.service_name,
            amount_paid: e.amount_paid,
            staff_name: e.staff_name,
            notes: e.notes,
            next_recommended_date: e.next_recommended_date,
            next_recommended_service: e.next_recommended_service,
            serviced_at: e.serviced_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ReminderResponse {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub customer_id: Uuid,
    pub service_name: String,
    pub next_recommended_date: Option<NaiveDate>,
    pub next_recommended_service: Option<String>,
    pub serviced_at: DateTime<Utc>,
}

impl From<ServiceHistoryEntry> for ReminderResponse {
    fn from(e: ServiceHistoryEntry) -> Self {
        Self {
            id: e.id,
            vehicle_id: e.vehicle_id,
            customer_id: e.customer_id,
            service_name: e.service_name,
            next_recommended_date: e.next_recommended_date,
            next_recommended_service: e.next_recommended_service,
            serviced_at: e.serviced_at,
        }
    }
}

// -- Membership DTOs --

#[derive(Debug, Deserialize)]
pub struct CreateMembershipRequest {
    pub plan_name: String,
    pub plan_type: String,
    pub total_uses: Option<i32>,
    pub price_paid: i64,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub plan_name: String,
    pub plan_type: String,
    pub total_uses: Option<i32>,
    pub used_count: i32,
    pub price_paid: i64,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<Membership> for MembershipResponse {
    fn from(m: Membership) -> Self {
        Self {
            id: m.id,
            customer_id: m.customer_id,
            plan_name: m.plan_name,
            plan_type: m.plan_type.to_string(),
            total_uses: m.total_uses,
            used_count: m.used_count,
            price_paid: m.price_paid,
            valid_from: m.valid_from,
            valid_to: m.valid_to,
            status: m.status.to_string(),
            created_at: m.created_at,
        }
    }
}

// -- Generic message --

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
