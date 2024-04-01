use crate::helpers::datetime::format_datetime;
use crate::models::task::item::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ToDoItem {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub creation_date: String,
    pub modification_date: Option<String>,
    pub deletion_date: Option<String>,
}

impl ToDoItem {
    pub fn new(input_item: Task) -> ToDoItem {
        ToDoItem {
            uuid: input_item.uuid.to_string(),
            title: input_item.title.to_owned(),
            description: input_item.description.to_owned(),
            status: input_item.status.to_string(),
            creation_date: input_item.creation_date.to_string(),
            modification_date: format_datetime(input_item.modification_date),
            deletion_date: format_datetime(input_item.deletion_date),
        }
    }
}

#[cfg(test)]
mod to_do_item_tests {
    use super::ToDoItem;
    use crate::helpers::datetime::format_datetime;
    use crate::models::task::item::Task;
    use crate::models::task_status::item::TaskStatus;
    use chrono::{NaiveDateTime, Utc};
    use uuid::Uuid;

    #[test]
    fn new() {
        let task = Task {
            id: 0,
            uuid: Uuid::new_v4(),
            title: "Test Task Title".to_string(),
            description: "Test Task Description".to_string(),
            status: TaskStatus::Open,
            creation_date: Utc::now().naive_utc(),
            modification_date: Some(Utc::now().naive_utc()),
            deletion_date: None,
            user_id: 0,
        };

        let to_do_item = ToDoItem::new(task.clone());

        assert_eq!(to_do_item.uuid, task.uuid.to_string());
        assert_eq!(to_do_item.title, task.title);
        assert_eq!(to_do_item.description, task.description);
        assert_eq!(to_do_item.status, task.status.to_string());
        assert_eq!(to_do_item.creation_date, task.creation_date.to_string());
        assert_eq!(
            to_do_item.modification_date,
            format_datetime(task.modification_date)
        );
        assert_eq!(
            to_do_item.deletion_date,
            format_datetime(task.deletion_date)
        );
    }

    #[test]
    fn serialize() {
        let time =
            NaiveDateTime::parse_from_str("2022-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let to_do_item = ToDoItem {
            uuid: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            status: "Test Status".to_string(),
            creation_date: time.to_string(),
            modification_date: Some(time.to_string()),
            deletion_date: None,
        };

        let serialized = serde_json::to_string(&to_do_item).expect("Failed to serialize");
        let expected_json = r#"{"uuid":"123e4567-e89b-12d3-a456-426614174000","title":"Test Title","description":"Test Description","status":"Test Status","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null}"#.to_string();

        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn deserialize() {
        let json = r#"
        {
            "uuid": "123e4567-e89b-12d3-a456-426614174000",
            "title": "Deserialization Test",
            "description": "Testing",
            "status": "Active",
            "creation_date": "2022-05-15T10:00:00",
            "modification_date": "2022-05-16T11:00:00",
            "deletion_date": null
        }"#;

        let deserialized: ToDoItem = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(deserialized.uuid, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(deserialized.title, "Deserialization Test");
        assert_eq!(deserialized.description, "Testing");
        assert_eq!(deserialized.status, "Active");
        assert_eq!(deserialized.creation_date, "2022-05-15T10:00:00");
        assert_eq!(
            deserialized.modification_date,
            Some("2022-05-16T11:00:00".to_string())
        );
        assert!(deserialized.deletion_date.is_none());
    }
}
