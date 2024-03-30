use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::HttpResponse;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ResponseItem::new(
        ResponseStatus::Error,
        "Not found".to_string(),
        "Requested resource not found",
    ))
}
