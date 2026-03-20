use chrono::{NaiveDate, NaiveTime, Utc};
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{Booking, BookingError, BookingStatus};

use super::ports::BookingRepository;

/// Maximum concurrent bookings per time slot per location.
const MAX_SLOT_CAPACITY: i64 = 4;

pub struct BookingService<R> {
    repo: R,
}

pub struct CreateBookingInput {
    pub service_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub vehicle_type: String,
    pub booking_date: NaiveDate,
    pub time_slot: NaiveTime,
    pub notes: Option<String>,
}

impl<R: BookingRepository> BookingService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_booking(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        input: CreateBookingInput,
    ) -> Result<Booking, AppError> {
        // Validate date is not in the past
        let today = Utc::now().date_naive();
        if input.booking_date < today {
            return Err(BookingError::PastBookingDate.into());
        }

        // Check slot capacity
        let count = self
            .repo
            .count_at_slot(tenant_id, location_id, input.booking_date, input.time_slot)
            .await
            .map_err(AppError::Internal)?;

        if count >= MAX_SLOT_CAPACITY {
            return Err(BookingError::TimeSlotConflict.into());
        }

        let now = Utc::now();
        let booking = Booking {
            id: Uuid::now_v7(),
            tenant_id,
            location_id,
            service_id: input.service_id,
            customer_name: input.customer_name,
            customer_phone: input.customer_phone,
            vehicle_type: input.vehicle_type,
            booking_date: input.booking_date,
            time_slot: input.time_slot,
            status: BookingStatus::Pending,
            notes: input.notes,
            created_at: now,
            updated_at: now,
            cancelled_at: None,
        };

        self.repo
            .create(&booking)
            .await
            .map_err(AppError::Internal)?;

        Ok(booking)
    }

    pub async fn confirm(&self, tenant_id: Uuid, id: Uuid) -> Result<Booking, AppError> {
        let mut booking = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(BookingError::NotFound)?;

        booking.confirm()?;

        self.repo
            .update_status(&booking)
            .await
            .map_err(AppError::Internal)?;

        Ok(booking)
    }

    pub async fn complete(&self, tenant_id: Uuid, id: Uuid) -> Result<Booking, AppError> {
        let mut booking = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(BookingError::NotFound)?;

        booking.complete()?;

        self.repo
            .update_status(&booking)
            .await
            .map_err(AppError::Internal)?;

        Ok(booking)
    }

    pub async fn cancel(&self, tenant_id: Uuid, id: Uuid) -> Result<Booking, AppError> {
        let mut booking = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(BookingError::NotFound)?;

        booking.cancel()?;

        self.repo
            .update_status(&booking)
            .await
            .map_err(AppError::Internal)?;

        Ok(booking)
    }

    pub async fn no_show(&self, tenant_id: Uuid, id: Uuid) -> Result<Booking, AppError> {
        let mut booking = self
            .repo
            .find_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(BookingError::NotFound)?;

        booking.no_show()?;

        self.repo
            .update_status(&booking)
            .await
            .map_err(AppError::Internal)?;

        Ok(booking)
    }

    pub async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Booking>, AppError> {
        self.repo
            .list_by_location(tenant_id, location_id, date)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_by_phone(
        &self,
        phone: &str,
        date: NaiveDate,
    ) -> Result<Vec<Booking>, AppError> {
        self.repo
            .list_by_phone(phone, date)
            .await
            .map_err(AppError::Internal)
    }
}
