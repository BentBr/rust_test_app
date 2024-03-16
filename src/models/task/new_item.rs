use crate::schema::to_do;
use crate::to_do::enums::TaskStatus;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub uuid: Uuid,
    pub status: TaskStatus,
}

impl NewTask {
    pub fn new(title: String, description: String, uuid: Uuid) -> NewTask {
        NewTask {
            title,
            description,
            uuid,
            status: TaskStatus::Open,
        }
    }
}
