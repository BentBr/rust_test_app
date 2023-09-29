use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;

pub async fn get() -> impl Responder {
    let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);

    return web::Json(state);
}
