use chrono::{DateTime, Utc};
use sqlx::{Error, FromRow, Row};
use sqlx::sqlite::SqliteRow;
use blogdown_model::Group;

pub(crate) struct GroupEntity
{
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) created_on_utc: DateTime<Utc>,
}

impl From<GroupEntity> for Group {
    fn from(value: GroupEntity) -> Self {
        Self {
            id: value.id,
            name: value.name
        }
    }
}

impl<'r> FromRow<'r, SqliteRow> for GroupEntity {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(GroupEntity {
            id: row.try_get(0)?,
            name: row.try_get(1)?,
            created_on_utc: row.try_get(2)?
        })
    }
}