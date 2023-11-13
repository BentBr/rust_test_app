use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;

pub trait Delete {
    fn delete(
        &self,
        title: &String,
        state: &mut Map<String, Value>
    ) {
        state.remove(title);
        let file_name = dotenv::var("STORAGE_FILE").unwrap();

        write_to_file(file_name.as_str(), state);
        println!("{} is being deleted", title);
    }
}
