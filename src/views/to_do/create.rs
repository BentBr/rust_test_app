use crate::json_serialization::to_do_item::ToDoItem;
use actix_web::{HttpRequest, Responder};

pub async fn create(req: HttpRequest) -> impl Responder {
    ToDoItem::create_state(req)
}
