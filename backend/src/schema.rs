// @generated automatically by Diesel CLI.

diesel::table! {
    booklistings (user_id, book_id) {
        user_id -> Uuid,
        book_id -> Text,
    }
}

diesel::table! {
    books (isbn) {
        title -> Text,
        author -> Text,
        embeddings -> Text,
        isbn -> Text,
    }
}

diesel::table! {
    socials (id) {
        id -> Uuid,
        platform -> Text,
        username -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        fullname -> Text,
    }
}

diesel::joinable!(booklistings -> users (user_id));
diesel::joinable!(socials -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    booklistings,
    books,
    socials,
    users,
);
