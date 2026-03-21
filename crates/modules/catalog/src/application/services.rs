use uuid::Uuid;
use washco_shared::AppError;
use washco_shared::money::Money;

use crate::domain::{CatalogError, Service, VehicleType};

use super::ports::ServiceRepository;

pub struct CatalogService<R> {
    repo: R,
}

pub struct CreateServiceInput {
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub vehicle_type: String,
    pub base_price: i64,
    pub duration_minutes: i32,
}

pub struct UpdateServiceInput {
    pub tenant_id: Uuid,
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_price: Option<i64>,
    pub duration_minutes: Option<i32>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

impl<R: ServiceRepository> CatalogService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list_services(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<Service>, AppError> {
        self.repo
            .find_by_location(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn get_service(&self, tenant_id: Uuid, id: Uuid) -> Result<Service, AppError> {
        self.repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| CatalogError::ServiceNotFound.into())
    }

    pub async fn create_service(&self, input: CreateServiceInput) -> Result<Service, AppError> {
        let vehicle_type = VehicleType::from_str(&input.vehicle_type)
            .ok_or_else(|| CatalogError::InvalidVehicleType(input.vehicle_type.clone()))?;

        let service = Service::new(
            input.tenant_id,
            input.location_id,
            input.name,
            input.description,
            vehicle_type,
            Money::new(input.base_price),
            input.duration_minutes,
        );

        self.repo
            .create(&service)
            .await
            .map_err(AppError::Internal)?;

        Ok(service)
    }

    pub async fn update_service(&self, input: UpdateServiceInput) -> Result<Service, AppError> {
        let mut service = self
            .repo
            .find_by_id(input.tenant_id, input.id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(AppError::from(CatalogError::ServiceNotFound))?;

        if let Some(name) = input.name {
            service.name = name;
        }
        if let Some(description) = input.description {
            service.description = Some(description);
        }
        if let Some(base_price) = input.base_price {
            service.base_price = Money::new(base_price);
        }
        if let Some(duration_minutes) = input.duration_minutes {
            service.duration_minutes = duration_minutes;
        }
        if let Some(is_active) = input.is_active {
            service.is_active = is_active;
        }
        if let Some(sort_order) = input.sort_order {
            service.sort_order = sort_order;
        }

        service.updated_at = chrono::Utc::now();

        self.repo
            .update(&service)
            .await
            .map_err(AppError::Internal)?;

        Ok(service)
    }

    pub async fn delete_service(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
        // Verify existence first
        self.repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(AppError::from(CatalogError::ServiceNotFound))?;

        self.repo
            .soft_delete(tenant_id, id)
            .await
            .map_err(AppError::Internal)?;

        Ok(())
    }
}
