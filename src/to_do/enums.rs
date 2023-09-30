use std::fmt;
use serde::ser::{Serialize, Serializer};

pub enum TaskStatus {
    DONE,
    OPEN,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::OPEN => "OPEN".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "OPEN" => TaskStatus::OPEN,
            _ => panic!("input {} not supported", input_string),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }
            &Self::OPEN => {
                write!(f, "OPEN")
            }
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}