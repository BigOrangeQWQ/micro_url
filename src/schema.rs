// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        salt -> Text,
        link -> Text,
    }
}
