use crate::schema::to_do;
use crate::to_do::enums::TaskStatus;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = to_do)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub creation_date: NaiveDateTime,
    pub modification_date: NaiveDateTime,
    pub deletion_date: NaiveDateTime,
}
