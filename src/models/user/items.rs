use crate::database::DB;
use crate::models::user::item::User;
use crate::schema::users;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn fetch_items(count: Option<i64>, mut db: DB) -> Vec<User> {
    // Loading it from DB
    let limit: i64 = count.unwrap_or(100);

    let users = users::table
        .limit(limit)
        .order(users::columns::id.asc())
        .load::<User>(&mut db.connection)
        .unwrap();

    // Verbosity for console
    println!("Fetched users '{}'", &users.len());

    users
}
