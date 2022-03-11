-- Your SQL goes here
CREATE TABLE users (
    id BYTEA PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    email VARCHAR(32) NOT NULL,
    password BYTEA NOT NULL
);

CREATE TABLE tokens (
    id BYTEA PRIMARY KEY,
    user_id BYTEA NOT NULL,
    name VARCHAR(32) NOT NULL,
    permissions SMALLINT NOT NULL
);