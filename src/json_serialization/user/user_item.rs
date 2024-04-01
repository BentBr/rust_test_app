use crate::helpers::datetime::format_datetime;
use crate::models::user::item::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserItem {
    pub uuid: String,
    pub username: String,
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub creation_date: String,
    pub modification_date: Option<String>,
    pub deletion_date: Option<String>,
}

impl UserItem {
    pub fn new(input_item: User) -> UserItem {
        UserItem {
            uuid: input_item.uuid.to_string(),
            username: input_item.username.to_owned(),
            salutation: input_item.salutation.to_owned(),
            first_name: input_item.first_name.to_string(),
            last_name: input_item.last_name.to_string(),
            email: input_item.email.to_string(),
            creation_date: input_item.creation_date.to_string(),
            modification_date: format_datetime(input_item.modification_date),
            deletion_date: format_datetime(input_item.deletion_date),
        }
    }
}

#[cfg(test)]
mod user_item_tests {
    use super::UserItem;
    use crate::helpers::datetime::format_datetime;
    use crate::models::user::item::User;
    use chrono::{NaiveDateTime, Utc};
    use uuid::Uuid;

    fn create_test_user(uuid: Uuid, time: NaiveDateTime) -> User {
        User {
            id: 0,
            uuid,
            username: "test user".to_string(),
            salutation: "Mr.".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "johndoe@example.com".to_string(),
            password: "".to_string(),
            creation_date: time,
            modification_date: Some(time),
            deletion_date: None,
        }
    }

    #[test]
    fn new() {
        let time = Utc::now().naive_utc();
        let uuid = Uuid::new_v4();
        let test_user = create_test_user(uuid, time.clone());
        let user_item = UserItem::new(test_user.clone());

        assert_eq!(user_item.uuid, test_user.uuid.to_string());
        assert_eq!(user_item.username, test_user.username);
        assert_eq!(user_item.salutation, test_user.salutation);
        assert_eq!(user_item.first_name, test_user.first_name);
        assert_eq!(user_item.last_name, test_user.last_name);
        assert_eq!(user_item.email, test_user.email);
        assert_eq!(user_item.creation_date, time.to_string());
        assert_eq!(
            user_item.modification_date,
            format_datetime(test_user.modification_date)
        );
        assert_eq!(
            user_item.deletion_date,
            format_datetime(test_user.deletion_date)
        );
    }

    #[test]
    fn serialize() {
        use serde_json;

        let time =
            NaiveDateTime::parse_from_str("2022-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let uuid = Uuid::parse_str("6023454a-2dd5-495f-86ba-9523cf645396").unwrap();
        let test_user = create_test_user(uuid, time.clone());
        let user_item = UserItem::new(test_user);

        let serialized = serde_json::to_string(&user_item).unwrap();
        let expected = r#"{"uuid":"6023454a-2dd5-495f-86ba-9523cf645396","username":"test user","salutation":"Mr.","first_name":"John","last_name":"Doe","email":"johndoe@example.com","creation_date":"2022-01-01 00:00:00","modification_date":"2022-01-01 00:00:00","deletion_date":null}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        use serde_json;

        let json = r#"{
            "uuid": "6023454a-2dd5-495f-86ba-9523cf645396",
            "username": "test user",
            "salutation": "Mr.",
            "first_name": "John",
            "last_name": "Doe",
            "email": "johndoe@example.com",
            "creation_date": "2022-01-01 00:00:00",
            "modification_date": "2022-01-01 00:00:00",
            "deletion_date": null
        }"#;
        let deserialized: UserItem = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.uuid, "6023454a-2dd5-495f-86ba-9523cf645396");
        assert_eq!(deserialized.username, "test user");
        assert_eq!(deserialized.salutation, "Mr.");
        assert_eq!(deserialized.first_name, "John");
        assert_eq!(deserialized.last_name, "Doe");
        assert_eq!(deserialized.email, "johndoe@example.com");
        assert_eq!(deserialized.creation_date, "2022-01-01 00:00:00");
        assert_eq!(
            deserialized.modification_date,
            Some("2022-01-01 00:00:00".to_string())
        );
        assert_eq!(deserialized.deletion_date, None);
    }
}
