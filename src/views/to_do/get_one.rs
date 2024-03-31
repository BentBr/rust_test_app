use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::task::to_do_item::ToDoItem;
use crate::jwt::JwToken;
use crate::models::task::item::fetch_item;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn get_one(request: HttpRequest, db: DB, token: JwToken) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Loading it
    let item = fetch_item(uuid, token.user_uuid, db);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Fetched one task".to_string(),
            ToDoItem::new(item.clone()),
        )),
        None => HttpResponse::NotFound().json(ResponseItem::new(
            ResponseStatus::Error,
            "Error during task lookup".to_string(),
            "Could not find it",
        )),
    }
}
