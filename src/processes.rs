use super::to_do::structs::done::Done;
use super::to_do::structs::open::Open;
use super::to_do::traits::create::Create;
use super::to_do::traits::delete::Delete;
use super::to_do::traits::edit::Edit;
use super::to_do::traits::get::Get;
use super::to_do::ItemTypes;
use serde_json::value::Value;
use serde_json::Map;

fn process_open(item: Open, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(
            &item.super_struct.title,
            &item.super_struct.status.to_string(),
            &mut state,
        ),
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
