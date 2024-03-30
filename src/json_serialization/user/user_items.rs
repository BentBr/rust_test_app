use crate::json_serialization::user::user_item::UserItem;
use crate::models::user::item::User;
use serde::Serialize;
use std::vec::Vec;

#[derive(Serialize)]
pub struct UserItems {
    pub user_items: Vec<UserItem>,
    pub user_items_count: i8,
}

impl UserItems {
    pub fn new(input_items: Vec<User>) -> UserItems {
        let mut user_array_buffer = Vec::new();

        for item in input_items {
            let user_item = UserItem::new(item);

            user_array_buffer.push(user_item)
        }

        let open_count: i8 = user_array_buffer.len() as i8;

        UserItems {
            user_items: user_array_buffer,
            user_items_count: open_count,
        }
    }
}
