use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{
    InventoryTransaction, LowStockAlert, Material, MaterialNorm, MaterialVariance,
};

use super::services::{
    CreateMaterialInput, RecordTransactionInput, SetNormInput, UpdateMaterialInput,
};

pub trait InventoryRepository: Send + Sync {
    // Materials
    fn create_material(
        &self,
        tenant_id: Uuid,
        input: &CreateMaterialInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Material>> + Send;

    fn get_material(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Material>>> + Send;

    fn list_materials(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Material>>> + Send;

    fn update_material(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: &UpdateMaterialInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Material>> + Send;

    // Norms
    fn set_norm(
        &self,
        tenant_id: Uuid,
        input: &SetNormInput,
    ) -> impl std::future::Future<Output = anyhow::Result<MaterialNorm>> + Send;

    fn list_norms(
        &self,
        tenant_id: Uuid,
        service_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<MaterialNorm>>> + Send;

    fn delete_norm(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    // Transactions
    fn record_transaction(
        &self,
        tenant_id: Uuid,
        input: &RecordTransactionInput,
    ) -> impl std::future::Future<Output = anyhow::Result<InventoryTransaction>> + Send;

    fn list_transactions(
        &self,
        tenant_id: Uuid,
        material_id: Uuid,
        limit: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<InventoryTransaction>>> + Send;

    // Reports
    fn low_stock_alerts(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<LowStockAlert>>> + Send;

    fn material_variance(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<MaterialVariance>>> + Send;
}
