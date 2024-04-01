use crate::models::task::item::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewToDoItem {
    pub title: String,
    pub description: String,
}

impl NewToDoItem {
    pub fn new(input_item: Task) -> NewToDoItem {
        NewToDoItem {
            title: input_item.title.to_owned(),
            description: input_item.description.to_owned(),
        }
    }
}

#[cfg(test)]
mod new_to_do_item_tests {
    use super::NewToDoItem;
    use crate::models::task::item::Task;
    use crate::models::task_status::item::TaskStatus;

    #[test]
    fn new() {
        let task = Task {
            id: 0,
            uuid: Default::default(),
            title: "Test Task Title".to_string(),
            description: "Test Task Description".to_string(),
            status: TaskStatus::Open,
            creation_date: Default::default(),
            modification_date: Some(Default::default()),
            deletion_date: None,
            user_id: 0,
        };

        let new_item = NewToDoItem::new(task.clone());

        assert_eq!(new_item.title, task.title);
        assert_eq!(new_item.description, task.description);
    }

    #[test]
    fn serialize() {
        let new_item = NewToDoItem {
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
        };

        let serialized = serde_json::to_string(&new_item).expect("Failed to serialize");
        let expected_json = r#"{"title":"Test Title","description":"Test Description"}"#;

        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn deserialize() {
        let json = r#"{
           "title": "Deserialization Test",
            "description": "Testing"
        }"#;

        let deserialized: NewToDoItem = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(deserialized.title, "Deserialization Test");
        assert_eq!(deserialized.description, "Testing");
    }
}
