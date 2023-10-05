use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::{HttpRequest, Responder};

pub async fn get_one(req: HttpRequest) -> impl Responder {
    ToDoItems::get_one_state(req)
}
