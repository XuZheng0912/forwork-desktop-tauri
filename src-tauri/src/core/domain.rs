use std::fmt::Display;
pub mod project;
mod task;

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub enum TodoPriority {
    Low,
    Medium,
    High,
}

impl Display for TodoPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TodoPriority::Low => "Low".to_string(),
            TodoPriority::Medium => "Medium".to_string(),
            TodoPriority::High => "High".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub trait Priority {
    fn priority(&self) -> TodoPriority;
}
