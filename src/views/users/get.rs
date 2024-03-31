use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::user::user_items::UserItems;
use crate::models::user::items::fetch_items;
use actix_web::HttpResponse;
use crate::jwt::JwToken;

pub async fn get(db: DB, _: JwToken) -> HttpResponse {
    // Loading them with default limit: 100
    let items = fetch_items(None, db);

    HttpResponse::Ok().json(ResponseItem::new(
        ResponseStatus::Success,
        format!("Fetched {} user items", items.len()),
        UserItems::new(items),
    ))
}
