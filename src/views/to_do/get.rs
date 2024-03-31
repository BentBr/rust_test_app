use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::task::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::models::task::items::fetch_items;
use actix_web::HttpResponse;

pub async fn get(db: DB, token: JwToken) -> HttpResponse {
    // Loading them with default limit: 100
    let items = fetch_items(None, token.user_uuid, db);

    HttpResponse::Ok().json(ResponseItem::new(
        ResponseStatus::Success,
        format!("Fetched {} task items", items.len()),
        ToDoItems::new(items),
    ))
}
