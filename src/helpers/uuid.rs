use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::{HttpRequest, HttpResponse};
use std::str::FromStr;
use uuid::Uuid;

pub fn parse_uuid_from_request(request: HttpRequest) -> Result<Uuid, HttpResponse> {
    let uuid_option = request.match_info().get("uuid");
    let uuid_string: &str;

    match uuid_option {
        Some(uuid) => {
            uuid_string = uuid;
        }
        None => {
            return Err(HttpResponse::BadRequest().json(ResponseItem::new(
                ResponseStatus::Error,
                "Uuid parsing error".to_string(),
                "Check your uuid for it's format. Must be uuid_v4".to_string(),
            )))
        }
    }
    let uuid_result = Uuid::from_str(uuid_string);

    match uuid_result {
        Err(error) => Err(HttpResponse::InternalServerError().json(ResponseItem::new(
            ResponseStatus::Error,
            "Uuid has an error".to_string(),
            error.to_string(),
        ))),
        Ok(valid_uuid) => Ok(valid_uuid),
    }
}
