use std::time::SystemTime;
use super::super::enums::TaskStatus;
use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

pub struct Open {
    pub super_struct: Base,
}

impl Open {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::OPEN,
            creation_date: SystemTime::now(),
        };

        return Open { super_struct: base };
    }
}

impl Get for Open {}
impl Create for Open {}
impl Edit for Open {}
