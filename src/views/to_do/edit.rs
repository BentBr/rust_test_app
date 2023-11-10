use crate::json_serialization::to_do_item::ToDoItem;
use actix_web::{HttpResponse, web};
use serde_json::{Map, Value};
use crate::state::read_file;
use crate::processes::process_input;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);

    // Checking status of existing one
    let status = match &state.get(&to_do_item.title) {
        Some(result) => {
            TaskStatus::from_string(result.to_string())
        }
        None => {
            return HttpResponse::NotFound().json(
                format!("{} not found in state", &to_do_item.title)
            );
        }
    };

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone(), to_do_item.creation_date.as_str());
    let to_do_item_json: ToDoItem = ToDoItem::new(existing_item.clone());
    if status == TaskStatus::from_string(to_do_item.status.to_string()) {
        return HttpResponse::Ok().json(to_do_item_json);
    }

    process_input(&existing_item, "edit".to_owned(), &state);
    HttpResponse::Ok().json(to_do_item_json)
}
