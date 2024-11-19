// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Text,
        filename -> Text,
        content_type -> Text,
        size -> Integer,
        created_at -> Timestamp,
        data -> Binary,
    }
}
