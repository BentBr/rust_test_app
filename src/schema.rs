// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Status;

    to_do (id) {
        id -> Int4,
        uuid -> Uuid,
        title -> Varchar,
        description -> Text,
        status -> Status,
        creation_date -> Timestamp,
        modification_date -> Nullable<Timestamp>,
        deletion_date -> Nullable<Timestamp>,
    }
}
