use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::task::items::fetch_items;
use actix_web::HttpResponse;
use serde_json::json;

pub async fn get() -> HttpResponse {
    // Loading them with default limit: 100
    let items = fetch_items(None);

    HttpResponse::Ok().json(ResponseItem::new(
        ResponseStatus::Success,
        format!("Fetched {} items", 100),
        json!(ToDoItems::new(items)).to_string(),
    ))
}
