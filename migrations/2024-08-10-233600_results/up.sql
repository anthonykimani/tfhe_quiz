-- Your SQL goes here
CREATE TABLE results (
    id SERIAL PRIMARY KEY,
    candidate_id INTEGER NOT NULL REFERENCES candidates(id),
    position_id INTEGER NOT NULL REFERENCES positions(id),
    encrypted_tally VARCHAR NOT NULL,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null
)