pub trait Updatable {
    fn update(&self) -> rusqlite::Result<usize>;
}
