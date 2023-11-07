use super::super::enums::TaskStatus;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
    pub creation_date: String,
}
