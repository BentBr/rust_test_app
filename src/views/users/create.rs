use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::new_user_item::NewUserItem;
use crate::json_serialization::user::user_item::UserItem;
use crate::models::user::new_item::create_item;
use actix_web::{web, HttpResponse};
use sentry::Level;

pub async fn create(new_user_item: web::Json<NewUserItem>, db: DB) -> HttpResponse {
    let username = String::from(&new_user_item.username);
    let email = String::from(&new_user_item.email);
    let password = String::from(&new_user_item.password);

    // Creating in DB
    let item = create_item(username, email, password, db);

    match item.first() {
        Some(item) => HttpResponse::Created().json(ResponseItem::new(
            ResponseStatus::Success,
            "Created new user".to_string(),
            UserItem::new(item),
        )),
        None => {
            // Logging a bit
            sentry::capture_message("Storing and lookup of new item failed!", Level::Error);

            HttpResponse::Conflict().json(ResponseItem::new(
                ResponseStatus::Error,
                "Error during user lookup and creation".to_string(),
                new_user_item,
            ))
        }
    }
}
