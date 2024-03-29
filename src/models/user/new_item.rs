use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> NewUser {
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4();

        NewUser {
            uuid,
            username,
            email,
            password: hashed_password,
        }
    }
}
