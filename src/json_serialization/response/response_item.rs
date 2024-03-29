use crate::json_serialization::response::response_status::ResponseStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseItem<T>
where
    T: Serialize,
{
    pub status: ResponseStatus,
    pub message: String,
    pub data: T,
}

impl<T> ResponseItem<T>
where
    T: Serialize,
{
    pub fn new(status: ResponseStatus, message: String, data: T) -> ResponseItem<T> {
        ResponseItem {
            status,
            message,
            data,
        }
    }
}
