use crate::core::datetime::LocalDateTime;
use crate::persistence::database::Database;
use crate::persistence::query::Queryable;
use crate::persistence::save::{Insertable, Savable, Updatable};
use crate::persistence::table::{Table, TableDefine};

pub mod database;
pub mod query;
pub mod save;
pub mod table;

pub trait Persistent<T = Self>: TableDefine + Insertable + Updatable + Queryable<T> {
    fn table() -> Table;

    fn id(&self) -> Option<i32>;
}
