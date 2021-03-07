table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
