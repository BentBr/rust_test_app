pub mod enums;
pub mod structs;

use enums::TaskStatus;
use structs::done::Done;
use structs::open::Open;
use structs::in_progress::InProgress;

#[derive(Clone)]
pub enum ItemTypes {
    Open(Open),
    Done(Done),
    InProgress(InProgress),
}

pub fn to_do_factory(title: &str, status: TaskStatus, creation_date: &str) -> ItemTypes {
    match status {
        TaskStatus::Done => ItemTypes::Done(Done::new(title, creation_date)),
        TaskStatus::Open => ItemTypes::Open(Open::new(title, creation_date)),
        TaskStatus::InProgress => ItemTypes::InProgress(InProgress::new(title, creation_date)),
    }
}
