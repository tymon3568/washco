use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    InventoryTransaction, LowStockAlert, Material, MaterialNorm, MaterialVariance,
};

// --- Material ---

#[derive(Debug, Deserialize)]
pub struct CreateMaterialRequest {
    pub name: String,
    pub category: String,
    pub unit: String,
    pub unit_cost: i64,
    pub current_stock: i64,
    pub min_stock: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMaterialRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub unit: Option<String>,
    pub unit_cost: Option<i64>,
    pub min_stock: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MaterialResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub category: String,
    pub unit: String,
    pub unit_cost: i64,
    pub current_stock: i64,
    pub min_stock: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Material> for MaterialResponse {
    fn from(m: Material) -> Self {
        Self {
            id: m.id,
            location_id: m.location_id,
            name: m.name,
            category: m.category.to_string(),
            unit: m.unit,
            unit_cost: m.unit_cost,
            current_stock: m.current_stock,
            min_stock: m.min_stock,
            is_active: m.is_active,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

// --- Norm ---

#[derive(Debug, Deserialize)]
pub struct SetNormRequest {
    pub service_id: Uuid,
    pub material_id: Uuid,
    pub quantity_per_job: i64,
}

#[derive(Debug, Serialize)]
pub struct MaterialNormResponse {
    pub id: Uuid,
    pub service_id: Uuid,
    pub material_id: Uuid,
    pub quantity_per_job: i64,
    pub created_at: DateTime<Utc>,
}

impl From<MaterialNorm> for MaterialNormResponse {
    fn from(n: MaterialNorm) -> Self {
        Self {
            id: n.id,
            service_id: n.service_id,
            material_id: n.material_id,
            quantity_per_job: n.quantity_per_job,
            created_at: n.created_at,
        }
    }
}

// --- Transaction ---

#[derive(Debug, Deserialize)]
pub struct RecordTransactionRequest {
    pub material_id: Uuid,
    pub transaction_type: String,
    pub quantity: i64,
    pub unit_cost: Option<i64>,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InventoryTransactionResponse {
    pub id: Uuid,
    pub material_id: Uuid,
    pub transaction_type: String,
    pub quantity: i64,
    pub unit_cost: Option<i64>,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<InventoryTransaction> for InventoryTransactionResponse {
    fn from(t: InventoryTransaction) -> Self {
        Self {
            id: t.id,
            material_id: t.material_id,
            transaction_type: t.transaction_type.to_string(),
            quantity: t.quantity,
            unit_cost: t.unit_cost,
            reference_id: t.reference_id,
            reference_type: t.reference_type,
            notes: t.notes,
            performed_by: t.performed_by,
            created_at: t.created_at,
        }
    }
}

// --- Reports ---

#[derive(Debug, Serialize)]
pub struct LowStockAlertResponse {
    pub material_id: Uuid,
    pub name: String,
    pub current_stock: i64,
    pub min_stock: i64,
    pub unit: String,
}

impl From<LowStockAlert> for LowStockAlertResponse {
    fn from(a: LowStockAlert) -> Self {
        Self {
            material_id: a.material_id,
            name: a.name,
            current_stock: a.current_stock,
            min_stock: a.min_stock,
            unit: a.unit,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MaterialVarianceResponse {
    pub material_id: Uuid,
    pub material_name: String,
    pub unit: String,
    pub job_count: i64,
    pub expected_usage: i64,
    pub actual_usage: i64,
    pub variance: i64,
}

impl From<MaterialVariance> for MaterialVarianceResponse {
    fn from(v: MaterialVariance) -> Self {
        Self {
            material_id: v.material_id,
            material_name: v.material_name,
            unit: v.unit,
            job_count: v.job_count,
            expected_usage: v.expected_usage,
            actual_usage: v.actual_usage,
            variance: v.variance,
        }
    }
}

// --- Shared ---

#[derive(Debug, Deserialize)]
pub struct VarianceQuery {
    pub from: chrono::NaiveDate,
    pub to: chrono::NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
