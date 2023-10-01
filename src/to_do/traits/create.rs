use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
use std::collections::HashMap;

use crate::state::write_to_file;

pub trait Create {
    fn create(
        &self,
        title: &String,
        data: &HashMap<String, String>,
        state: &mut Map<String, Value>,
    ) {
        state.insert(title.to_string(), json!(data));
        let file_name = dotenv::var("STORAGE_FILE").unwrap();

        write_to_file(file_name.as_str(), state);
        println!("{} is being created", title);
    }
}
