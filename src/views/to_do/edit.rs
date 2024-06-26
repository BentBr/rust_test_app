use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::task::edit_to_do_item::EditToDoItem;
use crate::json_serialization::task::to_do_item::ToDoItem;
use crate::jwt::JwToken;
use crate::models::task::item::edit_item;
use crate::models::task_status::item::TaskStatus;
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

pub async fn edit(
    to_do_item: web::Json<EditToDoItem>,
    request: HttpRequest,
    db: DB,
    token: JwToken,
) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    let title = String::from(&to_do_item.title);
    let description = String::from(&to_do_item.description);
    let status = TaskStatus::from_string(to_do_item.status.clone());

    let valid_status = match status {
        Some(status) => status,
        None => {
            return HttpResponse::UnprocessableEntity().json(ResponseItem::new(
                ResponseStatus::Error,
                "Status error".to_string(),
                format!("Status '{}' is not valid!", &to_do_item.status),
            ))
        }
    };

    // Editing in DB
    let item = edit_item(uuid, token.user_uuid, title, description, valid_status, db);

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            "Updated task".to_string(),
            ToDoItem::new(item.clone()),
        )),
        None => HttpResponse::NotFound().json(ResponseItem::new(
            ResponseStatus::Error,
            "Task not found for".to_string(),
            to_do_item,
        )),
    }
}
