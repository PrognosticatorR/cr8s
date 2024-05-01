-- Your SQL goes here
CREATE TABLE rustaceans (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
) 