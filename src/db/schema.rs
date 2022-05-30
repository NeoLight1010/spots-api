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
    groups,
    spots,
    users,
);
