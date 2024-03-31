use crate::database::DB;
use crate::models::task::item::{fetch_item, Task};
use crate::models::task_status::item::TaskStatus;
use crate::schema::to_do;
use diesel::RunQueryDsl;
use serde::Serialize;
use serde_json::json;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Insertable, Debug, Serialize)]
#[diesel(table_name = to_do)]
pub struct NewTask {
    pub uuid: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub user_id: i32,
}

impl NewTask {
    pub fn new(uuid: Uuid, title: String, description: String, user_id: i32) -> NewTask {
        NewTask {
            uuid,
            title,
            description,
            status: TaskStatus::Open,
            user_id,
        }
    }
}

impl Display for NewTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

pub fn create_item(title: String, description: String, user_id: i32, mut db: DB) -> Vec<Task> {
    let uuid = Uuid::new_v4();
    let new_item = NewTask::new(uuid, title, description, user_id);

    // Verbosity for console
    println!("Created new task: {}", &new_item);

    let exec = diesel::insert_into(to_do::table)
        .values(&new_item)
        .execute(&mut db.connection);

    if let Err(error) = exec {
        sentry::capture_error(&error);
    }

    fetch_item(uuid, db)
}
