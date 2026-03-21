use chrono::NaiveDate;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{
    InventoryError, InventoryTransaction, LowStockAlert, Material, MaterialNorm, MaterialVariance,
    TransactionType,
};

use super::ports::InventoryRepository;

pub struct InventoryService<R> {
    repo: R,
}

pub struct CreateMaterialInput {
    pub location_id: Uuid,
    pub name: String,
    pub category: String,
    pub unit: String,
    pub unit_cost: i64,
    pub current_stock: i64,
    pub min_stock: i64,
}

pub struct UpdateMaterialInput {
    pub name: Option<String>,
    pub category: Option<String>,
    pub unit: Option<String>,
    pub unit_cost: Option<i64>,
    pub min_stock: Option<i64>,
    pub is_active: Option<bool>,
}

pub struct SetNormInput {
    pub service_id: Uuid,
    pub material_id: Uuid,
    pub quantity_per_job: i64,
}

pub struct RecordTransactionInput {
    pub material_id: Uuid,
    pub transaction_type: String,
    pub quantity: i64,
    pub unit_cost: Option<i64>,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
}

pub struct RecordPurchaseInput {
    pub material_id: Uuid,
    pub quantity: i64,
    pub unit_cost: Option<i64>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
}

pub struct RecordUsageInput {
    pub material_id: Uuid,
    pub quantity: i64,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
}

impl<R: InventoryRepository> InventoryService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    // --- Materials ---

    pub async fn create_material(
        &self,
        tenant_id: Uuid,
        input: CreateMaterialInput,
    ) -> Result<Material, AppError> {
        self.repo
            .create_material(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn get_material(&self, tenant_id: Uuid, id: Uuid) -> Result<Material, AppError> {
        self.repo
            .get_material(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| InventoryError::NotFound.into())
    }

    pub async fn list_materials(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<Material>, AppError> {
        self.repo
            .list_materials(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_material(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateMaterialInput,
    ) -> Result<Material, AppError> {
        // Verify existence
        self.repo
            .get_material(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(AppError::from(InventoryError::NotFound))?;

        self.repo
            .update_material(tenant_id, id, &input)
            .await
            .map_err(AppError::Internal)
    }

    // --- Norms ---

    pub async fn set_norm(
        &self,
        tenant_id: Uuid,
        input: SetNormInput,
    ) -> Result<MaterialNorm, AppError> {
        if input.quantity_per_job <= 0 {
            return Err(InventoryError::InvalidQuantity(
                "quantity_per_job must be positive".to_string(),
            )
            .into());
        }
        self.repo
            .set_norm(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_norms(
        &self,
        tenant_id: Uuid,
        service_id: Uuid,
    ) -> Result<Vec<MaterialNorm>, AppError> {
        self.repo
            .list_norms(tenant_id, service_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn delete_norm(&self, tenant_id: Uuid, id: Uuid) -> Result<(), AppError> {
        self.repo
            .delete_norm(tenant_id, id)
            .await
            .map_err(AppError::Internal)
    }

    // --- Transactions ---

    pub async fn record_purchase(
        &self,
        tenant_id: Uuid,
        input: RecordPurchaseInput,
    ) -> Result<InventoryTransaction, AppError> {
        if input.quantity <= 0 {
            return Err(InventoryError::InvalidQuantity(
                "purchase quantity must be positive".to_string(),
            )
            .into());
        }

        let tx_input = RecordTransactionInput {
            material_id: input.material_id,
            transaction_type: "purchase".to_string(),
            quantity: input.quantity,
            unit_cost: input.unit_cost,
            reference_id: None,
            reference_type: None,
            notes: input.notes,
            performed_by: input.performed_by,
        };

        self.repo
            .record_transaction(tenant_id, &tx_input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn record_usage(
        &self,
        tenant_id: Uuid,
        input: RecordUsageInput,
    ) -> Result<InventoryTransaction, AppError> {
        if input.quantity <= 0 {
            return Err(InventoryError::InvalidQuantity(
                "usage quantity must be positive".to_string(),
            )
            .into());
        }

        // Check stock availability
        let material = self
            .repo
            .get_material(tenant_id, input.material_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(AppError::from(InventoryError::NotFound))?;

        if material.current_stock < input.quantity {
            return Err(InventoryError::InsufficientStock {
                current: material.current_stock,
                requested: input.quantity,
            }
            .into());
        }

        let tx_input = RecordTransactionInput {
            material_id: input.material_id,
            transaction_type: "usage".to_string(),
            quantity: -input.quantity, // negative for outgoing
            unit_cost: None,
            reference_id: input.reference_id,
            reference_type: input.reference_type,
            notes: input.notes,
            performed_by: input.performed_by,
        };

        self.repo
            .record_transaction(tenant_id, &tx_input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn record_transaction(
        &self,
        tenant_id: Uuid,
        input: RecordTransactionInput,
    ) -> Result<InventoryTransaction, AppError> {
        let tx_type = TransactionType::from_str(&input.transaction_type).ok_or_else(|| {
            InventoryError::InvalidTransactionType(input.transaction_type.clone())
        })?;

        // Validate stock won't go negative for outgoing transactions
        if input.quantity < 0 {
            let material = self
                .repo
                .get_material(tenant_id, input.material_id)
                .await
                .map_err(AppError::Internal)?
                .ok_or(AppError::from(InventoryError::NotFound))?;

            let new_stock = material.current_stock + input.quantity;
            if new_stock < 0 {
                return Err(InventoryError::InsufficientStock {
                    current: material.current_stock,
                    requested: -input.quantity,
                }
                .into());
            }
        }

        let _ = tx_type; // validated above

        self.repo
            .record_transaction(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_transactions(
        &self,
        tenant_id: Uuid,
        material_id: Uuid,
        limit: i64,
    ) -> Result<Vec<InventoryTransaction>, AppError> {
        self.repo
            .list_transactions(tenant_id, material_id, limit)
            .await
            .map_err(AppError::Internal)
    }

    // --- Reports ---

    pub async fn low_stock_alerts(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<LowStockAlert>, AppError> {
        self.repo
            .low_stock_alerts(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn material_variance(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<MaterialVariance>, AppError> {
        self.repo
            .material_variance(tenant_id, location_id, from, to)
            .await
            .map_err(AppError::Internal)
    }
}
