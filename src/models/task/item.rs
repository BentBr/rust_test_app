use crate::schema::to_do;
use crate::to_do::enums::TaskStatus;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = to_do)]
pub struct Task {
    pub id: i32,
    pub uuid: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub creation_date: NaiveDateTime,
    pub modification_date: Option<NaiveDateTime>,
    pub deletion_date: Option<NaiveDateTime>,
}
