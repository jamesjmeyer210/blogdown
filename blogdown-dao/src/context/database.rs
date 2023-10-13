use std::sync::Arc;
use sqlx::SqlitePool;
use crate::context::{JoinTable, Table};
use crate::entities::GroupEntity;

pub(crate) struct Database<'c> {
    pub groups: Arc<Table<'c, GroupEntity>>,
    /*pub users: Arc<Table<'c, User<'_>>>,
    pub users_to_groups: Arc<JoinTable<'c, User<'_>, Group>>,
    pub blogs: Arc<Table<'c, Blog<'_>>>,
    pub users_to_blogs: Arc<JoinTable<'c, User<'_>, Blog<'_>>>,
    pub comments: Arc<Table<'c, Comment<'_>>>,
    pub users_to_comments: Arc<JoinTable<'c, User<'_>, Comment<'_>>>,
    pub blogs_to_comments: Arc<JoinTable<'c, Blog<'_>, Comment<'_>>>*/
}

impl<'a> Database<'a> {
    pub(crate) async fn new(sql_url: &String) -> Database<'a> {
        let connection = SqlitePool::connect(sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            groups: Arc::from(Table::new(pool.clone()))
        }
    }
}