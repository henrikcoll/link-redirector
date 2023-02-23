// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        slug -> Varchar,
        url -> Text,
    }
}
