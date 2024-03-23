use crate::database::establish_connection;
use crate::models::task::item::Task;
use crate::schema::to_do;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn fetch_items(count: Option<i64>) -> Vec<Task> {
    let mut connection = establish_connection();
    let limit: i64;

    // Loading it from DB
    match count {
        Some(count) => {
            limit = count;
        }
        None => {
            limit = 100;
        }
    }

    to_do::table
        .limit(limit)
        .order(to_do::columns::id.asc())
        .load::<Task>(&mut connection)
        .unwrap()
}
