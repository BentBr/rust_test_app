use actix_web::{HttpRequest, HttpResponse};
use serde_json::{Map, Value};
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;

pub async fn delete(req: HttpRequest) -> HttpResponse {
    let file_name = dotenv::var("STORAGE_FILE").unwrap();
    let state: Map<String, Value> = read_file(&file_name);

    let title: String = req.match_info().get("title").unwrap().to_string();

    return match state.get(&title) {
        Some(result) => {
            let status_string: String = result["status"].to_string().trim_matches('"').to_string();
            let creation_date: String = result["creation_date"].to_string().trim_matches('"').to_string();

            let item = to_do_factory(&title, TaskStatus::from_string(status_string), &creation_date);
            process_input(&item, "delete".to_owned(), &state);

            HttpResponse::Ok().json(format!("Deleted item \"{}\"", &title))
        },
        None => {
            HttpResponse::NotFound().json(
                format!("{} not found in state for deletion", &title)
            )
        }
    }
}
