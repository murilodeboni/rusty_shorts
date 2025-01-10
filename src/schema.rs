// @generated automatically by Diesel CLI.

diesel::table! {
    urls (slug) {
        slug -> Varchar,
        original_url -> Text,
    }
}
