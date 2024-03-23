use crate::database::establish_connection;
use crate::schema::to_do;
use crate::to_do::enums::TaskStatus;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
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
    to_do::table
        .filter(to_do::columns::uuid.eq(uuid))
        .order(to_do::columns::id.asc())
        .load::<Task>(&mut connection)
        .unwrap()
}

pub fn delete_item(uuid: Uuid) {
    let mut connection = establish_connection();
    let item = fetch_item(uuid);

    match item.first() {
        Some(item) => {
            match diesel::delete(item).execute(&mut connection) {
                Ok(_) => {}
                Err(error) => {
                    panic!("Could not delete: {}", error.to_string())
                }
            };
        }
        None => {
            panic!("Cannot delete item: not found during deletion!");
        }
    }
}

pub fn edit_item(uuid: Uuid, title: String, description: String, status: TaskStatus) {
    let mut connection = establish_connection();

    let results = to_do::table.filter(to_do::columns::uuid.eq(&uuid));
    let _ = diesel::update(results)
        .set((
            to_do::columns::title.eq(title),
            to_do::columns::description.eq(description),
            to_do::columns::status.eq(status),
        ))
        .execute(&mut connection);
}

pub fn in_progress_item(uuid: Uuid) {}

pub fn done_item(uuid: Uuid) {}

pub fn open_item(uuid: Uuid) {}
