mod table;
mod join_table;
mod database;

pub type Table<'c, T> = table::Table<'c, T>;
pub type JoinTable<'c, T1, T2> = join_table::JoinTable<'c, T1, T2>;