use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::cmp::Eq;

#[derive(Clone)]
pub enum TaskStatus {
    Done,
    Open,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::Done => "Done".to_string(),
            Self::Open => "Open".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "Done" => TaskStatus::Done,
            "Open" => TaskStatus::Open,
            _ => panic!("input {} not supported", input_string),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Done => {
                write!(f, "Done")
            }
            Self::Open => {
                write!(f, "Open")
            }
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.stringify().as_str())
    }
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        self.stringify() == other.stringify()
    }
}
impl Eq for TaskStatus {
}
