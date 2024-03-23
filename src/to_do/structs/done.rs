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
    pub fn new(
        uuid: Option<&str>,
        title: &str,
        description: &str,
        creation_date: &str,
        modification_date: Option<&str>,
        deletion_date: Option<&str>,
    ) -> Self {
        let base = Base {
            uuid: Some(uuid.unwrap().to_string()),
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Done,
            creation_date: creation_date.to_string(),
            modification_date: Some(modification_date.unwrap().to_string()),
            deletion_date: Some(deletion_date.unwrap().to_string()),
        };

        Done { super_struct: base }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
