use crate::helpers::env::get_int_from_env;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::{env, fmt};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwToken {
    pub user_uuid: Uuid,
    #[serde(with = "ts_seconds")]
    pub minted: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

impl JwToken {
    pub fn get_key() -> String {
        env::var("APP_SECRET").expect("APP_SECRET must be set in environment")
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());

        encode(&Header::default(), &self, &key).expect("Token encoding failed")
    }

    pub fn new(user_uuid: Uuid) -> Self {
        let timestamp = Utc::now();
        let expiration_timestamp = Utc::now().add(get_session_lifetime());

        JwToken {
            user_uuid,
            minted: timestamp,
            exp: expiration_timestamp,
        }
    }

    pub fn from_token(token: String) -> Option<Self> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token, &key, &Validation::new(Algorithm::HS256));

        match token_result {
            Ok(data) => Some(data.claims),
            Err(error) => {
                sentry::capture_error(&error);

                None
            }
        }
    }
}

impl FromRequest for JwToken {
    type Error = UnauthorizedError;
    type Future = Ready<Result<JwToken, UnauthorizedError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);

                match token_result {
                    Some(token) => ok(token),
                    None => err(UnauthorizedError::new(
                        "Token cannot be decoded".to_string(),
                    )),
                }
            }
            None => err(UnauthorizedError::new(
                "Token not in header under key 'token'".to_string(),
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UnauthorizedError {
    message: String,
}

impl UnauthorizedError {
    pub fn new(message: String) -> UnauthorizedError {
        UnauthorizedError { message }
    }
}

impl Display for UnauthorizedError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for UnauthorizedError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(ResponseItem::new(
            ResponseStatus::Error,
            "JSON error".to_string(),
            &self.message,
        ))
    }
}

fn get_session_lifetime() -> TimeDelta {
    let lifetime_in_seconds = get_int_from_env("SESSION_LIFETIME".to_string());

    Duration::try_seconds(lifetime_in_seconds as i64)
        .expect("Duration calculation failed for token expiring")
}

#[cfg(test)]
mod tests {
    use super::{get_session_lifetime, JwToken, UnauthorizedError};
    use actix_web::dev::Payload;
    use actix_web::http::header::{HeaderName, HeaderValue};
    use actix_web::{test, FromRequest, ResponseError};
    use chrono::Utc;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use std::env;
    use uuid::Uuid;

