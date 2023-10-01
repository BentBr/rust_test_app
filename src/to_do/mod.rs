pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::open::Open;

pub enum ItemTypes {
    Open(Open),
    Done(Done),
}

pub fn to_do_factory(title: &str, status: TaskStatus, creation_date: &str) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title, creation_date)),
        TaskStatus::OPEN => ItemTypes::Open(Open::new(title, creation_date)),
    }
}
