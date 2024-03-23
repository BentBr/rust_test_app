pub mod enums;
pub mod structs;

use crate::schema::to_do::{deletion_date, modification_date};
use chrono::NaiveDateTime;
use enums::TaskStatus;
use structs::done::Done;
use structs::in_progress::InProgress;
use structs::open::Open;

#[derive(Clone)]
pub enum ItemTypes {
    Open(Open),
    Done(Done),
    InProgress(InProgress),
}

pub fn to_do_factory(
    uuid: &str,
    title: &str,
    description: &str,
    status: TaskStatus,
    creation_date: NaiveDateTime,
    modification_date: Option<NaiveDateTime>,
    deletion_date: Option<NaiveDateTime>,
) -> ItemTypes {
    match status {
        TaskStatus::Done => ItemTypes::Done(Done::new(title, creation_date)),
        TaskStatus::Open => ItemTypes::Open(Open::new(title, creation_date)),
        TaskStatus::InProgress => ItemTypes::InProgress(InProgress::new(title, creation_date)),
    }
}
