use actix_web::HttpRequest;
use chrono::Utc;
use serde_json::value::Value;
use serde_json::Map;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};

pub async fn create(req: HttpRequest) -> String {
    let file_name: String = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);

    let title: String = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(
        &title.as_str(),
        TaskStatus::OPEN,
        Utc::now().to_string().as_str(),
    );

    process_input(item, "create".to_string(), &state);
    return format!("{} created", title);
}
