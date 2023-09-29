use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(result) => {
                println!("Item is being fetched: {}", title);
                println!("Status: {}", result);
            }
            None => println!("Item: {} was not found", title),
        }
    }
}
