/// Number representing quantity of something.
/// Must be explicitly casted to other types of numbers, which have different semantics.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity {
    value: u32,
}

impl Quantity {
    pub const ZERO: Quantity = Self { value: 0 };
    pub const MAX: Quantity = Self { value: u32::MAX };

    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub const fn into_u32(self) -> u32 {
        self.value
    }

    // todo:const
    pub fn add(self, rhs: Self) -> Option<Self> {
        self.value
            .checked_add(rhs.value)
            .map(|value| Self::from_u32(value))
    }

    // todo:const
    pub fn sub(self, rhs: Self) -> Option<Self> {
        self.value
            .checked_sub(rhs.value)
            .map(|value| Self::from_u32(value))
    }

    // todo:const
    pub fn mul(self, rhs: Self) -> Option<Self> {
        self.value
            .checked_mul(rhs.value)
            .map(|value| Self::from_u32(value))
    }

    // todo:const
    pub fn div(self, rhs: Self) -> Option<Self> {
        self.value
            .checked_div(rhs.value)
            .map(|value| Self::from_u32(value))
    }
}
