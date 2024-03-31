use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::auth::login::Login;
use crate::jwt::JwToken;
use crate::models::user::item::fetch_user_by_login;
use actix_web::{web, HttpResponse};
use std::collections::HashMap;

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let password = credentials.password.clone();
    let email = credentials.email.clone();

    match fetch_user_by_login(email, password, db) {
        None => HttpResponse::Forbidden().json(ResponseItem::new(
            ResponseStatus::Error,
            "Login failed".to_string(),
            "Check credentials: Email and password",
        )),
        Some(user) => {
            let new_token = JwToken::new(user.uuid);
            let raw_token = new_token.encode();
            let mut body = HashMap::new();
            body.insert("token", raw_token);

            HttpResponse::Created().json(ResponseItem::new(
                ResponseStatus::Success,
                "Session token created".to_string(),
                body,
            ))
        }
    }
}
