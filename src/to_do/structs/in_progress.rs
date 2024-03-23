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
    pub fn new(
        title: &str,
        description: &str,
        creation_date: &str,
        modification_date: Option<&str>,
    ) -> Self {
        let base = Base {
            uuid: None,
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Done,
            creation_date: creation_date.to_string(),
            modification_date: Some(modification_date.unwrap().to_string()),
            deletion_date: None,
        };
        InProgress { super_struct: base }
    }
}

impl Get for InProgress {}
impl Create for InProgress {}
impl Edit for InProgress {}
