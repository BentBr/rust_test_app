use serde::{Serialize, Serializer};
use std::fmt;

#[derive(PartialEq, Debug)]
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
                "Input '{}' not supported as at valid response status",
                input_string
            ),
        }
    }
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.stringify())
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

#[cfg(test)]
mod response_status_tests {
    use super::ResponseStatus;

    #[test]
    fn stringify() {
        let success_status = ResponseStatus::Success;
        let error_status = ResponseStatus::Error;

        assert_eq!(success_status.stringify(), "Success".to_string());
        assert_eq!(error_status.stringify(), "Error".to_string());
    }

    #[test]
    fn from_string() {
        let success_string = "Success".to_string();
        let error_string = "Error".to_string();

        assert_eq!(
            ResponseStatus::from_string(success_string),
            ResponseStatus::Success
        );
        assert_eq!(
            ResponseStatus::from_string(error_string),
            ResponseStatus::Error
        );
    }

    #[test]
    #[should_panic(expected = "Input 'fail' not supported as at valid response status")]
    fn from_string_panic() {
        let not_existing_status_string = "fail".to_string();

        ResponseStatus::from_string(not_existing_status_string);
    }

    #[test]
    fn display() {
        let success_status = ResponseStatus::Success;
        let error_status = ResponseStatus::Error;

        assert_eq!(format!("{}", success_status), "Success");
        assert_eq!(format!("{}", error_status), "Error");
    }

    #[test]
    fn serialize() {
        use serde_json;

        let success_status = ResponseStatus::Success;
        let error_status = ResponseStatus::Error;

        assert_eq!(
            serde_json::to_string(&success_status).unwrap(),
            "\"Success\""
        );
        assert_eq!(serde_json::to_string(&error_status).unwrap(), "\"Error\"");
    }
}
