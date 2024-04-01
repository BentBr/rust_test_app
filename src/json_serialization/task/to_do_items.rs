use crate::json_serialization::task::to_do_item::ToDoItem;
use crate::models::{task::item::Task, task_status::item::TaskStatus};
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub open_items: Vec<ToDoItem>,
    pub done_items: Vec<ToDoItem>,
    pub in_progress_items: Vec<ToDoItem>,
    pub open_items_count: i8,
    pub done_items_count: i8,
    pub in_progress_items_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<Task>) -> ToDoItems {
        let mut open_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();
        let mut in_progress_array_buffer = Vec::new();

        for item in input_items {
            let to_do_item = ToDoItem::new(item.clone());

            match item.status {
                TaskStatus::Open => open_array_buffer.push(to_do_item),
                TaskStatus::Done => done_array_buffer.push(to_do_item),
                TaskStatus::InProgress => in_progress_array_buffer.push(to_do_item),
            }
        }

        let open_count: i8 = open_array_buffer.len() as i8;
        let done_count: i8 = done_array_buffer.len() as i8;
        let in_progress_count: i8 = in_progress_array_buffer.len() as i8;

        ToDoItems {
            open_items: open_array_buffer,
            open_items_count: open_count,
            done_items: done_array_buffer,
            done_items_count: done_count,
            in_progress_items: in_progress_array_buffer,
            in_progress_items_count: in_progress_count,
        }
    }
}
#[cfg(test)]
mod to_do_items_tests {
    use super::*;
    use chrono::{NaiveDateTime, Utc};
    use uuid::Uuid;

    #[test]
    fn new() {
        let task_open = Task {
            id: 0,
            uuid: Uuid::new_v4(),
            title: "Open Task".to_string(),
            description: "Description of open task".to_string(),
            status: TaskStatus::Open,
            creation_date: Default::default(),
            modification_date: None,
            deletion_date: None,
            user_id: 0,
        };
        let task_done = Task {
            id: 0,
            uuid: Uuid::new_v4(),
            title: "Done Task".to_string(),
            description: "Description of done task".to_string(),
            status: TaskStatus::Done,
            creation_date: Default::default(),
            modification_date: None,
            deletion_date: None,
            user_id: 0,
        };
        let task_in_progress = Task {
            id: 0,
            uuid: Uuid::new_v4(),
            title: "In Progress Task".to_string(),
            description: "Description of in progress task".to_string(),
            status: TaskStatus::InProgress,
            creation_date: Default::default(),
            modification_date: Some(Utc::now().naive_utc()),
            deletion_date: None,
            user_id: 0,
        };

        let input_tasks = vec![task_open, task_done, task_in_progress];

        let to_do_items = ToDoItems::new(input_tasks);

        assert_eq!(to_do_items.open_items_count, 1);
        assert_eq!(to_do_items.done_items_count, 1);
        assert_eq!(to_do_items.in_progress_items_count, 1);
        assert_eq!(to_do_items.open_items[0].title, "Open Task");
        assert_eq!(to_do_items.done_items[0].title, "Done Task");
        assert_eq!(to_do_items.in_progress_items[0].title, "In Progress Task");
    }

    #[test]
    fn serialize() {
        let uuid = "123e4567-e89b-12d3-a456-426614174000".to_string();
        let time =
            NaiveDateTime::parse_from_str("2022-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let open_item = ToDoItem {
            uuid: uuid.clone(),
            title: "Open Item".to_string(),
            description: "Description of open item".to_string(),
            status: "Open".to_string(),
            creation_date: time.clone().to_string(),
            modification_date: Some(time.clone().to_string()),
            deletion_date: None,
        };
        let done_item = ToDoItem {
            uuid: uuid.clone(),
            title: "Done Item".to_string(),
            description: "Description of done item".to_string(),
            status: "Done".to_string(),
            creation_date: time.clone().to_string(),
            modification_date: Some(time.clone().to_string()),
            deletion_date: None,
        };
        let in_progress_item = ToDoItem {
            uuid,
            title: "In Progress Item".to_string(),
            description: "Description of in progress item".to_string(),
            status: "In Progress".to_string(),
            creation_date: time.clone().to_string(),
            modification_date: Some(time.clone().to_string()),
            deletion_date: None,
        };

        let to_do_items = ToDoItems {
            open_items: vec![open_item],
            done_items: vec![done_item],
            in_progress_items: vec![in_progress_item],
            open_items_count: 1,
            done_items_count: 1,
            in_progress_items_count: 1,
        };

        let serialized = serde_json::to_string(&to_do_items).expect("Failed to serialize");
        let expected_json = r#"{"open_items":[{"uuid":"123e4567-e89b-12d3-a456-426614174000","title":"Open Item","description":"Description of open item","status":"Open","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null}],"done_items":[{"uuid":"123e4567-e89b-12d3-a456-426614174000","title":"Done Item","description":"Description of done item","status":"Done","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null}],"in_progress_items":[{"uuid":"123e4567-e89b-12d3-a456-426614174000","title":"In Progress Item","description":"Description of in progress item","status":"In Progress","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null}],"open_items_count":1,"done_items_count":1,"in_progress_items_count":1}"#;

        assert_eq!(serialized, expected_json);
    }
}
