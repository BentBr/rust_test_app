use super::super::enums::TaskStatus;
use super::super::structs::traits::delete::Delete;
use super::super::structs::traits::edit::Edit;
use super::super::structs::traits::get::Get;
use super::base::Base;

#[derive(Clone)]
pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str, creation_date: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Done,
            creation_date: creation_date.to_string(),
        };

        Done { super_struct: base }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
