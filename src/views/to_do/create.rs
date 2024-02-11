use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use actix_web::{HttpRequest, HttpResponse};
use chrono::Utc;
use serde_json::{Map, Value};

pub async fn create(req: HttpRequest) -> HttpResponse {
    let item = to_do_factory(
        req.match_info().get("title").unwrap().to_string().as_str(),
        TaskStatus::Open,
        Utc::now().to_string().as_str(),
    );

    // Writing to file
    let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);
    process_input(&item, "create".to_string(), &state);

    ToDoItem::get_state(req)
}
