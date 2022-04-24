-- Your SQL goes here
CREATE TABLE users (
    id BYTEA PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    avatar VARCHAR(255) NOT NULL,
    email VARCHAR(32) NOT NULL,
    password BYTEA NOT NULL
);

-- Add case insensitive unique name
CREATE UNIQUE INDEX name_idx ON users ((lower(name)));
-- Add case insensitive unique email
CREATE UNIQUE INDEX email_idx ON users ((lower(email)));

CREATE TABLE tokens (
    id BYTEA PRIMARY KEY,
    token BYTEA NOT NULL,
    user_id BYTEA NOT NULL 
    REFERENCES users (id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL,
    permissions SMALLINT NOT NULL,
    expire TIMESTAMP WITH TIME ZONE DEFAULT NOW() + INTERVAL '5 minutes'
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

CREATE FUNCTION clean_account_db () 
RETURNS void 
AS $$
BEGIN
    DELETE FROM tokens WHERE expire < NOW();
END;
$$ LANGUAGE plpgsql;