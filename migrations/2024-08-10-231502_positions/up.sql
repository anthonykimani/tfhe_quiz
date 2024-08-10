-- Your SQL goes here
CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    position_name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null
)