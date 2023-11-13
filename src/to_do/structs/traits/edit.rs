use std::collections::HashMap;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(
        &self,
        title: &String,
        data: &HashMap<String, String>,
        state: &mut Map<String, Value>,
    ) {
        state.insert(title.to_string(), json!(data));

        let file_name = dotenv::var("STORAGE_FILE").unwrap();

        write_to_file(file_name.as_str(), state);
        println!("{} is being set to done", title);
    }
    fn set_to_open(
        &self,
        title: &String,
        data: &HashMap<String, String>,
        state: &mut Map<String, Value>,
    ) {
        state.insert(title.to_string(), json!(data));

        let file_name = dotenv::var("STORAGE_FILE").unwrap();
        write_to_file(file_name.as_str(), state);
        println!("{} is being set to open", title);
    }
}
