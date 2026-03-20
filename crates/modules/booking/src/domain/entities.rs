use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use uuid::Uuid;

use super::BookingError;

#[derive(Debug, Clone, PartialEq)]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
    NoShow,
}

impl BookingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::NoShow => "no_show",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "confirmed" => Self::Confirmed,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            "no_show" => Self::NoShow,
            _ => Self::Pending,
        }
    }
}

impl std::fmt::Display for BookingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Booking {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub service_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub vehicle_type: String,
    pub booking_date: NaiveDate,
    pub time_slot: NaiveTime,
    pub status: BookingStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

impl Booking {
    /// Transition from pending -> confirmed.
    pub fn confirm(&mut self) -> Result<(), BookingError> {
        if self.status != BookingStatus::Pending {
            return Err(BookingError::InvalidTransition {
                from: self.status.to_string(),
                to: "confirmed".to_string(),
            });
        }
        self.status = BookingStatus::Confirmed;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Transition from confirmed -> completed.
    pub fn complete(&mut self) -> Result<(), BookingError> {
        if self.status != BookingStatus::Confirmed {
            return Err(BookingError::InvalidTransition {
                from: self.status.to_string(),
                to: "completed".to_string(),
            });
        }
        self.status = BookingStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Transition from pending or confirmed -> cancelled.
    pub fn cancel(&mut self) -> Result<(), BookingError> {
        if self.status != BookingStatus::Pending && self.status != BookingStatus::Confirmed {
            return Err(BookingError::InvalidTransition {
                from: self.status.to_string(),
                to: "cancelled".to_string(),
            });
        }
        self.status = BookingStatus::Cancelled;
        self.cancelled_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Transition from confirmed -> no_show.
    pub fn no_show(&mut self) -> Result<(), BookingError> {
        if self.status != BookingStatus::Confirmed {
            return Err(BookingError::InvalidTransition {
                from: self.status.to_string(),
                to: "no_show".to_string(),
            });
        }
        self.status = BookingStatus::NoShow;
        self.updated_at = Utc::now();
        Ok(())
    }
}
