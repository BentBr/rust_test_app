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

#[derive(Debug, Serialize, Deserialize)]
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
                println!("Error during decoding: {}", error);

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
// Implement `ResponseError` for `UnauthorizedError`
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
    let session_lifetime = env::var("SESSION_LIFETIME")
        .expect("SESSION_LIFETIME must be set in environment")
        .to_string();
    let lifetime_in_seconds = match session_lifetime.clone().parse::<i64>() {
        Ok(time) => time,
        Err(error) => {
            panic!("Lifetime parsing failed! {}", error);
        }
    };

    Duration::try_seconds(lifetime_in_seconds)
        .expect("Duration calculation failed for token expiring")
}