    #[test]
    async fn new_token() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            || {
                let uuid = Uuid::new_v4();
                let token = JwToken::new(uuid);

                assert_eq!(token.user_uuid, uuid);
                assert!(token.minted <= Utc::now());
                assert!(token.exp > token.minted);
            },
        );
    }

    #[test]
    async fn encode_token() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            || {
                let uuid = Uuid::new_v4();
                let token = JwToken::new(uuid);
                let encoded = token.encode();

                assert!(!encoded.is_empty());
            },
        );
    }

    #[test]
    async fn encode_decode_token() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            || {
                let uuid = Uuid::new_v4();
                let original_token = JwToken::new(uuid);
                let encoded = original_token.clone().encode();

                // Manually decode the token
                let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
                let decoding_result =
                    decode::<JwToken>(&encoded, &key, &Validation::new(Algorithm::HS256));

                match decoding_result {
                    Ok(decoded_token_data) => {
                        let decoded_token = decoded_token_data.claims;
                        // Assert that the decoded token matches the original one
                        assert_eq!(decoded_token.user_uuid, original_token.clone().user_uuid);
                        assert_eq!(
                            decoded_token.minted.to_string(),
                            original_token
                                .clone()
                                .minted
                                .format("%Y-%m-%d %H:%M:%S %Z")
                                .to_string()
                        );
                        assert_eq!(
                            decoded_token.exp.to_string(),
                            original_token
                                .clone()
                                .exp
                                .format("%Y-%m-%d %H:%M:%S %Z")
                                .to_string()
                        );
                    }
                    Err(_) => panic!("Token decoding failed"),
                }
            },
        );
    }

    #[test]
    async fn from_token() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            || {
                let uuid = Uuid::new_v4();
                let original_token = JwToken::new(uuid);
                let encoded = original_token.clone().encode();

                // Use from_token to decode the token
                let decoded_token = JwToken::from_token(encoded).expect("Token decoding failed");

                // Assert that the decoded token matches the original one
                assert_eq!(decoded_token.user_uuid, original_token.user_uuid);
                assert_eq!(
                    decoded_token.minted.to_string(),
                    original_token
                        .minted
                        .format("%Y-%m-%d %H:%M:%S %Z")
                        .to_string()
                );
                assert_eq!(
                    decoded_token.exp.to_string(),
                    original_token
                        .exp
                        .format("%Y-%m-%d %H:%M:%S %Z")
                        .to_string()
                );
            },
        );
    }

    #[test]
    async fn new_unauthorized_error() {
        let message = "Test error message".to_string();
        let error = UnauthorizedError::new(message.clone());

        assert_eq!(error.message, message);
    }

    #[test]
    async fn display_unauthorized_error() {
        let message = "Test error message".to_string();
        let error = UnauthorizedError::new(message.clone());

        assert_eq!(format!("{}", error), message);
    }

    #[test]
    async fn get_key() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            || {
                assert_eq!(JwToken::get_key(), env::var("APP_SECRET").unwrap());
            },
        );
    }

    #[test]
    #[should_panic(expected = "APP_SECRET must be set in environment")]
    async fn get_key_not_set() {
        temp_env::with_vars_unset([("APP_SECRET"), ("SESSION_LIFETIME")], || {
            JwToken::get_key();
        });
    }

    #[test]
    async fn from_request_token_valid() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            move || {
                actix_rt::task::spawn_blocking(move || {
                    let system = actix_rt::System::new();
                    system.block_on(async {
                        let uuid = Uuid::new_v4();
                        let token_instance = JwToken::new(uuid);
                        let valid_token = token_instance.encode();

                        let header_name = HeaderName::from_static("token");
                        let header_value = HeaderValue::from_str(valid_token.as_str()).unwrap();

                        let request = test::TestRequest::default()
                            .insert_header((header_name, header_value))
                            .to_http_request();

                        // Call the from_request method
                        let token_result =
                            JwToken::from_request(&request, &mut Payload::None).await;

                        // Check if the token is valid
                        match token_result {
                            Ok(token) => assert_eq!(token.encode(), valid_token),
                            Err(_) => panic!("Token is not valid"),
                        }
                    });
                });
            },
        );
    }

    #[test]
    async fn from_request_token_invalid() {
        temp_env::with_vars(
            [
                ("APP_SECRET", Some("test_secret")),
                ("SESSION_LIFETIME", Some("3600")),
            ],
            || {
                actix_rt::task::spawn_blocking(|| {
                    let system = actix_rt::System::new();

                    system.block_on(async {
                        let header_name = HeaderName::from_static("token");
                        let header_value = HeaderValue::from_static("invalid_token");

                        let request = test::TestRequest::default()
                            .insert_header((header_name, header_value))
                            .to_http_request();

                        // Call the from_request method
                        let token_result =
                            JwToken::from_request(&request, &mut Payload::None).await;

                        // Check if the token is valid
                        match token_result {
                            Ok(_) => panic!("Token is valid but must not be"),
                            Err(err) => {
                                assert_eq!(err.message, "Token cannot be decoded".to_string())
                            }
                        }
                    });
                });
            },
        );
    }

    #[test]
    async fn from_request_token_not_present() {
        let req = test::TestRequest::default().to_http_request();
        let token = JwToken::from_request(&req, &mut Payload::None).await;

        assert!(token.is_err());
    }

    #[test]
    #[should_panic(expected = "SESSION_LIFETIME must be set in environment as unsigned int")]
    async fn get_session_lifetime_not_set() {
        temp_env::with_vars_unset([("APP_SECRET"), ("SESSION_LIFETIME")], || {
            get_session_lifetime();
        });
    }

    #[test]
    async fn unauthorized_error_response() {
        let error = UnauthorizedError::new("Test error message".to_string());
        let response = error.error_response();

        assert_eq!(
            response.status().as_str(),
            http::StatusCode::UNAUTHORIZED.as_str()
        );
    }
}
