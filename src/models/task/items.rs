use crate::database::DB;
use crate::models::task::item::Task;
use crate::schema::to_do;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn fetch_items(count: Option<i64>, mut db: DB) -> Vec<Task> {
    // Loading it from DB
    let limit: i64 = count.unwrap_or(100);

    let tasks = to_do::table
        .limit(limit)
        .order(to_do::columns::id.asc())
        .load::<Task>(&mut db.connection)
        .unwrap();

    // Verbosity for console
    println!("Fetched items '{}'", &tasks.len());

    tasks
}
