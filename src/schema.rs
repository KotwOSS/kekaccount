table! {
    access_codes (id) {
        id -> Bytea,
        app_id -> Bytea,
        token_id -> Bytea,
    }
}

table! {
    app_tokens (id) {
        id -> Bytea,
        token -> Bytea,
        app_id -> Bytea,
        name -> Varchar,
        permissions -> Int2,
    }
}

table! {
    apps (id) {
        id -> Bytea,
        owner -> Bytea,
        name -> Varchar,
        avatar -> Varchar,
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
        name -> Varchar,
        avatar -> Varchar,
        email -> Varchar,
        password -> Bytea,
    }
}

table! {
    verifications (id) {
        id -> Bytea,
        owner -> Bytea,
    }
}

joinable!(access_codes -> apps (app_id));
joinable!(access_codes -> tokens (token_id));
joinable!(app_tokens -> apps (app_id));
joinable!(apps -> users (owner));
joinable!(tokens -> users (user_id));
joinable!(verifications -> users (owner));

allow_tables_to_appear_in_same_query!(
    access_codes,
    app_tokens,
    apps,
    tokens,
    users,
    verifications,
);
