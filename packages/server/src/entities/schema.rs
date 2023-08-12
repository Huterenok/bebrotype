// @generated automatically by Diesel CLI.

diesel::table! {
    texts (id) {
        id -> Int8,
        title -> Text,
        content -> Text,
        likes -> Int4,
        user_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        email -> Text,
        password -> Text,
        avatar -> Nullable<Text>,
        near_address -> Nullable<Text>,
        favourite_texts -> Array<Nullable<Int8>>,
    }
}

diesel::joinable!(texts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    texts,
    users,
);
