use crate::schema::to_do;
use crate::to_do::enums::TaskStatus;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;


#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewItem {
    pub title: String,
    pub description: String,
    pub uuid: Uuid,
    pub status: TaskStatus,
    pub date: NaiveDateTime,
}

impl NewItem {
    pub fn new(title: String, description: String, uuid: Uuid) -> NewItem {
        let now = Utc::now().naive_local();
        
        NewItem {
            title,
            description,
            uuid,
            status: TaskStatus::Open,
            date: now,
        }
    }
}
