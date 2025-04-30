use chrono::{DateTime, Local, NaiveDate};

pub type LocalDateTime = DateTime<Local>;
pub type LocalDate = NaiveDate;

pub fn today() -> LocalDate {
    now().date_naive()
}

pub fn now() -> LocalDateTime {
    Local::now()
}

pub trait LocalDateExtension<T = Self> {
    fn since_days(&self, other: T) -> i64;

    fn until_days(&self, other: T) -> i64;
}

impl LocalDateExtension for LocalDate {
    fn since_days(&self, other: Self) -> i64 {
        self.signed_duration_since(other).num_days()
    }

    fn until_days(&self, other: Self) -> i64 {
        -self.since_days(other)
    }
}
