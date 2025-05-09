use crate::core::datetime::LocalDateTime;
use crate::core::domain::task::Task;
use crate::core::domain::{Priority, TodoPriority};
use crate::persistence::database::Database;
use crate::persistence::query::{FromQueryRow, Queryable};
use crate::persistence::save::{Insertable, Savable, Updatable};
use crate::persistence::table::{Table, TableDefine};
use crate::persistence::Persistent;
use rusqlite::{Result, Row};
use std::string::ToString;

pub struct Project {
    pub id: i32,
    pub create_time: LocalDateTime,
    pub update_time: LocalDateTime,

    pub name: String,
    pub simple_name: String,
    pub svn_name: String,
}

impl Project {
    const DEFINE_SQL: &'static str = "
    create table if not exists project 
    (
        id          integer not null
            constraint project_pk
            primary key autoincrement,
        name        TEXT    not null,
        simple_name TEXT,
        svn_name    TEXT    not null
    )";

    const INSERT_SQL: &'static str = "
    insert into project (name, simple_name, svn_name)
    values (?, ?, ?)
    ";

    const UPDATE_SQL: &'static str = "
    update project set 
    name = ?, 
    simple_name = ?, 
    svn_name = ? 
    where id = ?
    ";

    pub fn query_all_tasks(&self) -> Vec<Task> {
        Task::query_by_project(&self.id)
    }
}

impl TableDefine for Project {
    fn define_sql() -> &'static str {
        Self::DEFINE_SQL
    }
}

impl Insertable for Project {
    fn insert(&self) -> Result<usize> {
        Self::execute(
            Self::INSERT_SQL,
            [&self.name, &self.simple_name, &self.svn_name],
        )
    }
}

impl Updatable for Project {
    fn update(&self) -> Result<usize> {
        todo!()
    }
}

impl Queryable<Self> for Project {
    fn query_by_id(id: i32) -> Result<Self, ()> {
        todo!()
    }
}

impl Persistent for Project {
    fn table() -> Table {
        Table {
            table_name: "project".to_string(),
            field_names: vec![
                "name".to_string(),
                "simple_name".to_string(),
                "svn_name".to_string(),
            ],
        }
    }

    fn id(&self) -> Option<i32> {
        if self.id < 0 {
            None
        } else {
            Some(self.id)
        }
    }
}

impl FromQueryRow for Project {
    fn from_query_row(row: &Row) -> Self {
        todo!()
    }
}

impl Priority for Project {
    fn priority(&self) -> TodoPriority {
        self.query_all_tasks()
            .iter()
            .map(|task| task.priority())
            .max()
            .unwrap_or(TodoPriority::Low)
    }
}
