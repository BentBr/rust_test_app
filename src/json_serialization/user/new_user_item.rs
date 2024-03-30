use crate::models::user::item::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewUserItem {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUserItem {
    pub fn new(input_item: &User) -> NewUserItem {
        NewUserItem {
            username: input_item.username.to_owned(),
            email: input_item.email.to_owned(),
            password: input_item.password.to_owned(),
        }
    }
}
