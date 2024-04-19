use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::error::JsonPayloadError;
use actix_web::{error, HttpRequest, HttpResponse};

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::error::Error {
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

#[cfg(test)]
mod tests {
    use super::json_error_handler;
    use actix_web::error::JsonPayloadError;
    use actix_web::test;
    use serde::de::Error;

    #[actix_rt::test]
    async fn test_json_error_handler() {
        let req = test::TestRequest::default().to_http_request();

        // Test ContentType error
        let err = JsonPayloadError::ContentType;
        let error = json_error_handler(err, &req);
        let expected_error_message = "Content type error".to_string();
        assert_eq!(expected_error_message, error.to_string());

        // Test Deserialize error
        let err = JsonPayloadError::Deserialize(Error::custom("custom error"));
        let error = json_error_handler(err, &req);
        let expected_error_message = "Json deserialize error: custom error".to_string();
        assert_eq!(expected_error_message, error.to_string());
    }
}
