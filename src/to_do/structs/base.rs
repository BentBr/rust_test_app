use super::super::enums::TaskStatus;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
    pub creation_date: SystemTime
}
