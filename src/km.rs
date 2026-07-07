#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilometers {
    value: u32,
}

impl Kilometers {
    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub const fn into_u32(self) -> u32 {
        self.value
    }

    // ...
}
