use crate::core::datetime::LocalDateTime;
use rusqlite::Result;
use rusqlite::{Connection, Row};

pub trait Persistent {
    fn id(&self) -> &i32;
    fn create_time(&self) -> &LocalDateTime;
    fn update_time(&self) -> &LocalDateTime;
}

pub trait PersistenceAttribute {
    fn persistence(&self) -> &Persistence;
}

pub struct Persistence {
    id: i32,
    create_time: LocalDateTime,
    update_time: LocalDateTime,
}

impl<T> Persistent for T
where
    T: PersistenceAttribute,
{
    fn id(&self) -> &i32 {
        &self.persistence().id
    }

    fn create_time(&self) -> &LocalDateTime {
        &self.persistence().create_time
    }

    fn update_time(&self) -> &LocalDateTime {
        &self.persistence().update_time
    }
}

pub trait Database {
    fn get_connection() -> Result<Connection>;
}

impl<T> Database for T
where
    T: Persistent,
{
    fn get_connection() -> Result<Connection> {
        Connection::open("data.db")
    }
}

pub trait Table {
    fn table_name() -> String;
}

pub trait QueryAll<T = Self> {
    fn query_all() -> Result<Vec<T>>;
}

impl<T> QueryAll<T> for T
where
    T: Database + Table + FromQueryRow,
{
    fn query_all() -> Result<Vec<T>> { 
        todo!()
    }
}

pub trait FromQueryRow<T = Self> {
    fn from_row(row: &Row<'_>) -> Result<T>;
}
