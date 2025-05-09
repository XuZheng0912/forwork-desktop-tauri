use crate::persistence::database::Database;
use crate::persistence::Persistent;
use rusqlite::Row;

pub trait Queryable<T = Self> {
    fn query_by_id(id: i32) -> Result<T, ()>;
}

pub trait FromQueryRow {
    fn from_query_row(row: &Row) -> Self;
}

pub trait QueryAll<T = Self> {
    fn query_all() -> rusqlite::Result<Vec<T>>;
}

impl<T> QueryAll<T> for T
where
    T: Persistent + FromQueryRow,
{
    fn query_all() -> rusqlite::Result<Vec<T>> {
        let result = Self::get_connection()?
            .prepare(format!("select * from {}", Self::table().table_name).as_str())?
            .query_map([], |row| Ok(T::from_query_row(row)))?
            .filter(|row| row.is_ok())
            .map(|row| row.unwrap())
            .collect();
        Ok(result)
    }
}
