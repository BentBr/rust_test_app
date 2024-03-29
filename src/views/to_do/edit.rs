use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::edit_to_do_item::EditToDoItem;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::task::item::{edit_item, fetch_item};
use crate::models::task_status::item::TaskStatus;
use actix_web::{web, HttpRequest, HttpResponse};
use sentry::Level;
use uuid::Uuid;

pub async fn edit(to_do_item: web::Json<EditToDoItem>, request: HttpRequest) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    let title = String::from(&to_do_item.title);
    let description = String::from(&to_do_item.description);
    let status = TaskStatus::from_string(to_do_item.status.clone());

    // Editing in DB
    edit_item(uuid, title, description, status);

    // Loading it again (the model with modification_date and other default values)
    let item = fetch_item(uuid);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Updated task".to_string(),
            ToDoItem::new(item),
        )),
        None => {
            // Logging a bit
            sentry::capture_message("Editing and lookup of changed item failed!", Level::Error);

            HttpResponse::InternalServerError().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during task update".to_string(),
                to_do_item,
            ))
        }
    }
}
