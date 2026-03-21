use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{Customer, Membership, ServiceHistoryEntry, Vehicle};

use super::services::{
    AddServiceRecordInput, AddVehicleInput, CreateCustomerInput, CreateMembershipInput,
    UpdateCustomerInput,
};

pub trait CustomerRepository: Send + Sync {
    // Customers
    fn create_customer(
        &self,
        tenant_id: Uuid,
        input: &CreateCustomerInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Customer>> + Send;

    fn find_by_phone(
        &self,
        tenant_id: Uuid,
        phone: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Customer>>> + Send;

    fn get_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Customer>> + Send;

    fn list_customers(
        &self,
        tenant_id: Uuid,
        segment: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Customer>>> + Send;

    fn update_customer(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: &UpdateCustomerInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Customer>> + Send;

    fn update_visit_stats(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        amount: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    // Vehicles
    fn add_vehicle(
        &self,
        tenant_id: Uuid,
        input: &AddVehicleInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Vehicle>> + Send;

    fn list_vehicles(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Vehicle>>> + Send;

    fn find_by_plate(
        &self,
        tenant_id: Uuid,
        plate: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Vehicle>>> + Send;

    // Service history
    fn add_service_record(
        &self,
        tenant_id: Uuid,
        input: &AddServiceRecordInput,
    ) -> impl std::future::Future<Output = anyhow::Result<ServiceHistoryEntry>> + Send;

    fn vehicle_history(
        &self,
        tenant_id: Uuid,
        vehicle_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<ServiceHistoryEntry>>> + Send;

    fn due_reminders(
        &self,
        tenant_id: Uuid,
        as_of: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<ServiceHistoryEntry>>> + Send;

    // Memberships
    fn create_membership(
        &self,
        tenant_id: Uuid,
        input: &CreateMembershipInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Membership>> + Send;

    fn list_memberships(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Membership>>> + Send;

    fn use_membership(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Membership>> + Send;
}
