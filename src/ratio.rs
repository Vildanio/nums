use crate::Percentage;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio {
    value: f64,
}

impl Ratio {
    pub const ZERO: Ratio = Self { value: 0.0 };
    pub const HALF: Ratio = Self { value: 0.5 };
    pub const ONE: Ratio = Self { value: 1.0 };

    pub const fn from_f64(value: f64) -> Option<Self> {
        if value.is_normal() {
            Some(Self { value })
        } else {
            None
        }
    }

    pub const fn into_f64(self) -> f64 {
        self.value
    }

    pub const fn from_percentage(value: Percentage) -> Self {
        Self {
            value: value.into_f64() / 100.0,
        }
    }

    pub const fn into_percentage(self) -> Percentage {
        Percentage::from_ratio(self)
    }

    pub const fn add(self, rhs: &Self) -> Option<Self> {
        Self::from_f64(self.value + rhs.value)
    }

    pub const fn sub(self, rhs: &Self) -> Option<Self> {
        Self::from_f64(self.value - rhs.value)
    }

    pub const fn mul(self, rhs: &Self) -> Option<Self> {
        Self::from_f64(self.value * rhs.value)
    }

    pub const fn div(self, rhs: &Self) -> Option<Self> {
        Self::from_f64(self.value / rhs.value)
    }

    pub const fn mul_f64(self, rhs: f64) -> Option<Self> {
        Self::from_f64(self.value * rhs)
    }

    pub const fn div_f64(self, rhs: f64) -> Option<Self> {
        Self::from_f64(self.value / rhs)
    }
}
