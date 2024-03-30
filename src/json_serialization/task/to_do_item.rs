use crate::helpers::datetime::format_datetime;
use crate::models::task::item::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ToDoItem {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub creation_date: String,
    pub modification_date: Option<String>,
    pub deletion_date: Option<String>,
}

impl ToDoItem {
    pub fn new(input_item: &Task) -> ToDoItem {
        ToDoItem {
            uuid: input_item.uuid.to_string(),
            title: input_item.title.to_owned(),
            description: input_item.description.to_owned(),
            status: input_item.status.to_string(),
            creation_date: input_item.creation_date.to_string(),
            modification_date: format_datetime(input_item.modification_date),
            deletion_date: format_datetime(input_item.deletion_date),
        }
    }
}
