-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS access_codes;
DROP TABLE IF EXISTS tokens;
DROP TABLE IF EXISTS app_tokens;
DROP TABLE IF EXISTS verifications;
DROP TABLE IF EXISTS apps;
DROP TABLE IF EXISTS users;
DROP FUNCTION clean_account_db;