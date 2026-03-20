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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_booking() -> Booking {
        Booking {
            id: Uuid::now_v7(),
            tenant_id: Uuid::now_v7(),
            location_id: Uuid::now_v7(),
            service_id: Uuid::now_v7(),
            customer_name: "Test".into(),
            customer_phone: "0900000000".into(),
            vehicle_type: "sedan".into(),
            booking_date: NaiveDate::from_ymd_opt(2026, 3, 22).unwrap(),
            time_slot: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            status: BookingStatus::Pending,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            cancelled_at: None,
        }
    }

    #[test]
    fn confirm_from_pending_succeeds() {
        let mut b = make_booking();
        assert!(b.confirm().is_ok());
        assert_eq!(b.status, BookingStatus::Confirmed);
    }

    #[test]
    fn complete_from_confirmed_succeeds() {
        let mut b = make_booking();
        b.confirm().unwrap();
        assert!(b.complete().is_ok());
        assert_eq!(b.status, BookingStatus::Completed);
    }

    #[test]
    fn complete_from_pending_fails() {
        let mut b = make_booking();
        assert!(b.complete().is_err());
    }

    #[test]
    fn cancel_from_pending_succeeds() {
        let mut b = make_booking();
        assert!(b.cancel().is_ok());
        assert_eq!(b.status, BookingStatus::Cancelled);
        assert!(b.cancelled_at.is_some());
    }

    #[test]
    fn cancel_from_confirmed_succeeds() {
        let mut b = make_booking();
        b.confirm().unwrap();
        assert!(b.cancel().is_ok());
    }

    #[test]
    fn cancel_from_completed_fails() {
        let mut b = make_booking();
        b.confirm().unwrap();
        b.complete().unwrap();
        assert!(b.cancel().is_err());
    }

    #[test]
    fn no_show_from_confirmed_succeeds() {
        let mut b = make_booking();
        b.confirm().unwrap();
        assert!(b.no_show().is_ok());
        assert_eq!(b.status, BookingStatus::NoShow);
    }

    #[test]
    fn no_show_from_pending_fails() {
        let mut b = make_booking();
        assert!(b.no_show().is_err());
    }

    #[test]
    fn status_roundtrip() {
        for s in ["pending", "confirmed", "completed", "cancelled", "no_show"] {
            let status = BookingStatus::from_str(s);
            assert_eq!(status.as_str(), s);
        }
    }
}
