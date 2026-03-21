use chrono::NaiveDate;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{
    Customer, CustomerError, CustomerSegment, Membership, MembershipStatus, MembershipType,
    ServiceHistoryEntry, Vehicle,
};

use super::ports::CustomerRepository;

pub struct CustomerService<R> {
    repo: R,
}

// -- Application input types --

pub struct CreateCustomerInput {
    pub phone: String,
    pub name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
}

pub struct UpdateCustomerInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
    pub loyalty_points: Option<i32>,
}

pub struct AddVehicleInput {
    pub customer_id: Uuid,
    pub plate_number: Option<String>,
    pub vehicle_type: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub year: Option<i32>,
    pub notes: Option<String>,
}

pub struct AddServiceRecordInput {
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
}

pub struct CreateMembershipInput {
    pub customer_id: Uuid,
    pub plan_name: String,
    pub plan_type: String,
    pub total_uses: Option<i32>,
    pub price_paid: i64,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
}

impl<R: CustomerRepository> CustomerService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    // -- Customers --

    /// Find by phone, create if not exists.
    pub async fn get_or_create_customer(
        &self,
        tenant_id: Uuid,
        phone: &str,
        name: &str,
    ) -> Result<Customer, AppError> {
        if let Some(existing) = self
            .repo
            .find_by_phone(tenant_id, phone)
            .await
            .map_err(AppError::Internal)?
        {
            return Ok(existing);
        }

        let input = CreateCustomerInput {
            phone: phone.to_string(),
            name: name.to_string(),
            email: None,
            notes: None,
            tags: Vec::new(),
        };

        self.repo
            .create_customer(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn create_customer(
        &self,
        tenant_id: Uuid,
        input: CreateCustomerInput,
    ) -> Result<Customer, AppError> {
        // Check duplicate phone
        if let Some(_existing) = self
            .repo
            .find_by_phone(tenant_id, &input.phone)
            .await
            .map_err(AppError::Internal)?
        {
            return Err(CustomerError::AlreadyExists.into());
        }

        self.repo
            .create_customer(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn find_by_phone(
        &self,
        tenant_id: Uuid,
        phone: &str,
    ) -> Result<Option<Customer>, AppError> {
        self.repo
            .find_by_phone(tenant_id, phone)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn get_customer(&self, tenant_id: Uuid, id: Uuid) -> Result<Customer, AppError> {
        self.repo
            .get_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_customers(
        &self,
        tenant_id: Uuid,
        segment: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Customer>, AppError> {
        self.repo
            .list_customers(tenant_id, segment, limit, offset)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_customer(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateCustomerInput,
    ) -> Result<Customer, AppError> {
        self.repo
            .update_customer(tenant_id, id, &input)
            .await
            .map_err(AppError::Internal)
    }

    // -- Vehicles --

    pub async fn add_vehicle(
        &self,
        tenant_id: Uuid,
        input: AddVehicleInput,
    ) -> Result<Vehicle, AppError> {
        self.repo
            .add_vehicle(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_vehicles(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> Result<Vec<Vehicle>, AppError> {
        self.repo
            .list_vehicles(tenant_id, customer_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn find_by_plate(
        &self,
        tenant_id: Uuid,
        plate: &str,
    ) -> Result<Option<Vehicle>, AppError> {
        self.repo
            .find_by_plate(tenant_id, plate)
            .await
            .map_err(AppError::Internal)
    }

    // -- Service history --

    pub async fn add_service_record(
        &self,
        tenant_id: Uuid,
        input: AddServiceRecordInput,
    ) -> Result<ServiceHistoryEntry, AppError> {
        let record = self
            .repo
            .add_service_record(tenant_id, &input)
            .await
            .map_err(AppError::Internal)?;

        // Update customer visit stats
        let _ = self
            .repo
            .update_visit_stats(tenant_id, input.customer_id, input.amount_paid)
            .await;

        Ok(record)
    }

    pub async fn vehicle_history(
        &self,
        tenant_id: Uuid,
        vehicle_id: Uuid,
    ) -> Result<Vec<ServiceHistoryEntry>, AppError> {
        self.repo
            .vehicle_history(tenant_id, vehicle_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn due_reminders(
        &self,
        tenant_id: Uuid,
        as_of: NaiveDate,
    ) -> Result<Vec<ServiceHistoryEntry>, AppError> {
        self.repo
            .due_reminders(tenant_id, as_of)
            .await
            .map_err(AppError::Internal)
    }

    // -- Memberships --

    pub async fn create_membership(
        &self,
        tenant_id: Uuid,
        input: CreateMembershipInput,
    ) -> Result<Membership, AppError> {
        // Validate membership type
        let _plan_type = MembershipType::from_str(&input.plan_type)
            .ok_or_else(|| CustomerError::InvalidMembershipType(input.plan_type.clone()))?;

        self.repo
            .create_membership(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_memberships(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> Result<Vec<Membership>, AppError> {
        self.repo
            .list_memberships(tenant_id, customer_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn use_membership(&self, tenant_id: Uuid, id: Uuid) -> Result<Membership, AppError> {
        let updated = self
            .repo
            .use_membership(tenant_id, id)
            .await
            .map_err(AppError::Internal)?;

        if updated.status != MembershipStatus::Active {
            return Err(CustomerError::MembershipExpired.into());
        }

        Ok(updated)
    }

    // -- Segment helper --

    pub fn compute_segment(
        total_visits: i32,
        last_visit_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> CustomerSegment {
        if let Some(last_visit) = last_visit_at {
            let days_since = (chrono::Utc::now() - last_visit).num_days();
            if days_since > 90 {
                return CustomerSegment::Dormant;
            }
        }
        match total_visits {
            0..=1 => CustomerSegment::New,
            2..=9 => CustomerSegment::Regular,
            _ => CustomerSegment::Vip,
        }
    }
}
