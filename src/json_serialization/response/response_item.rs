use crate::json_serialization::response::response_status::ResponseStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseItem {
    pub status: ResponseStatus,
    pub message: String,
    pub data: String,
}

impl ResponseItem {
    pub fn new(status: ResponseStatus, message: String, data: String) -> ResponseItem {
        ResponseItem {
            status,
            message,
            data,
        }
    }
}
