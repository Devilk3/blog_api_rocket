// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        created_by -> Integer,
        title -> Text,
        body -> Text,
    }
}

diesel::table! {
    posts_tags (id) {
        id -> Nullable<Integer>,
        fk_post_id -> Integer,
        tag -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}

diesel::joinable!(posts -> users (created_by));
diesel::joinable!(posts_tags -> posts (fk_post_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    posts_tags,
    users,
);
