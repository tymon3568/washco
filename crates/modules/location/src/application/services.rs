use chrono::Utc;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{Bay, Location, LocationError, LocationStatus, OperatingHours, QueueMode};

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

pub struct OperatingHoursInput {
    pub day_of_week: i16,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
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
        if input.name.trim().is_empty() {
            return Err(AppError::Validation {
                message: "Location name is required".into(),
            });
        }

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

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Location, AppError> {
        self.repo
            .find_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| LocationError::NotFound.into())
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Location>, AppError> {
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
            if new_slug != location.slug
                && let Some(existing) = self.repo.find_by_slug(tenant_id, &new_slug).await?
                && existing.id != location.id
            {
                return Err(LocationError::SlugConflict.into());
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
            if !(-90.0..=90.0).contains(&lat) {
                return Err(LocationError::InvalidCoordinates.into());
            }
            location.latitude = lat;
        }
        if let Some(lng) = input.longitude {
            if !(-180.0..=180.0).contains(&lng) {
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

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
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
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lng) {
            return Err(LocationError::InvalidCoordinates.into());
        }
        if radius_meters < 0.0 {
            return Err(AppError::Validation {
                message: "Radius must not be negative".into(),
            });
        }
        let results = self.repo.find_nearby(lat, lng, radius_meters).await?;
        Ok(results)
    }

    pub async fn get_operating_hours(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<OperatingHours>, AppError> {
        // Verify location exists and belongs to tenant
        self.repo
            .find_by_id(tenant_id, location_id)
            .await?
            .ok_or(LocationError::NotFound)?;

        let hours = self
            .repo
            .get_operating_hours(tenant_id, location_id)
            .await?;
        Ok(hours)
    }

    pub async fn set_operating_hours(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        inputs: Vec<OperatingHoursInput>,
    ) -> Result<Vec<OperatingHours>, AppError> {
        // Verify location exists and belongs to tenant
        self.repo
            .find_by_id(tenant_id, location_id)
            .await?
            .ok_or(LocationError::NotFound)?;

        let mut hours = Vec::with_capacity(inputs.len());
        for input in &inputs {
            if input.day_of_week < 0 || input.day_of_week > 6 {
                return Err(AppError::Validation {
                    message: format!("Invalid day_of_week: {}", input.day_of_week),
                });
            }
            let open_time =
                chrono::NaiveTime::parse_from_str(&input.open_time, "%H:%M").map_err(|_| {
                    AppError::Validation {
                        message: format!("Invalid open_time format: {}", input.open_time),
                    }
                })?;
            let close_time = chrono::NaiveTime::parse_from_str(&input.close_time, "%H:%M")
                .map_err(|_| AppError::Validation {
                    message: format!("Invalid close_time format: {}", input.close_time),
                })?;
            hours.push(OperatingHours::new(
                location_id,
                tenant_id,
                input.day_of_week,
                open_time,
                close_time,
                input.is_closed,
            ));
        }

        self.repo
            .set_operating_hours(tenant_id, location_id, &hours)
            .await?;

        Ok(hours)
    }

    pub async fn list_bays(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<Bay>, AppError> {
        // Verify location exists and belongs to tenant
        self.repo
            .find_by_id(tenant_id, location_id)
            .await?
            .ok_or(LocationError::NotFound)?;

        let bays = self.repo.list_bays(tenant_id, location_id).await?;
        Ok(bays)
    }

    pub async fn create_bay(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        name: String,
    ) -> Result<Bay, AppError> {
        // Verify location exists and belongs to tenant
        self.repo
            .find_by_id(tenant_id, location_id)
            .await?
            .ok_or(LocationError::NotFound)?;

        let bay = Bay::new(location_id, tenant_id, name);
        self.repo.create_bay(&bay).await?;
        Ok(bay)
    }

    pub async fn update_bay(
        &self,
        tenant_id: Uuid,
        bay_id: Uuid,
        name: Option<String>,
        is_active: Option<bool>,
    ) -> Result<Bay, AppError> {
        // Find existing bay by listing all bays for tenant locations
        // We need to find the bay - search across all locations for this tenant
        let locations = self.repo.find_by_tenant(tenant_id).await?;
        let mut found_bay: Option<Bay> = None;
        for loc in &locations {
            let bays = self.repo.list_bays(tenant_id, loc.id).await?;
            for bay in bays {
                if bay.id == bay_id {
                    found_bay = Some(bay);
                    break;
                }
            }
            if found_bay.is_some() {
                break;
            }
        }

        let mut bay = found_bay.ok_or(LocationError::NotFound)?;

        if let Some(n) = name {
            bay.name = n;
        }
        if let Some(active) = is_active {
            bay.is_active = active;
        }

        self.repo.update_bay(&bay).await?;
        Ok(bay)
    }

    pub async fn delete_bay(&self, tenant_id: Uuid, bay_id: Uuid) -> Result<(), AppError> {
        self.repo.delete_bay(tenant_id, bay_id).await?;
        Ok(())
    }
}
