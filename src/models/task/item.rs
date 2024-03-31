use crate::database::DB;
use crate::models::task_status::item::TaskStatus;
use crate::models::user::item::User;
use crate::schema::{to_do, users};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Debug, Clone, Associations)]
#[diesel(belongs_to(User))]
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
    pub user_id: i32,
}

pub fn fetch_item(uuid: Uuid, user_uuid: Uuid, mut db: DB) -> Vec<Task> {
    // Loading it from DB
    let task = to_do::table
        .inner_join(users::table.on(users::id.eq(to_do::user_id)))
        .filter(users::uuid.eq(user_uuid))
        .select(to_do::all_columns)
        .filter(to_do::columns::uuid.eq(uuid))
        .order(to_do::columns::id.asc())
        .load::<Task>(&mut db.connection)
        .unwrap();

    // Verbosity for console
    println!("Fetched task '{}'", uuid);

    task
}

pub fn delete_item(uuid: Uuid, user_uuid: Uuid, mut db: DB) -> Option<Uuid> {
    match id_for_todo(uuid, user_uuid, &mut db) {
        Ok(task_id) => {
            match diesel::delete(to_do::table.filter(to_do::columns::id.eq(task_id)))
                .execute(&mut db.connection)
            {
                Ok(exec) => {
                    // Verbosity for console
                    if exec > 0 {
                        println!("Deleted task '{}'", uuid);
                        return Some(uuid);
                    }

                    println!("Task not found for deletion '{}'", uuid);
                    None
                }
                Err(error) => {
                    // Logging a bit
                    sentry::capture_error(&error);

                    None
                }
            }
        }
        Err(_) => None,
    }
}

pub fn edit_item(
    uuid: Uuid,
    user_uuid: Uuid,
    title: String,
    description: String,
    status: TaskStatus,
    mut db: DB,
) -> Vec<Task> {
    // Verbosity for console
    println!(
        "Updating task '{}' with data: {}, {}, {}",
        uuid, title, description, status
    );

    match id_for_todo(uuid, user_uuid, &mut db) {
        Ok(task_id) => {
            let exec = diesel::update(to_do::table.filter(to_do::id.eq(task_id)))
                .set((
                    to_do::columns::title.eq(title),
                    to_do::columns::description.eq(description),
                    to_do::columns::status.eq(status),
                ))
                .execute(&mut db.connection);

            match exec {
                Ok(_) => {
                    println!("Task updated successfully");
                    fetch_item(uuid, user_uuid, db)
                }
                Err(error) => {
                    sentry::capture_error(&error);

                    Vec::new()
                }
            }
        }
        Err(_) => Vec::new(),
    }
}

pub fn in_progress_item(uuid: Uuid, user_uuid: Uuid, db: DB) -> Vec<Task> {
    let status = TaskStatus::InProgress;
    transition(uuid, user_uuid, status, db)
}

pub fn done_item(uuid: Uuid, user_uuid: Uuid, db: DB) -> Vec<Task> {
    let status = TaskStatus::Done;
    transition(uuid, user_uuid, status, db)
}

pub fn open_item(uuid: Uuid, user_uuid: Uuid, db: DB) -> Vec<Task> {
    let status = TaskStatus::Open;
    transition(uuid, user_uuid, status, db)
}

fn transition(uuid: Uuid, user_uuid: Uuid, status: TaskStatus, mut db: DB) -> Vec<Task> {
    // Verbosity for console
    println!("Transitioning task '{}' to '{}'", uuid, status);

    let results = to_do::table.filter(to_do::columns::uuid.eq(&uuid));
    let exec = diesel::update(results)
        .set(to_do::columns::status.eq(status))
        .execute(&mut db.connection);

    if let Err(error) = exec {
        sentry::capture_error(&error);
    }

    fetch_item(uuid, user_uuid, db)
}

fn id_for_todo(uuid: Uuid, user_uuid: Uuid, db: &mut DB) -> QueryResult<i32> {
    to_do::table
        .inner_join(users::table.on(users::id.eq(to_do::user_id)))
        .filter(users::uuid.eq(user_uuid))
        .filter(to_do::uuid.eq(&uuid))
        .select(to_do::id)
        .first::<i32>(&mut db.connection)
}
