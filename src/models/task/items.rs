use crate::database::DB;
use crate::models::task::item::Task;
use crate::schema::{to_do, users};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn fetch_items(count: Option<i64>, user_uuid: Uuid, mut db: DB) -> Vec<Task> {
    // Loading it from DB
    let limit: i64 = count.unwrap_or(100);

    let tasks = to_do::table
        .inner_join(users::table.on(users::id.eq(to_do::user_id)))
        .filter(users::uuid.eq(user_uuid))
        .select(to_do::all_columns)
        .limit(limit)
        .order(to_do::columns::id.asc())
        .load::<Task>(&mut db.connection)
        .unwrap();

    // Verbosity for console
    println!("Fetched tasks '{}'", &tasks.len());

    tasks
}
