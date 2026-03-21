use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::domain::Booking;

pub trait BookingRepository: Send + Sync {
    fn create(
        &self,
        booking: &Booking,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Booking>>> + Send;

    fn update_status(
        &self,
        booking: &Booking,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Booking>>> + Send;

    fn list_by_phone(
        &self,
        tenant_id: Uuid,
        phone: &str,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Booking>>> + Send;

    fn count_at_slot(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
        time: NaiveTime,
    ) -> impl std::future::Future<Output = anyhow::Result<i64>> + Send;
}
