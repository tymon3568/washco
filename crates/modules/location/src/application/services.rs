use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{Location, LocationError, LocationStatus, QueueMode};

use super::ports::LocationRepository;

pub struct LocationService<R> {
    repo: R,
}

pub struct CreateLocationInput {
    pub name: String,
    pub phone: Option<String>,
    pub address: String,
    pub district: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub bay_count: i16,
    pub queue_mode: String,
    pub amenities: Option<serde_json::Value>,
}

pub struct UpdateLocationInput {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub bay_count: Option<i16>,
    pub queue_mode: Option<String>,
    pub status: Option<String>,
    pub amenities: Option<serde_json::Value>,
}

impl<R: LocationRepository> LocationService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        input: CreateLocationInput,
    ) -> Result<Location, AppError> {
        let queue_mode = QueueMode::from_str(&input.queue_mode)
            .ok_or_else(|| LocationError::InvalidQueueMode(input.queue_mode.clone()))?;

        if input.latitude < -90.0
            || input.latitude > 90.0
            || input.longitude < -180.0
            || input.longitude > 180.0
        {
            return Err(LocationError::InvalidCoordinates.into());
        }

        let location = Location::new(
            tenant_id,
            input.name,
            input.phone,
            input.address,
            input.district,
            input.city,
            input.latitude,
            input.longitude,
            input.bay_count,
            queue_mode,
            input.amenities.unwrap_or(serde_json::Value::Array(vec![])),
        );

        // Check slug uniqueness within tenant
        if self
            .repo
            .find_by_slug(tenant_id, &location.slug)
            .await?
            .is_some()
        {
            return Err(LocationError::SlugConflict.into());
        }

        self.repo.create(&location).await?;

        Ok(location)
    }

    pub async fn get_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<Location, AppError> {
        self.repo
            .find_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| LocationError::NotFound.into())
    }

    pub async fn list_by_tenant(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<Location>, AppError> {
        let locations = self.repo.find_by_tenant(tenant_id).await?;
        Ok(locations)
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateLocationInput,
    ) -> Result<Location, AppError> {
        let mut location = self
            .repo
            .find_by_id(tenant_id, id)
            .await?
            .ok_or(LocationError::NotFound)?;

        if let Some(name) = input.name {
            let new_slug = Location::slugify(&name);
            // Check slug uniqueness if it changed
            if new_slug != location.slug {
                if let Some(existing) = self.repo.find_by_slug(tenant_id, &new_slug).await? {
                    if existing.id != location.id {
                        return Err(LocationError::SlugConflict.into());
                    }
                }
            }
            location.name = name;
            location.slug = new_slug;
        }

        if let Some(phone) = input.phone {
            location.phone = Some(phone);
        }
        if let Some(address) = input.address {
            location.address = address;
        }
        if let Some(district) = input.district {
            location.district = district;
        }
        if let Some(city) = input.city {
            location.city = city;
        }
        if let Some(lat) = input.latitude {
            if lat < -90.0 || lat > 90.0 {
                return Err(LocationError::InvalidCoordinates.into());
            }
            location.latitude = lat;
        }
        if let Some(lng) = input.longitude {
            if lng < -180.0 || lng > 180.0 {
                return Err(LocationError::InvalidCoordinates.into());
            }
            location.longitude = lng;
        }
        if let Some(bay_count) = input.bay_count {
            location.bay_count = bay_count;
        }
        if let Some(queue_mode) = input.queue_mode {
            location.queue_mode = QueueMode::from_str(&queue_mode)
                .ok_or_else(|| LocationError::InvalidQueueMode(queue_mode))?;
        }
        if let Some(status) = input.status {
            location.status = LocationStatus::from_str(&status)
                .ok_or_else(|| LocationError::InvalidStatus(status))?;
        }
        if let Some(amenities) = input.amenities {
            location.amenities = amenities;
        }

        location.updated_at = Utc::now();
        self.repo.update(&location).await?;

        Ok(location)
    }

    pub async fn delete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<(), AppError> {
        // Verify it exists first
        self.repo
            .find_by_id(tenant_id, id)
            .await?
            .ok_or(LocationError::NotFound)?;

        self.repo.soft_delete(tenant_id, id).await?;
        Ok(())
    }

    pub async fn find_nearby(
        &self,
        lat: f64,
        lng: f64,
        radius_meters: f64,
    ) -> Result<Vec<(Location, f64)>, AppError> {
        if lat < -90.0 || lat > 90.0 || lng < -180.0 || lng > 180.0 {
            return Err(LocationError::InvalidCoordinates.into());
        }
        let results = self.repo.find_nearby(lat, lng, radius_meters).await?;
        Ok(results)
    }
}
