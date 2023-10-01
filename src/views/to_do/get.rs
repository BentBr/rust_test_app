use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory, ItemTypes};

pub async fn get() -> impl Responder {
    let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);
    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let status: TaskStatus =
            TaskStatus::from_string(value["status"].as_str().unwrap().to_string());
        let item: ItemTypes = to_do_factory(&key, status, value["creation_date"].as_str().unwrap());

        array_buffer.push(item);
    }

    let return_package: ToDoItems = ToDoItems::new(array_buffer);
    return web::Json(return_package);
}
