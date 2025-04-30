use crate::core::datetime::{today, LocalDate, LocalDateExtension};
use crate::core::domain::persistence::{Persistence, PersistenceAttribute};
use crate::core::domain::{Priority, TodoPriority};

mod task_progress;

pub struct Task {
    persistence: Persistence,
    expect_date: LocalDate,
}

impl PersistenceAttribute for Task {
    fn persistence(&self) -> &Persistence {
        &self.persistence
    }
}

impl Priority for Task {
    fn priority(&self) -> TodoPriority {
        let today = today();
        match self.expect_date {
            date if date > today => TodoPriority::High,
            date if { date.since_days(today) <= 1 } => TodoPriority::Medium,
            _ => TodoPriority::Low,
        }
    }
}
