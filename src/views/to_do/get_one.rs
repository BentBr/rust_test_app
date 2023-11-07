use crate::json_serialization::to_do_item::ToDoItem;
use actix_web::{HttpRequest, Responder};

pub async fn get_one(req: HttpRequest) -> impl Responder {
    ToDoItem::get_state(req)
}
