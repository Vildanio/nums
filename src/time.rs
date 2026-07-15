#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hours: u8,   // 0..23
    pub minutes: u8, // 0..59
    pub seconds: u8, // 0..59
}

impl Time {
    pub fn new(hours: u8, minutes: u8, seconds: u8) -> Option<Self> {
        if hours < 24 && minutes < 60 && seconds < 60 {
            Some(Self {
                hours,
                minutes,
                seconds,
            })
        } else {
            None
        }
    }

    /// Создает время из часов (минуты и секунды = 0)
    pub fn from_hours(hours: u8) -> Option<Self> {
        Self::new(hours, 0, 0)
    }

    /// Создает время из минут (часы = 0, секунды = 0)
    pub fn from_minutes(minutes: u16) -> Option<Self> {
        if minutes < 1440 {
            // 24 * 60
            let hours = (minutes / 60) as u8;
            let mins = (minutes % 60) as u8;
            Self::new(hours, mins, 0)
        } else {
            None
        }
    }

    pub fn to_seconds(&self) -> u32 {
        self.hours as u32 * 3600 + self.minutes as u32 * 60 + self.seconds as u32
    }

    pub fn from_seconds(total: u32) -> Self {
        let hours = (total / 3600) % 24;
        let minutes = (total / 60) % 60;
        let seconds = total % 60;
        Self {
            hours: hours as u8,
            minutes: minutes as u8,
            seconds: seconds as u8,
        }
    }
}
