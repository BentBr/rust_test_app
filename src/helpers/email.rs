use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::HttpResponse;
use regex::Regex;

pub fn parse_email_from_string(email: String) -> Result<String, HttpResponse> {
    // Remember to use the same here as for the database constraint (users table)
    // See 2024-03-30-223226_user_email_constraint
    let email_regex =
        Regex::new(r"(?i)^[^@]+@[A-Za-z0-9üäöß-]+(\.[A-Za-z0-9üäöß-]+)*\.[A-Za-z]{2,4}$");

    match email_regex {
        Err(_) => {
            panic!("Regex string is invalid!");
        }
        Ok(regex) => {
            if regex.is_match(&email) {
                return Ok(email.clone());
            }

            Err(HttpResponse::UnprocessableEntity().json(ResponseItem::new(
                ResponseStatus::Error,
                "Email format error".to_string(),
                format!("Email '{}' is not a proper email format", email),
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use super::parse_email_from_string;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;
    use serde_json::Value;

    #[test]
    fn test_parse_valid_email() {
        let invalid_emails: Vec<&str> = vec![
            "bla@uiae.de",
            "uiae@uae-uiüa.uiae",
            "brÜggemaünnn@uae.de",
            "brÜggemaünnn@ußae.de",
            "brÜggemÜaßünnn@uae.de",
            "brÜggemaün-%nn@u-.uaeae.de",
        ];

        for email in invalid_emails {
            match parse_email_from_string(email.to_string()) {
                Ok(parsed_email) => assert_eq!(parsed_email, email),
                Err(_) => panic!(
                    "Valid email '{}' was incorrectly not identified as valid.",
                    email
                ),
            }
        }
    }

    #[actix_rt::test]
    async fn test_parse_invalid_email() {
        let invalid_emails: Vec<&str> = vec![
            "uae",
            "üötat@uae...de",
            "mail@domain....com",
            "1+ia@@@dom.ii.ain.com",
            "@domain.com",
            "mail@@domain.com",
        ];

        for email in invalid_emails {
            match parse_email_from_string(email.to_string()) {
                Ok(_) => panic!(
                    "Invalid email '{}' was incorrectly identified as not invalid.",
                    email
                ),
                Err(response) => {
                    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

                    let body_bytes = to_bytes(response.into_body()).await.unwrap();
                    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

                    assert_eq!(body_json["message"], "Email format error");
                    assert_eq!(
                        body_json["data"],
                        format!("Email '{}' is not a proper email format", email)
                    );
                }
            }
        }
    }
}
