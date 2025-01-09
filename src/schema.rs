// @generated automatically by Diesel CLI.

diesel::table! {
    urls (slug) {
        slug -> Varchar,
        original_url -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    urls,
);
