use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::models::user::item::delete_item;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn delete(request: HttpRequest, db: DB) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    delete_item(uuid, db);

    HttpResponse::Ok().json(ResponseItem::new(
        ResponseStatus::Success,
        "Deleted user".to_string(),
        "Done with success",
    ))
}
