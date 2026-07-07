#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Year {
    value: u16,
}

impl Year {
    pub const fn from_u16(value: u16) -> Self {
        Self { value }
    }

    pub const fn into_u16(self) -> u16 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// Number from 1 to 31
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Day {
    value: u8,
}

impl Day {
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value == 0 || value > 31 {
            None
        } else {
            Some(Self { value })
        }
    }

    pub const fn into_u8(self) -> u8 {
        self.value
    }
}

/// Date with precision up to day.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalendarDate {
    year: Year,
    month: Month,
    day: Day,
}

impl CalendarDate {
    pub const fn year(self) -> Year {
        self.year
    }

    pub const fn month(self) -> Month {
        self.month
    }

    pub const fn day(self) -> Day {
        self.day
    }
}

impl PartialOrd for CalendarDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
