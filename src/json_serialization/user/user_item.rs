use crate::helpers::datetime::format_datetime;
use crate::models::user::item::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserItem {
    pub uuid: String,
    pub username: String,
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub creation_date: String,
    pub modification_date: Option<String>,
    pub deletion_date: Option<String>,
}

impl UserItem {
    pub fn new(input_item: User) -> UserItem {
        UserItem {
            uuid: input_item.uuid.to_string(),
            username: input_item.username.to_owned(),
            salutation: input_item.salutation.to_owned(),
            first_name: input_item.first_name.to_string(),
            last_name: input_item.last_name.to_string(),
            email: input_item.email.to_string(),
            creation_date: input_item.creation_date.to_string(),
            modification_date: format_datetime(input_item.modification_date),
            deletion_date: format_datetime(input_item.deletion_date),
        }
    }
}
