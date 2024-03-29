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
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        username -> Varchar,
        salutation -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        creation_date -> Timestamp,
        modification_date -> Nullable<Timestamp>,
        deletion_date -> Nullable<Timestamp>,
    }
}

diesel::joinable!(to_do -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(to_do, users,);
