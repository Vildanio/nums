/// Date with precision up to day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    pub const fn new(year: Year, month: Month, day: Day) -> Self {
        Self { year, month, day }
    }

    pub const fn year(self) -> Year {
        self.year
    }

    pub const fn month(self) -> Month {
        self.month
    }

    pub const fn day(self) -> Day {
        self.day
    }

    /// Checks if the date is valid (respects days in month)
    pub const fn is_valid(&self) -> bool {
        let month_days = match self.month {
            Month::January => 31,
            Month::February => {
                if self.year.is_leap() {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        };
        self.day.value() <= month_days
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Equal => {}
            other => return other,
        }
        match self.month.cmp(&other.month) {
            std::cmp::Ordering::Equal => {}
            other => return other,
        }
        self.day.cmp(&other.day)
    }
}

// ============================================================
// Year
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub const fn value(&self) -> u16 {
        self.value
    }

    /// Checks if the year is a leap year
    pub const fn is_leap(&self) -> bool {
        let year = self.value;
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    /// Returns the number of days in the year
    pub const fn days_in_year(&self) -> u16 {
        if self.is_leap() { 366 } else { 365 }
    }
}

impl From<u16> for Year {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<Year> for u16 {
    fn from(year: Year) -> Self {
        year.value
    }
}

// ============================================================
// Month
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Month {
    January = 1,
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

impl Month {
    /// Returns the month number (1..12)
    pub const fn number(&self) -> u8 {
        *self as u8
    }

    /// Creates a Month from a number (1..12)
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }

    /// Returns the number of days in the month for a given year
    pub const fn days_in_month(&self, year: Year) -> u8 {
        match self {
            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => 31,
            Month::April | Month::June | Month::September | Month::November => 30,
            Month::February => {
                if year.is_leap() {
                    29
                } else {
                    28
                }
            }
        }
    }
}

// ============================================================
// Day
// ============================================================

/// Number from 1 to 31
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day {
    value: u8,
}

impl Day {
    pub const fn new(value: u8) -> Option<Self> {
        if value == 0 || value > 31 {
            None
        } else {
            Some(Self { value })
        }
    }

    /// Creates a Day without validation (use only when you know the value is valid)
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> u8 {
        self.value
    }

    pub const fn into_u8(self) -> u8 {
        self.value
    }
}

impl TryFrom<u8> for Day {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 || value > 31 {
            Err("Day must be between 1 and 31")
        } else {
            Ok(Self { value })
        }
    }
}
