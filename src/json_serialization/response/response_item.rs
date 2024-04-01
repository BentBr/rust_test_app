use crate::json_serialization::response::response_status::ResponseStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseItem<T>
where
    T: Serialize,
{
    pub status: ResponseStatus,
    pub message: String,
    pub data: T,
}

impl<T> ResponseItem<T>
where
    T: Serialize,
{
    pub fn new(status: ResponseStatus, message: String, data: T) -> ResponseItem<T> {
        ResponseItem {
            status,
            message,
            data,
        }
    }
}

#[cfg(test)]
mod response_item_tests {
    use super::ResponseItem;
    use crate::json_serialization::response::response_status::ResponseStatus;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestData {
        value: i32,
    }

    #[test]
    fn new() {
        let test_data = TestData { value: 42 };
        let response_item = ResponseItem::new(
            ResponseStatus::Success,
            "Test message".to_string(),
            test_data,
        );

        assert_eq!(response_item.status, ResponseStatus::Success);
        assert_eq!(response_item.message, "Test message");
        assert_eq!(response_item.data.value, 42);
    }

    #[test]
    fn serialize() {
        let test_data = TestData { value: 42 };
        let response_item = ResponseItem::new(
            ResponseStatus::Success,
            "Test message".to_string(),
            test_data,
        );

        let serialized = serde_json::to_string(&response_item).unwrap();
        let expected = r#"{"status":"Success","message":"Test message","data":{"value":42}}"#;

        assert_eq!(serialized, expected);
    }
}
