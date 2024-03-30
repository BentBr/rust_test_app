use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::HttpResponse;
use regex::Regex;

pub fn parse_email_from_string(email: String) -> Result<String, HttpResponse> {
    // Remember to use the same here as for the database constraint (users table)
    // See 2024-03-30-223226_user_email_constraint
    let email_regex = Regex::new(
        r"(?i)^[A-Za-z0-9._%-üäöß]+@[A-Za-z0-9üäöß-]+(\.[A-Za-z0-9üäöß-]+)*\.[A-Za-z]{2,4}$",
    );

    match email_regex {
        Err(error) => {
            sentry::capture_error(&error);

            Err(HttpResponse::InternalServerError().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during parsing".to_string(),
                error.to_string(),
            )))
        }
        Ok(regex) => {
            if regex.is_match(&email) {
                return Ok(email.clone());
            }

            Err(HttpResponse::UnprocessableEntity().json(ResponseItem::new(
                ResponseStatus::Error,
                "Email format error".to_string(),
                "Email field has not a proper email format",
            )))
        }
    }
}
