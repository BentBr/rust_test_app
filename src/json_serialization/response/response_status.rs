use serde::{Serialize, Serializer};
use std::fmt;

pub enum ResponseStatus {
    Success,
    Error,
}

impl ResponseStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::Success => "Success".to_string(),
            Self::Error => "Error".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "Success" => Self::Success,
            "Error" => Self::Error,
            _ => panic!(
                "input '{}' not supported as at valid response status",
                input_string
            ),
        }
    }
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Success => {
                write!(f, "Success")
            }
            Self::Error => {
                write!(f, "Error")
            }
        }
    }
}

impl Serialize for ResponseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.stringify().as_str())
    }
}
