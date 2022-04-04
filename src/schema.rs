table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        is_deleted -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
);