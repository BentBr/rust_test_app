use crate::json_serialization::task::to_do_item::ToDoItem;
use crate::models::{task::item::Task, task_status::item::TaskStatus};
use serde::Serialize;
use std::vec::Vec;

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
            let to_do_item = ToDoItem::new(&item);

            match &item.status {
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
