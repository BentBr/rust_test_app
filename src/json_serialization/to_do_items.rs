use crate::processes::process_input;
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;
use serde_json::value::Value;
use serde_json::Map;
use std::vec::Vec;

use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, structs::base::Base, to_do_factory, ItemTypes};

#[derive(Serialize)]
pub struct ToDoItems {
    pub open_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub open_items_count: i8,
    pub done_items_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut open_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Open(packed) => open_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let open_count: i8 = open_array_buffer.len() as i8;
        let done_count: i8 = done_array_buffer.len() as i8;

        ToDoItems {
            open_items: open_array_buffer,
            open_items_count: open_count,
            done_items: done_array_buffer,
            done_items_count: done_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        let file_name = dotenv::var("STORAGE_FILE").unwrap();
        let state: Map<String, Value> = read_file(&file_name);
        let mut array_buffer = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value["status"].as_str().unwrap().to_string());
            let item = to_do_factory(&key, status, value["creation_date"].as_str().unwrap());
            array_buffer.push(item);
        }

        ToDoItems::new(array_buffer)
    }

    pub fn get_one_state(req: HttpRequest) -> ToDoItems {
        let file_name = dotenv::var("STORAGE_FILE").unwrap();
        let state: Map<String, Value> = read_file(&file_name);
        let mut array_buffer = Vec::new();

        let title = req.match_info().get("title").unwrap().to_string();
        for (key, value) in state {
            if key == title {
                let status = TaskStatus::from_string(value["status"].as_str().unwrap().to_string());
                let item = to_do_factory(&title, status, value["creation_date"].as_str().unwrap());
                array_buffer.push(item);

                break;
            }
        }

        ToDoItems::new(array_buffer)
    }

    pub fn create_state(req: HttpRequest) -> ToDoItems {
        let mut array_buffer = Vec::new();
        let title: String = req.match_info().get("title").unwrap().to_string();

        let item = to_do_factory(
            title.as_str(),
            TaskStatus::Open,
            Utc::now().to_string().as_str(),
        );

        // Writing to file
        let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
        let state: Map<String, Value> = read_file(&file_name);
        process_input(&item, "create".to_string(), &state);

        array_buffer.push(item);

        ToDoItems::new(array_buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
