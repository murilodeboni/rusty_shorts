// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        slug -> Varchar,
        original_url -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    urls
);
