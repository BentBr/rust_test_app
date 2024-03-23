use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::task::item::fetch_item;
use crate::models::task::new_item::create_item;

pub async fn create(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let uuid = Uuid::new_v4();
    let title = String::from(&to_do_item.title);
    let description = String::from(&to_do_item.description);

    // Creating in DB
    create_item(uuid, title, description);

    // Loading it again (the model with creation_date and other default values)
    let item = fetch_item(uuid);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Created new task".to_string(),
            json!(ToDoItem::new(item)).to_string(),
        )),
        None => {
            return HttpResponse::InternalServerError().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during item lookup after creation".to_string(),
                json!(to_do_item).to_string(),
            ))
        }
    }
}
