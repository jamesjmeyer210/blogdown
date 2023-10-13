use sqlx::FromRow;
use blogdown_model::Group;
use crate::context::{Database, Table};
use crate::entities::GroupEntity;

mod context;
mod entities;
mod dao;

pub struct DaoContext<'a> {
    _database: Database<'a>
}

impl<'a> DaoContext<'a> {
    pub async fn new(sql_str: &String) -> DaoContext<'a> {
        DaoContext {
            _database: Database::new(sql_str).await
        }
    }
}

pub struct GroupCrud<'c> {
    _table: Table<'c, GroupEntity>
}

impl<'c> From<Table<'c, GroupEntity>> for GroupCrud<'c> {
    fn from(value: Table<'c, GroupEntity>) -> Self {
        Self {
            _table: value
        }
    }
}

impl<'c> GroupCrud<'c> {
    pub async fn add_group(&self, group: &Group) -> Result<u64, sqlx::Error> {
        self._table.add_group(group).await
    }

    pub async fn group_exists(&self, name: &str) -> Result<bool, sqlx::Error> {
        self._table.get_group_by_name(name)
            .await
            .map(|x|true)
    }
}