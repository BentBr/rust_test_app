use crate::database::DB;
use crate::helpers::uuid::parse_uuid_from_request;
use actix_web::{HttpRequest, HttpResponse};
use sentry::Level;
use uuid::Uuid;

use crate::json_serialization::response::response_item::ResponseItem;
use crate::json_serialization::response::response_status::ResponseStatus;
use crate::json_serialization::task::to_do_item::ToDoItem;
use crate::jwt::JwToken;
use crate::models::task::item::{done_item, in_progress_item, open_item};
use crate::models::task_status::item::TaskStatus;

pub async fn open(request: HttpRequest, db: DB, token: JwToken) -> HttpResponse {
    transition_into(request, TaskStatus::Open, token.user_uuid, db)
}

pub async fn in_progress(request: HttpRequest, db: DB, token: JwToken) -> HttpResponse {
    transition_into(request, TaskStatus::InProgress, token.user_uuid, db)
}

pub async fn done(request: HttpRequest, db: DB, token: JwToken) -> HttpResponse {
    transition_into(request, TaskStatus::Done, token.user_uuid, db)
}

fn transition_into(
    request: HttpRequest,
    status: TaskStatus,
    user_uuid: Uuid,
    db: DB,
) -> HttpResponse {
    let uuid: Uuid = match parse_uuid_from_request(request) {
        Err(response) => return response,
        Ok(valid_uuid) => valid_uuid,
    };

    // Transitioning in DB
    let item = match status {
        TaskStatus::Open => open_item(uuid, user_uuid, db),
        TaskStatus::InProgress => in_progress_item(uuid, user_uuid, db),
        TaskStatus::Done => done_item(uuid, user_uuid, db),
    };

    match item.first() {
        Some(item) => HttpResponse::Ok().json(ResponseItem::new(
            ResponseStatus::Success,
            format!("Transitioned task to '{}'", status),
            ToDoItem::new(item.clone()),
        )),
        None => {
            // Logging a bit
            sentry::capture_message(
                "Transition and lookup of changed item failed!",
                Level::Error,
            );

            HttpResponse::NotFound().json(ResponseItem::new(
                ResponseStatus::Error,
                format!("Error during task transition to {}", status),
                "Could not find it",
            ))
        }
    }
}
