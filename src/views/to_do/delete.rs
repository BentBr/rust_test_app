use crate::json_serialization::to_do_item::ToDoItem;
use actix_web::{HttpRequest, Responder};

pub async fn delete(req: HttpRequest) -> impl Responder {
    ToDoItem::delete(req)
}
