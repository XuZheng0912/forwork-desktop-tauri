use crate::core::domain::persistence::{Persistence, PersistenceAttribute, Table};

pub struct Project {
    persistence: Persistence,
}

impl Project {
    const TABLE_NAME: &'static str = "project";
}

impl PersistenceAttribute for Project {
    fn persistence(&self) -> &Persistence {
        &self.persistence
    }
}

impl Table for Project {
    fn table_name() -> String {
        Self::TABLE_NAME.to_string()
    }
}
