use crate::json_serialization::to_do_item::ToDoItem;
use actix_web::{Responder, web};

pub async fn create(to_do_item: web::Json<ToDoItem>) -> impl Responder {
    ToDoItem::create_state(to_do_item)
}
