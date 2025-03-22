diesel::table! {
    users {
        id -> Uuid,
        username -> Text,
        password -> Text,
        fullname -> Text,
    }
}
