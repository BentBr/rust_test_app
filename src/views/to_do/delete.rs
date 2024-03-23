use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::models::task::item::delete_item;
use actix_web::{HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn delete(request: HttpRequest) -> HttpResponse {
    let uuid: Uuid;
    match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => uuid = valid_uuid,
    }

    delete_item(uuid);

    HttpResponse::Ok().json(ResponseItem::new(
        ResponseStatus::Success,
        "Deleted task".to_string(),
        "Done with success".to_string(),
    ))
}
