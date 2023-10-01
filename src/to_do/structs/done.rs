use super::super::enums::TaskStatus;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str, creation_date: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
            creation_date: creation_date.to_string(),
        };

        return Done { super_struct: base };
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
