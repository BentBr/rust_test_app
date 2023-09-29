use std::fmt;

pub enum TaskStatus {
    DONE,
    OPEN,
}

// Basically the same as the implementation of Display for Taskstatus, but we have to invoke it:
// TaskStatus::DONE._to_string() vs TaskStatus::DONE
impl TaskStatus {
    pub fn _stringify(&self) -> String {
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
