mod persistence;
mod project;
mod task;

pub enum TodoPriority {
    High,
    Medium,
    Low,
}

pub trait Priority {
    fn priority(&self) -> TodoPriority;
}