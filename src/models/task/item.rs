use crate::database::establish_connection;
use crate::models::task_status::item::TaskStatus;
use crate::schema::to_do;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use sentry::Level;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Debug, Clone)]
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

pub fn fetch_item(uuid: Uuid) -> Vec<Task> {
    let mut connection = establish_connection();

    // Loading it from DB
    let task = to_do::table
        .filter(to_do::columns::uuid.eq(uuid))
        .order(to_do::columns::id.asc())
        .load::<Task>(&mut connection)
        .unwrap();

    // Verbosity for console
    println!("Fetched item: {}", &task.len());

    task
}

pub fn delete_item(uuid: Uuid) {
    let mut connection = establish_connection();
    let item = fetch_item(uuid);

    match item.first() {
        Some(item) => {
            match diesel::delete(item).execute(&mut connection) {
                Ok(_) => {
                    // Verbosity for console
                    println!("Deleted item: {}", uuid);
                }
                Err(error) => {
                    // Logging a bit
                    sentry::capture_error(&error);
                }
            };
        }
        None => {
            // Logging a bit
            sentry::capture_message(
                "Cannot delete item: not found during deletion!",
                Level::Error,
            );
        }
    }
}

pub fn edit_item(uuid: Uuid, title: String, description: String, status: TaskStatus) {
    let mut connection = establish_connection();

    // Verbosity for console
    println!(
        "Updating item: {}, {}, {}, {}",
        uuid, title, description, status
    );

    let results = to_do::table.filter(to_do::columns::uuid.eq(&uuid));
    let exec = diesel::update(results)
        .set((
            to_do::columns::title.eq(title),
            to_do::columns::description.eq(description),
            to_do::columns::status.eq(status),
        ))
        .execute(&mut connection);

    if let Err(error) = exec {
        sentry::capture_error(&error);
    }
}

pub fn _in_progress_item(_uuid: Uuid) {}

pub fn _done_item(_uuid: Uuid) {}

pub fn _open_item(_uuid: Uuid) {}
