// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}
