pub trait Insertable {
    fn insert(&self) -> rusqlite::Result<usize>;
}
