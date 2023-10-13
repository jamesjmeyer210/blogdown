mod table;
mod join_table;
mod database;

pub(crate) type Table<'c, T> = table::Table<'c, T>;
pub(crate) type JoinTable<'c, T1, T2> = join_table::JoinTable<'c, T1, T2>;
pub(crate) type Database<'a> = database::Database<'a>;