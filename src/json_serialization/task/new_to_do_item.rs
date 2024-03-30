use crate::models::task::item::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewToDoItem {
    pub title: String,
    pub description: String,
    pub user_id: i32,
}

impl NewToDoItem {
    pub fn new(input_item: Task) -> NewToDoItem {
        NewToDoItem {
            title: input_item.title.to_owned(),
            description: input_item.description.to_owned(),
            user_id: input_item.user_id.to_owned(),
        }
    }
}
