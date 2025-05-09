use crate::core::datetime::{today, LocalDate, LocalDateExtension};
use crate::core::domain::{Priority, TodoPriority};

mod task_progress;

pub struct Task {
    expect_date: LocalDate,
}

impl Task {
    pub fn query_by_project(project_id: &i32) -> Vec<Task> {
        todo!()
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