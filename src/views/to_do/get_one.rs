use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::task::item::fetch_item;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;
use uuid::Uuid;

pub async fn get_one(request: HttpRequest) -> HttpResponse {
    let uuid: Uuid;
    match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => uuid = valid_uuid,
    }

    // Loading it
    let item = fetch_item(uuid);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Fetched one task".to_string(),
            json!(ToDoItem::new(item)).to_string(),
        )),
        None => {
            return HttpResponse::NotFound().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during task lookup".to_string(),
                "Could not find it".to_string(),
            ))
        }
    }
}
