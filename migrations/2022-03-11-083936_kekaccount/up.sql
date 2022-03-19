-- Your SQL goes here
CREATE TABLE users (
    id BYTEA PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    email VARCHAR(32) NOT NULL,
    password BYTEA NOT NULL
);

CREATE TABLE tokens (
    id BYTEA PRIMARY KEY,
    token BYTEA NOT NULL,
    user_id BYTEA NOT NULL,
    name VARCHAR(32) NOT NULL,
    permissions SMALLINT NOT NULL
);

CREATE TABLE app_tokens (
    id BYTEA PRIMARY KEY,
    token BYTEA NOT NULL,
    app_id BYTEA NOT NULL,
    name VARCHAR(32) NOT NULL,
    permissions SMALLINT NOT NULL
);

CREATE TABLE apps (
    id BYTEA PRIMARY KEY,
    owner BYTEA NOT NULL,
    name VARCHAR(32) NOT NULL,
    description VARCHAR(255) NOT NULL,
    redirect_uri VARCHAR(255) NOT NULL,
    homepage VARCHAR(255) NOT NULL
);

CREATE TABLE verifications (
    id BYTEA PRIMARY KEY,
    owner BYTEA NOT NULL
);

CREATE TABLE access_codes (
    id BYTEA PRIMARY KEY,
    app_id BYTEA NOT NULL,
    token_id BYTEA NOT NULL
);