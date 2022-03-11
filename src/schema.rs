table! {
    tokens (id) {
        id -> Bytea,
        user_id -> Bytea,
        name -> Varchar,
        permissions -> Int2,
    }
}

table! {
    users (id) {
        id -> Bytea,
        username -> Varchar,
        email -> Varchar,
        password -> Bytea,
    }
}

allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
