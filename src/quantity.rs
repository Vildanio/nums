use std::ops::{Add, Div, Mul, Sub};

/// Number representing quantity of something.
/// Must be explicitly casted to other types of numbers, which have different semantics.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity {
    value: u32,
}

impl Quantity {
    pub const ZERO: Quantity = Self { value: 0 };
}

impl From<Quantity> for u32 {
    fn from(value: Quantity) -> u32 {
        value.value
    }
}

impl From<Quantity> for f64 {
    fn from(value: Quantity) -> f64 {
        value.value as f64
    }
}

impl From<u32> for Quantity {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Sub for Quantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}
