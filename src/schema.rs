table! {
    apps (id) {
        id -> Bytea,
        owner -> Bytea,
        name -> Varchar,
        description -> Varchar,
        redirect_uri -> Varchar,
        homepage -> Varchar,
    }
}

table! {
    tokens (id) {
        id -> Bytea,
        token -> Bytea,
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
    apps,
    tokens,
    users,
);
