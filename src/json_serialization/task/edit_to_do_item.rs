use crate::models::task::item::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EditToDoItem {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

impl EditToDoItem {
    pub fn new(input_item: Task) -> EditToDoItem {
        EditToDoItem {
            uuid: input_item.uuid.to_string(),
            title: input_item.title.to_owned(),
            description: input_item.description.to_owned(),
            status: input_item.status.to_string(),
        }
    }
}
