use serde::{Deserialize, Serialize};

/// Money amount stored as BIGINT in minor units.
/// For VND, 1 VND = 1 minor unit (no sub-units).
/// Example: 150_000 VND is stored as 150_000.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(i64);

impl Money {
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn amount(&self) -> i64 {
        self.0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }
}

impl std::ops::Add for Money {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format as Vietnamese style: 150.000
        let abs = self.0.unsigned_abs();
        let formatted = format_vnd(abs);
        if self.0 < 0 {
            write!(f, "-{formatted}")
        } else {
            write!(f, "{formatted}")
        }
    }
}

fn format_vnd(amount: u64) -> String {
    let s = amount.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_vnd_display() {
        assert_eq!(Money::new(150_000).to_string(), "150.000");
        assert_eq!(Money::new(1_250_000).to_string(), "1.250.000");
        assert_eq!(Money::new(0).to_string(), "0");
        assert_eq!(Money::new(500).to_string(), "500");
    }
}
