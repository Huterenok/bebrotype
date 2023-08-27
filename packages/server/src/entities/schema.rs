// @generated automatically by Diesel CLI.

diesel::table! {
    liked_texts (user_id, text_id) {
        user_id -> Int8,
        text_id -> Int8,
    }
}

diesel::table! {
    oauth_google_storage (id) {
        id -> Int8,
        csrf_state -> Text,
        pkce_code_verifier -> Text,
        return_url -> Text,
    }
}

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
    }
}

diesel::joinable!(liked_texts -> texts (text_id));
diesel::joinable!(liked_texts -> users (user_id));
diesel::joinable!(texts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    liked_texts,
    oauth_google_storage,
    texts,
    users,
);
