use crate::database::establish_connection;
use crate::models::task_status::item::TaskStatus;
use crate::schema::to_do;
use diesel::RunQueryDsl;
use uuid::Uuid;

#[derive(Insertable, Debug)]
#[diesel(table_name = to_do)]
pub struct NewTask {
    pub uuid: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

impl NewTask {
    pub fn new(uuid: Uuid, title: String, description: String) -> NewTask {
        NewTask {
            uuid,
            title,
            description,
            status: TaskStatus::Open,
        }
    }
}

pub fn create_item(uuid: Uuid, title: String, description: String) {
    let mut connection = establish_connection();

    let new_item = NewTask::new(uuid, title, description);

    // Verbosity for console
    println!("Created new item: {:?}", &new_item);

    let exec = diesel::insert_into(to_do::table)
        .values(&new_item)
        .execute(&mut connection);

    if let Err(error) = exec {
        sentry::capture_error(&error);
    }
}
