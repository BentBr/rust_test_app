use super::to_do::structs::done::Done;
use super::to_do::structs::open::Open;
use super::to_do::traits::create::Create;
use super::to_do::traits::delete::Delete;
use super::to_do::traits::edit::Edit;
use super::to_do::traits::get::Get;
use super::to_do::ItemTypes;
use chrono::prelude::*;
use serde_json::value::Value;
use serde_json::Map;
use std::collections::HashMap;

fn process_open(item: Open, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => {
            let mut data: HashMap<String, String> = HashMap::new();
            data.insert("creation_date".to_string(), Utc::now().to_string());
            data.insert("status".to_string(), item.super_struct.status.stringify());

            item.create(&item.super_struct.title, &data, &mut state)
        }
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("Command {} not supported", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_open(&item.super_struct.title, &mut state),
        _ => println!("Command {} not supported", command),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Open(item) => process_open(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
