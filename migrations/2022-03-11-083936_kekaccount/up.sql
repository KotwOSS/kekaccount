-- Your SQL goes here
CREATE TABLE users (
    id BYTEA PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE,
    avatar VARCHAR(255) NOT NULL,
    email VARCHAR(32) NOT NULL,
    password BYTEA NOT NULL
);

CREATE TABLE tokens (
    id BYTEA PRIMARY KEY,
    token BYTEA NOT NULL,
    user_id BYTEA NOT NULL 
    REFERENCES users (id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL,
    permissions SMALLINT NOT NULL
);

CREATE TABLE apps (
    id BYTEA PRIMARY KEY,
    owner BYTEA NOT NULL 
    REFERENCES users (id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL,
    avatar VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    redirect_uri VARCHAR(255) NOT NULL,
    homepage VARCHAR(255) NOT NULL
);

CREATE TABLE app_tokens (
    id BYTEA PRIMARY KEY,
    token BYTEA NOT NULL,
    app_id BYTEA NOT NULL 
    REFERENCES apps (id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL,
    permissions SMALLINT NOT NULL
);

CREATE TABLE verifications (
    id BYTEA PRIMARY KEY,
    owner BYTEA NOT NULL 
    REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE access_codes (
    id BYTEA PRIMARY KEY,
    app_id BYTEA NOT NULL 
    REFERENCES apps (id) ON DELETE CASCADE,
    token_id BYTEA NOT NULL 
    REFERENCES tokens (id) ON DELETE CASCADE
);