use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use actix_web::{web, HttpResponse};
use serde_json::{Map, Value};

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);

    let status: TaskStatus;
    let creation_date: String;
    let status_string: String;

    // Checking status of existing one (and checking if existing in state)
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status_string = result["status"].to_string().trim_matches('"').to_string();
            creation_date = result["creation_date"]
                .to_string()
                .trim_matches('"')
                .to_string();

            status = TaskStatus::from_string(status_string.clone());
        }
        None => {
            return HttpResponse::NotFound()
                .json(format!("{} not found in state", &to_do_item.title));
        }
    };

    // Changing status
    let existing_item = to_do_factory(
        to_do_item.title.as_str(),
        status,
        creation_date.as_str(),
    );
    process_input(&existing_item, "edit".to_owned(), &state);

    let new_state: Map<String, Value> = read_file(&file_name);
    let title: &String = &to_do_item.title;

    return match new_state.get(title) {
        Some(result) => {
            let status_string: String = result["status"].to_string().trim_matches('"').to_string();
            let creation_date: String = result["creation_date"]
                .to_string()
                .trim_matches('"')
                .to_string();

            let item = to_do_factory(
                title,
                TaskStatus::from_string(status_string),
                &creation_date,
            );

            HttpResponse::Ok().json(ToDoItem::new(item))
        }
        None => {
            panic!(
                "Could not found item \"{}\" after changing to new status {}",
                &title, status_string
            )
        }
    };
}
