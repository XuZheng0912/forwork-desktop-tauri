use crate::persistence::Persistent;
use rusqlite::{Connection, Params, Result};

pub trait Database {
    fn get_connection() -> Result<Connection>;

    fn execute<P>(sql: &str, params: P) -> Result<usize>
    where
        P: Params;
}

impl<T> Database for T
where
    T: Persistent,
{
    fn get_connection() -> Result<Connection> {
        Connection::open("data.db")
    }

    fn execute<P>(sql: &str, params: P) -> Result<usize>
    where
        P: Params,
    {
        Self::get_connection()?.execute(sql, params)
    }
}
