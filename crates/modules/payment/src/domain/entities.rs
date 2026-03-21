use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    QR,
    EWallet,
}

impl PaymentMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::BankTransfer => "bank_transfer",
            Self::QR => "qr",
            Self::EWallet => "ewallet",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cash" => Some(Self::Cash),
            "bank_transfer" => Some(Self::BankTransfer),
            "qr" => Some(Self::QR),
            "ewallet" => Some(Self::EWallet),
            _ => None,
        }
    }
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Refunded,
}

impl PaymentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Completed => "completed",
            Self::Refunded => "refunded",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "completed" => Some(Self::Completed),
            "refunded" => Some(Self::Refunded),
            _ => None,
        }
    }
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub service_id: Uuid,
    pub service_name: String,
    pub base_price: i64,
    pub discount_amount: i64,
    pub final_amount: i64,
    pub promotion_id: Option<Uuid>,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub paid_at: Option<DateTime<Utc>>,
    pub collected_by: Uuid,
    pub verified_by: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub assistant_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentLineItem {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub service_id: Option<Uuid>,
    pub description: String,
    pub quantity: i32,
    pub unit_price: i64,
    pub amount: i64,
}
