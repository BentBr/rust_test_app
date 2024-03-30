use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::user_item::UserItem;
use crate::models::user::item::fetch_item;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn get_one(request: HttpRequest, db: DB) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Loading it
    let item = fetch_item(uuid, db);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Fetched one user".to_string(),
            UserItem::new(item.clone()),
        )),
        None => HttpResponse::NotFound().json(ResponseItem::new(
            ResponseStatus::Error,
            "Error during user lookup".to_string(),
            "Could not find it",
        )),
    }
}
