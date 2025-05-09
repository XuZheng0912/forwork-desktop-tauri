use crate::persistence::Database;

pub struct Table {
    pub table_name: String,
    pub field_names: Vec<String>,
}

pub trait TableDefine {
    fn define_sql() -> &'static str;
}

pub trait TableCreate: TableDefine + Database {
    fn create_if_not_exist() -> rusqlite::Result<()>;
}

impl<T> TableCreate for T
where
    T: TableDefine + Database,
{
    fn create_if_not_exist() -> rusqlite::Result<()> {
        let conn = Self::get_connection()?;
        conn.execute(T::define_sql(), [])?;
        Ok(())
    }
}
