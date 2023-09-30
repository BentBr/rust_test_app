use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

#[derive(Serialize)]
pub struct ToDoItems {
    pub open_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub open_items_count: i8,
    pub done_items_count: i8
}

impl ToDoItems {
    pub fn new (input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut open_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Open(packed) => open_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct)
            }
        }

        let open_count: i8 = open_array_buffer.len() as i8;
        let done_count: i8 = done_array_buffer.len() as i8;

        return ToDoItems {
            open_items: open_array_buffer,
            open_items_count: open_count,
            done_items: done_array_buffer,
            done_items_count: done_count
        }
    }
}