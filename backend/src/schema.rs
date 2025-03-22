// @generated automatically by Diesel CLI.

diesel::table! {
    booklistings (user_id, book_id) {
        user_id -> Uuid,
        book_id -> Int8,
    }
}

diesel::table! {
    books (isbn) {
        isbn -> Int8,
        title -> Text,
        author -> Text,
        embeddings -> Text,
        user_id -> Uuid,
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

diesel::joinable!(booklistings -> books (book_id));
diesel::joinable!(booklistings -> users (user_id));
diesel::joinable!(books -> users (user_id));
diesel::joinable!(socials -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    booklistings,
    books,
    socials,
    users,
);
