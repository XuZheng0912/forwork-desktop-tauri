use crate::core::domain::project::Project;
use crate::persistence::table::TableCreate;
use crate::persistence::Persistent;

pub fn init_environment() {
    init_database();
}

fn init_database() {
    create_table::<Project>()
}

fn create_table<T>()
where
    T: Persistent,
{
    T::create_if_not_exist().expect(create_table_failed_message(&T::table().table_name).as_str())
}

fn create_table_failed_message(table_name: &str) -> String {
    format!("create table {} failed", table_name)
}
