use super::super::enums::TaskStatus;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Base {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub creation_date: String,
    pub modification_date: Option<String>,
    pub deletion_date: Option<String>,
}
