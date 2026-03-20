use uuid::Uuid;

use crate::domain::Location;

pub trait LocationRepository: Send + Sync {
    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Location>, sqlx::Error>> + Send;

    fn find_by_slug(
        &self,
        tenant_id: Uuid,
        slug: &str,
    ) -> impl std::future::Future<Output = Result<Option<Location>, sqlx::Error>> + Send;

    fn find_by_tenant(
        &self,
        tenant_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<Location>, sqlx::Error>> + Send;

    fn create(
        &self,
        location: &Location,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;

    fn update(
        &self,
        location: &Location,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;

    fn soft_delete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;

    fn find_nearby(
        &self,
        lat: f64,
        lng: f64,
        radius_meters: f64,
    ) -> impl std::future::Future<Output = Result<Vec<(Location, f64)>, sqlx::Error>> + Send;
}
