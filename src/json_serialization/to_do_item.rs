use std::fmt;
use std::fmt::{Debug, Formatter};
use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType, web};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::processes::process_input;
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
    
    pub fn get_state(req: HttpRequest) -> HttpResponse {
        let file_name = dotenv::var("STORAGE_FILE").unwrap();
        let state: Map<String, Value> = read_file(&file_name);

        let title: String = req.match_info().get("title").unwrap().to_string();

        return match state.get(&title) {
            Some(result) => {
                let status_string: String = result["status"].to_string().trim_matches('"').to_string();
                let creation_date: String = result["creation_date"].to_string().trim_matches('"').to_string();

                let item = to_do_factory(&title, TaskStatus::from_string(status_string), &creation_date);

                HttpResponse::Ok().json(ToDoItem::new(item))
            },
            None => {
                HttpResponse::NotFound().json(
                    format!("{} not found in state", &title)
                )
            }
        }
    }

    pub fn create_state(item: web::Json<ToDoItem>) -> ToDoItem {

        let item = to_do_factory(
            item.title.as_str(),
            TaskStatus::Open,
            Utc::now().to_string().as_str(),
        );

        // Writing to file
        let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
        let state: Map<String, Value> = read_file(&file_name);
        process_input(&item, "create".to_string(), &state);

        ToDoItem::new(item)
    }
}

impl Responder for ToDoItem {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
