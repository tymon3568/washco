use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Material {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub category: MaterialCategory,
    pub unit: String,
    pub unit_cost: i64,
    pub current_stock: i64,
    pub min_stock: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MaterialNorm {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub service_id: Uuid,
    pub material_id: Uuid,
    pub quantity_per_job: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct InventoryTransaction {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub material_id: Uuid,
    pub transaction_type: TransactionType,
    pub quantity: i64,
    pub unit_cost: Option<i64>,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MaterialVariance {
    pub material_id: Uuid,
    pub material_name: String,
    pub unit: String,
    pub job_count: i64,
    pub expected_usage: i64,
    pub actual_usage: i64,
    pub variance: i64,
}

#[derive(Debug, Clone)]
pub struct LowStockAlert {
    pub material_id: Uuid,
    pub name: String,
    pub current_stock: i64,
    pub min_stock: i64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MaterialCategory {
    Chemical,
    Accessory,
    Consumable,
    Equipment,
}

impl MaterialCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            MaterialCategory::Chemical => "chemical",
            MaterialCategory::Accessory => "accessory",
            MaterialCategory::Consumable => "consumable",
            MaterialCategory::Equipment => "equipment",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "chemical" => Some(MaterialCategory::Chemical),
            "accessory" => Some(MaterialCategory::Accessory),
            "consumable" => Some(MaterialCategory::Consumable),
            "equipment" => Some(MaterialCategory::Equipment),
            _ => None,
        }
    }
}

impl std::fmt::Display for MaterialCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Purchase,
    Usage,
    Adjustment,
    Return,
    Waste,
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Purchase => "purchase",
            TransactionType::Usage => "usage",
            TransactionType::Adjustment => "adjustment",
            TransactionType::Return => "return",
            TransactionType::Waste => "waste",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "purchase" => Some(TransactionType::Purchase),
            "usage" => Some(TransactionType::Usage),
            "adjustment" => Some(TransactionType::Adjustment),
            "return" => Some(TransactionType::Return),
            "waste" => Some(TransactionType::Waste),
            _ => None,
        }
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
