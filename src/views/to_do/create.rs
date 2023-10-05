use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::{HttpRequest, Responder};

pub async fn create(req: HttpRequest) -> impl Responder {
    ToDoItems::create_state(req)
}
