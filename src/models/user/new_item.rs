use crate::database::DB;
use crate::models::user::item::{fetch_item, User};
use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};
use diesel::{Insertable, RunQueryDsl};
use uuid::Uuid;

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(uuid: Uuid, username: String, email: String, password: String) -> NewUser {
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();

        NewUser {
            uuid,
            username,
            email,
            password: hashed_password,
        }
    }
}

pub fn create_item(username: String, email: String, password: String, mut db: DB) -> Vec<User> {
    let uuid = Uuid::new_v4();
    let new_item = NewUser::new(uuid, username, email, password);

    // Verbosity for console
    println!("Created new user: {:?}", &new_item);

    let exec = diesel::insert_into(users::table)
        .values(&new_item)
        .execute(&mut db.connection);

    if let Err(error) = exec {
        sentry::capture_error(&error);
    }

    fetch_item(uuid, db)
}
