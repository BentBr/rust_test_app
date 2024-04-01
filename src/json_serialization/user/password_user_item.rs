use crate::models::user::item::PasswordUser;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PasswordUserItem {
    pub uuid: String,
    pub old_password: String,
    pub new_password: String,
}

impl PasswordUserItem {
    pub fn new(input_item: PasswordUser) -> PasswordUserItem {
        PasswordUserItem {
            uuid: input_item.uuid.to_string(),
            old_password: input_item.password.to_owned(),
            new_password: input_item.password.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordUserItem;
    use crate::models::user::item::PasswordUser;
    use uuid::Uuid;

    #[test]
    fn new() {
        let uuid = Uuid::new_v4();
        let password_user = PasswordUser {
            id: 0,
            uuid,
            password: "old_password".to_string(),
        };

        let password_user_item = PasswordUserItem::new(password_user);

        assert_eq!(password_user_item.uuid, uuid.to_string());
        assert_eq!(password_user_item.old_password, "old_password");
        assert_eq!(password_user_item.new_password, "old_password");
    }

    #[test]
    fn serialize() {
        use serde_json;

        let password_user_item = PasswordUserItem {
            uuid: "72655de0-21e6-40f0-9856-9530344bf78d".to_string(),
            old_password: "old_password".to_string(),
            new_password: "new_password".to_string(),
        };

        let serialized = serde_json::to_string(&password_user_item).unwrap();
        let expected = r#"{"uuid":"72655de0-21e6-40f0-9856-9530344bf78d","old_password":"old_password","new_password":"new_password"}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        use serde_json;

        let json = r#"{"uuid":"72655de0-21e6-40f0-9856-9530344bf78d","old_password":"old_password","new_password":"new_password"}"#;
        let deserialized: PasswordUserItem = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.uuid, "72655de0-21e6-40f0-9856-9530344bf78d");
        assert_eq!(deserialized.old_password, "old_password");
        assert_eq!(deserialized.new_password, "new_password");
    }
}
