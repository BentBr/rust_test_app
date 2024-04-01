use crate::models::user::item::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EditUserItem {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
}

impl EditUserItem {
    pub fn new(input_item: User) -> EditUserItem {
        EditUserItem {
            uuid: input_item.uuid.to_string(),
            username: input_item.username.to_owned(),
            salutation: input_item.salutation.to_owned(),
            first_name: input_item.first_name.to_string(),
            last_name: input_item.last_name.to_string(),
            email: input_item.email.to_string(),
        }
    }
}

#[cfg(test)]
mod edit_user_item_tests {
    use super::EditUserItem;
    use crate::models::user::item::User;
    use uuid::Uuid;

    #[test]
    fn new() {
        let uuid = Uuid::new_v4();
        let test_user = User {
            id: 0,
            uuid,
            username: "john_doe".to_string(),
            email: "johndoe@example.com".to_string(),
            password: "secure password".to_string(),
            salutation: "Mr.".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            creation_date: Default::default(),
            modification_date: None,
            deletion_date: None,
        };

        let edit_item = EditUserItem::new(test_user);

        assert_eq!(edit_item.uuid, uuid.to_string());
        assert_eq!(edit_item.username, "john_doe");
        assert_eq!(edit_item.email, "johndoe@example.com");
        assert_eq!(edit_item.salutation, "Mr.");
        assert_eq!(edit_item.first_name, "John");
        assert_eq!(edit_item.last_name, "Doe");
    }

    #[test]
    fn serialize() {
        let edit_item = EditUserItem {
            uuid: "72655de0-21e6-40f0-9856-9530344bf78d".to_string(),
            username: "john_doe".to_string(),
            email: "johndoe@example.com".to_string(),
            salutation: "Mr.".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };

        let serialized = serde_json::to_string(&edit_item).unwrap();
        let expected = r#"{"uuid":"72655de0-21e6-40f0-9856-9530344bf78d","username":"john_doe","email":"johndoe@example.com","salutation":"Mr.","first_name":"John","last_name":"Doe"}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        let json = r#"{"uuid":"72655de0-21e6-40f0-9856-9530344bf78d","username":"john_doe","email":"johndoe@example.com","salutation":"Mr.","first_name":"John","last_name":"Doe"}"#;
        let deserialized: EditUserItem = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.uuid, "72655de0-21e6-40f0-9856-9530344bf78d");
        assert_eq!(deserialized.username, "john_doe");
        assert_eq!(deserialized.email, "johndoe@example.com");
        assert_eq!(deserialized.salutation, "Mr.");
        assert_eq!(deserialized.first_name, "John");
        assert_eq!(deserialized.last_name, "Doe");
    }
}
