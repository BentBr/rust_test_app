use crate::models::user::item::PasswordUser;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PasswordUserItem {
    pub uuid: String,
    pub password: String,
}

impl PasswordUserItem {
    pub fn new(input_item: &PasswordUser) -> PasswordUserItem {
        PasswordUserItem {
            uuid: input_item.uuid.to_string(),
            password: input_item.password.to_owned(),
        }
    }
}
