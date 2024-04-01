use crate::models::task::item::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EditToDoItem {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

impl EditToDoItem {
    pub fn new(input_item: Task) -> EditToDoItem {
        EditToDoItem {
            uuid: input_item.uuid.to_string(),
            title: input_item.title.to_owned(),
            description: input_item.description.to_owned(),
            status: input_item.status.to_string(),
        }
    }
}

#[cfg(test)]
mod edit_to_do_item_tests {
    use super::EditToDoItem;
    use crate::models::task::item::Task;
    use crate::models::task_status::item::TaskStatus;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_edit_to_do_item_new_general_case() {
        let uuid = Uuid::new_v4();

        // General case test
        let task = Task {
            id: 0,
            uuid,
            title: "Test Task".to_string(),
            description: "This is a test description.".to_string(),
            status: TaskStatus::Done,
            creation_date: Utc::now().naive_utc(),
            modification_date: Some(Utc::now().naive_utc()),
            deletion_date: None,
            user_id: 0,
        };

        let edit_item = EditToDoItem::new(task.clone());

        assert_eq!(edit_item.uuid, task.uuid.to_string());
        assert_eq!(edit_item.title, task.title);
        assert_eq!(edit_item.description, task.description);
        assert_eq!(edit_item.status, TaskStatus::Done.to_string());
    }

    #[test]
    fn test_edit_to_do_item_serialization() {
        let uuid = Uuid::new_v4();

        let edit_item = EditToDoItem {
            uuid: uuid.to_string(),
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            status: "Test Status".to_string(),
        };

        let serialized = serde_json::to_string(&edit_item).expect("Failed to serialize");

        let expected_json = format!(
            r#"{{"uuid":"{}","title":"Test Title","description":"Test Description","status":"Test Status"}}"#,
            uuid
        );

        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_edit_to_do_item_deserialization() {
        let json = r#"
        {
            "uuid": "123e4567-e89b-12d3-a456-426614174000",
            "title": "Deserialization Test",
            "description": "Testing",
            "status": "Done"
        }"#;

        let deserialized: EditToDoItem = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(deserialized.uuid, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(deserialized.title, "Deserialization Test");
        assert_eq!(deserialized.description, "Testing");
        assert_eq!(deserialized.status, "Done");
    }
}
