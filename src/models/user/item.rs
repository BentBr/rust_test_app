use crate::schema::users;
use bcrypt::verify;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Clone, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub creation_date: NaiveDateTime,
    pub modification_date: Option<NaiveDateTime>,
    pub deletion_date: Option<NaiveDateTime>,
}

impl User {
    pub fn verify(&self, password: String) -> bool {
        verify(password.as_str(), &self.password).unwrap()
    }
}
