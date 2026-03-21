use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StaffProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub location_id: Uuid,
    pub display_name: String,
    pub skill_level: SkillLevel,
    pub hourly_rate: i64,
    pub is_active: bool,
    pub joined_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Shift {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub staff_id: Uuid,
    pub shift_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: ShiftStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CommissionRule {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub service_id: Option<Uuid>,
    pub skill_level: Option<SkillLevel>,
    pub role_in_job: JobRole,
    pub commission_type: CommissionType,
    pub commission_value: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CommissionEntry {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub payment_id: Uuid,
    pub staff_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub role_in_job: JobRole,
    pub payment_amount: i64,
    pub commission_amount: i64,
    pub created_at: DateTime<Utc>,
}

// --- Enums ---

#[derive(Debug, Clone, PartialEq)]
pub enum SkillLevel {
    Junior,
    Senior,
    Lead,
    Detailer,
}

impl SkillLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkillLevel::Junior => "junior",
            SkillLevel::Senior => "senior",
            SkillLevel::Lead => "lead",
            SkillLevel::Detailer => "detailer",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "junior" => Some(SkillLevel::Junior),
            "senior" => Some(SkillLevel::Senior),
            "lead" => Some(SkillLevel::Lead),
            "detailer" => Some(SkillLevel::Detailer),
            _ => None,
        }
    }
}

impl std::fmt::Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShiftStatus {
    Scheduled,
    Active,
    Completed,
    Absent,
}

impl ShiftStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ShiftStatus::Scheduled => "scheduled",
            ShiftStatus::Active => "active",
            ShiftStatus::Completed => "completed",
            ShiftStatus::Absent => "absent",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "scheduled" => Some(ShiftStatus::Scheduled),
            "active" => Some(ShiftStatus::Active),
            "completed" => Some(ShiftStatus::Completed),
            "absent" => Some(ShiftStatus::Absent),
            _ => None,
        }
    }
}

impl std::fmt::Display for ShiftStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobRole {
    Primary,
    Assistant,
}

impl JobRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobRole::Primary => "primary",
            JobRole::Assistant => "assistant",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "primary" => Some(JobRole::Primary),
            "assistant" => Some(JobRole::Assistant),
            _ => None,
        }
    }
}

impl std::fmt::Display for JobRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommissionType {
    Percentage,
    Fixed,
}

impl CommissionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommissionType::Percentage => "percentage",
            CommissionType::Fixed => "fixed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "percentage" => Some(CommissionType::Percentage),
            "fixed" => Some(CommissionType::Fixed),
            _ => None,
        }
    }
}

impl std::fmt::Display for CommissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
