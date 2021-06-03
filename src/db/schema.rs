table! {
    users (id) {
        id -> Int4,
        is_admin -> Bool,
        username -> Varchar,
        email -> Varchar,
        token_key -> Text,
        password_hash -> Text,
        reset_token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
