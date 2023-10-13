use std::time::SystemTime;
use chrono::{DateTime, Utc};
use blogdown_model::Group;
use crate::entities::GroupEntity;
use crate::context::Table;

impl<'c> Table<'c, GroupEntity> {
    pub(crate) async fn add_group(&self, group: &Group) -> Result<u64, sqlx::Error>
    {
        let utc_now: DateTime<Utc> = DateTime::from(SystemTime::now());

        sqlx::query(
            r#"
            insert into `groups` (`name`, `created_on_utc`)
            values (?, ?)
            "#)
            .bind(group.name.as_str())
            .bind(utc_now)
            .execute(&*self.pool)
            .await
            .map(|x|x.rows_affected())
    }

    pub(crate) async fn get_group_by_name(&self, name: &str) -> Result<Group, sqlx::Error> {
        let x: GroupEntity = sqlx::query_as(
            r#"
                select `id`, `name`
                from `groups`
                where `name` = ?
            "#)
            .bind(name)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Group::from(x))
    }

    pub(crate) async fn get_group_by_id(&self, id: u32) -> Result<Group, sqlx::Error> {
        let x: GroupEntity = sqlx::query_as(
            r#"
                select `id`, `name`
                from `groups`
                where `id` = ?
            "#)
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Group::from(x))
    }
}