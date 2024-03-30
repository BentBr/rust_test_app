use crate::models::user::item::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EditUserItem {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
}

impl EditUserItem {
    pub fn new(input_item: User) -> EditUserItem {
        EditUserItem {
            uuid: input_item.uuid.to_string(),
            username: input_item.username.to_owned(),
            salutation: input_item.salutation.to_owned(),
            first_name: input_item.first_name.to_string(),
            last_name: input_item.last_name.to_string(),
            email: input_item.email.to_string(),
        }
    }
}
