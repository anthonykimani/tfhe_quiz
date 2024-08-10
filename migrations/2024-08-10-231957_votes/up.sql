-- Your SQL goes here
CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    candidate_id INTEGER NOT NULL REFERENCES candidates(id),
    position_id INTEGER NOT NULL REFERENCES positions(id),
    encrypted_vote VARCHAR NOT NULL,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null
)