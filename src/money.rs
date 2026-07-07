use std::marker::PhantomData;

pub type Rubles = Money<currencies::Ruble>;
pub type Dollars = Money<currencies::Dollar>;
pub type Euros = Money<currencies::Euro>;
pub type Yuans = Money<currencies::Yuan>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money<C: Currency> {
    value: u32,
    phantom_data: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    pub const fn from_u32(value: u32) -> Self {
        Self {
            value,
            phantom_data: PhantomData,
        }
    }

    pub const fn into_u32(self) -> u32 {
        self.value as u32
    }

    // You can sum 5 dollars and 2 dollars, but you can't
    // multiply or divide 5 dollars by 2 dollars.

    pub const fn add(self, rhs: &Self) -> Option<Self> {
        match self.value.checked_add(rhs.value) {
            Some(sum) => Some(Self::from_u32(sum)),
            None => None,
        }
    }

    pub const fn sub(self, rhs: &Self) -> Option<Self> {
        match self.value.checked_sub(rhs.value) {
            Some(diff) => Some(Self::from_u32(diff)),
            None => None,
        }
    }

    pub const fn mul(self, rhs: u32) -> Option<Self> {
        match self.value.checked_mul(rhs) {
            Some(product) => Some(Self::from_u32(product)),
            None => None,
        }
    }

    pub const fn div(self, rhs: u32) -> Option<Self> {
        match self.value.checked_div(rhs) {
            Some(quotient) => Some(Self::from_u32(quotient)),
            None => None,
        }
    }
}

pub trait Currency: Sized {
    const SYMBOL: char;
}

pub mod currencies {
    use crate::Currency;

    pub struct Ruble;
    impl Currency for Ruble {
        const SYMBOL: char = '₽';
    }

    pub struct Dollar;
    impl Currency for Dollar {
        const SYMBOL: char = '$';
    }

    pub struct Euro;
    impl Currency for Euro {
        const SYMBOL: char = '€';
    }

    pub struct Yuan;
    impl Currency for Yuan {
        const SYMBOL: char = '¥';
    }
}
