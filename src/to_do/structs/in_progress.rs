use super::super::enums::TaskStatus;
use super::super::structs::traits::create::Create;
use super::super::structs::traits::edit::Edit;
use super::super::structs::traits::get::Get;
use super::base::Base;

#[derive(Clone)]
pub struct InProgress {
    pub super_struct: Base,
}

impl InProgress {
    pub fn new(input_title: &str, creation_date: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::InProgress,
            creation_date: creation_date.to_string(),
        };

        InProgress { super_struct: base }
    }
}

impl Get for InProgress {}
impl Create for InProgress {}
impl Edit for InProgress {}
