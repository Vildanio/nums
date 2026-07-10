pub struct Hour {
    minutes: Minutes,
}

impl Hour {
    pub const fn from_minutes(minutes: Minutes) -> Self {
        Self { minutes }
    }

    pub const fn into_minutes(self) -> Minutes {
        self.minutes
    }

    pub const fn hours(self) -> u32 {
        self.minutes.minutes() / 60
    }

    pub const fn minutes(self) -> u32 {
        self.minutes.minutes() % 60
    }

    pub const fn seconds(self) -> u32 {
        self.minutes.seconds()
    }
}

pub struct Minutes {
    seconds: Seconds,
}

impl Minutes {
    pub const fn from_seconds(seconds: Seconds) -> Self {
        Self { seconds }
    }

    pub const fn into_seconds(self) -> Seconds {
        self.seconds
    }

    pub const fn minutes(self) -> u32 {
        self.seconds.into_u32() / 60
    }

    pub const fn seconds(self) -> u32 {
        self.seconds.into_u32() % 60
    }
}

pub struct Seconds {
    value: u32,
}

impl Seconds {
    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub const fn into_u32(self) -> u32 {
        self.value
    }
}
