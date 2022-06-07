table! {
    group_users (group_id, user_id) {
        group_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    spots (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        group_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(group_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    group_users,
    groups,
    spots,
    users,
);
