-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    public_key VARCHAR NOT NULL,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null
)