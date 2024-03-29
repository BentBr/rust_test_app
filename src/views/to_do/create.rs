use crate::json_serialization::new_to_do_item::NewToDoItem;
use actix_web::{web, HttpResponse};
use sentry::Level;
use uuid::Uuid;

use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::task::item::fetch_item;
use crate::models::task::new_item::create_item;

pub async fn create(new_to_do_item: web::Json<NewToDoItem>) -> HttpResponse {
    let uuid = Uuid::new_v4();
    let title = String::from(&new_to_do_item.title);
    let description = String::from(&new_to_do_item.description);

    // Creating in DB
    create_item(uuid, title, description);

    // Loading it again (the model with creation_date and other default values)
    let item = fetch_item(uuid);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Created new task".to_string(),
            ToDoItem::new(item),
        )),
        None => {
            // Logging a bit
            sentry::capture_message("Storing and lookup of new item failed!", Level::Error);

            HttpResponse::InternalServerError().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during item lookup after creation".to_string(),
                new_to_do_item,
            ))
        }
    }
}
