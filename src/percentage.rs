use crate::Ratio;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage {
    value: f64,
}

impl Percentage {
    pub const HUNDRED: Percentage = Ratio::ONE.into_percentage();
    pub const HALF: Percentage = Ratio::HALF.into_percentage();
    pub const ZERO: Percentage = Ratio::ZERO.into_percentage();

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

    pub const fn from_ratio(ratio: Ratio) -> Self {
        Self {
            value: ratio.into_f64() * 100.0,
        }
    }

    pub const fn into_ratio(self) -> Ratio {
        Ratio::from_percentage(self)
    }

    pub const fn add(self, rhs: Self) -> Option<Self> {
        Self::from_f64(self.value + rhs.value)
    }

    pub const fn sub(self, rhs: Self) -> Option<Self> {
        Self::from_f64(self.value - rhs.value)
    }

    pub const fn mul(self, rhs: Self) -> Option<Self> {
        Self::from_f64(self.value * rhs.value)
    }

    pub const fn div(self, rhs: Self) -> Option<Self> {
        Self::from_f64(self.value / rhs.value)
    }
}
