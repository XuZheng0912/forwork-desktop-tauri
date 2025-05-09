use crate::persistence::database::Database;
use crate::persistence::query::Queryable;
pub use crate::persistence::save::insert::Insertable;
pub use crate::persistence::save::update::Updatable;
use crate::persistence::Persistent;
use rusqlite::Result;

mod insert;
mod update;

pub trait Savable: Database {
    fn save(&self) -> Result<usize>;
}

impl<T> Savable for T
where
    T: Persistent,
{
    fn save(&self) -> Result<usize> {
        match self.id() {
            None => self.insert(),
            Some(_) => self.update(),
        }
    }
}

pub trait SaveAll {
    fn save_all(&self) -> Result<()>;
}

impl<T> SaveAll for Vec<T>
where
    T: Savable,
{
    fn save_all(&self) -> Result<()> {
        match self.iter().map(|it| it.save()).find(|it| it.is_err()) {
            None => Ok(()),
            Some(it) => Err(it.unwrap_err()),
        }
    }
}
