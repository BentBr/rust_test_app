use super::super::enums::TaskStatus;
use super::super::structs::traits::create::Create;
use super::super::structs::traits::edit::Edit;
use super::super::structs::traits::get::Get;
use super::base::Base;

#[derive(Clone)]
pub struct Open {
    pub super_struct: Base,
}

impl Open {
    pub fn new(input_title: &str, creation_date: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Open,
            creation_date: creation_date.to_string(),
        };

        Open { super_struct: base }
    }
}

impl Get for Open {}
impl Create for Open {}
impl Edit for Open {}
