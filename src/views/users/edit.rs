use crate::database::DB;
use crate::helpers::email::parse_email_from_string;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::edit_user_item::EditUserItem;
use crate::json_serialization::user::password_user_item::PasswordUserItem;
use crate::json_serialization::user::user_item::UserItem;
use crate::models::user::item::{edit_item, update_password};
use actix_web::{web, HttpRequest, HttpResponse};
use sentry::Level;
use uuid::Uuid;

pub async fn edit(
    user_item: web::Json<EditUserItem>,
    request: HttpRequest,
    db: DB,
) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    let username = String::from(&user_item.username);
    let email = String::from(&user_item.email);
    let salutation = String::from(&user_item.salutation);
    let first_name = String::from(&user_item.first_name);
    let last_name = String::from(&user_item.last_name);

    let valid_email: String = match parse_email_from_string(email) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Editing in DB
    let item = edit_item(
        uuid,
        username,
        valid_email,
        salutation,
        first_name,
        last_name,
        db,
    );

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Updated user".to_string(),
            UserItem::new(item.clone()),
        )),
        None => {
            // Logging a bit
            sentry::capture_message("Editing and lookup of changed user failed!", Level::Error);

            HttpResponse::NotFound().json(ResponseItem::new(
                ResponseStatus::Error,
                "User not found for".to_string(),
                user_item,
            ))
        }
    }
}

pub async fn password(
    user_item: web::Json<PasswordUserItem>,
    request: HttpRequest,
    db: DB,
    db2: DB,
) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    let new_password = String::from(&user_item.password);
    if new_password.is_empty() {
        return HttpResponse::UnprocessableEntity().json(ResponseItem::new(
            ResponseStatus::Error,
            "Password constraint".to_string(),
            "Must not be empty",
        ));
    }

    // Editing in DB
    let item = update_password(uuid, new_password, db, db2);

    match item {
        Some(user) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Updated user".to_string(),
            UserItem::new(user),
        )),
        None => HttpResponse::Conflict().json(ResponseItem::new(
            ResponseStatus::Error,
            "User not found or password wrong".to_string(),
            user_item,
        )),
    }
}
