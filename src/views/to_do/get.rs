use crate::database::DB;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::task::items::fetch_items;
use actix_web::HttpResponse;

pub async fn get(db: DB) -> HttpResponse {
    // Loading them with default limit: 100
    let items = fetch_items(None, db);

    HttpResponse::Ok().json(ResponseItem::new(
        ResponseStatus::Success,
        format!("Fetched {} items", items.len()),
        ToDoItems::new(items),
    ))
}
