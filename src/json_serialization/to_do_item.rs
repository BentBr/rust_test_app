use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::{ItemTypes, to_do_factory};

#[derive(Deserialize, Serialize)]
pub struct ToDoItem {
    pub creation_date: String,
    pub status: String,
    pub title: String,
}

impl ToDoItem {
    pub fn new(input_item: ItemTypes) -> ToDoItem {
        let status: String;
        let creation_date: String;
        let title: String;
        
        match input_item {
            ItemTypes::Open(packed) => {
                title = packed.super_struct.title;
                creation_date = packed.super_struct.creation_date;
                status = packed. super_struct.status.to_string();
            },
            ItemTypes::Done(packed) => {
                title = packed.super_struct.title;
                creation_date = packed.super_struct.creation_date;
                status = packed. super_struct.status.to_string();
            },
        }

        ToDoItem {
            creation_date,
            title,
            status,
        }
    }
    
    pub fn get_state() -> ToDoItem {
        let file_name = dotenv::var("STORAGE_FILE").unwrap();
        let state: Map<String, Value> = read_file(&file_name);
        let mut array_buffer = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value["status"].as_str().unwrap().to_string());
            let item = to_do_factory(&key, status, value["creation_date"].as_str().unwrap());
            array_buffer.push(item);
        }

        ToDoItem::new(input_item)
    }
}