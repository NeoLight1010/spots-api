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
        owner_id -> Int4,
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

allow_tables_to_appear_in_same_query!(
    group_users,
    groups,
    spots,
    users,
);
