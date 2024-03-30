use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::error::JsonPayloadError;
use actix_web::{error, HttpRequest, HttpResponse};

pub(crate) fn json_error_handler(
    err: JsonPayloadError,
    _req: &HttpRequest,
) -> actix_web::error::Error {
    let error_message = match &err {
        JsonPayloadError::ContentType => "Content type error".to_string(),
        JsonPayloadError::Deserialize(json_error) => {
            format!("JSON deserialize error: {}", json_error)
        }
        _ => "Unknown error".to_string(),
    };

    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(ResponseItem::new(
            ResponseStatus::Error,
            "JSON error".to_string(),
            error_message,
        )),
    )
    .into()
}
