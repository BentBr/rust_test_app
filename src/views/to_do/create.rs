use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::task::new_to_do_item::NewToDoItem;
use crate::json_serialization::task::to_do_item::ToDoItem;
use crate::models::task::new_item::create_item;
use actix_web::{web, HttpResponse};
use sentry::Level;

pub async fn create(new_to_do_item: web::Json<NewToDoItem>, db: DB) -> HttpResponse {
    let title = String::from(&new_to_do_item.title);
    let description = String::from(&new_to_do_item.description);
    let user_id = new_to_do_item.user_id;

    // Creating in DB
    let item = create_item(title, description, user_id, db);

    match item.first() {
        Some(item) => HttpResponse::Created().json(ResponseItem::new(
            ResponseStatus::Success,
            "Created new task".to_string(),
            ToDoItem::new(item.clone()),
        )),
        None => {
            // Logging a bit
            sentry::capture_message("Storing and lookup of new item failed!", Level::Error);

            HttpResponse::InternalServerError().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during task lookup and creation".to_string(),
                new_to_do_item,
            ))
        }
    }
}
