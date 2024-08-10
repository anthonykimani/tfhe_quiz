// @generated automatically by Diesel CLI.

diesel::table! {
    candidates (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        position_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    positions (id) {
        id -> Int4,
        position_name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    results (id) {
        id -> Int4,
        candidate_id -> Int4,
        position_id -> Int4,
        encrypted_tally -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        public_key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    votes (id) {
        id -> Int4,
        user_id -> Int4,
        candidate_id -> Int4,
        position_id -> Int4,
        encrypted_vote -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(candidates -> positions (position_id));
diesel::joinable!(results -> candidates (candidate_id));
diesel::joinable!(results -> positions (position_id));
diesel::joinable!(votes -> candidates (candidate_id));
diesel::joinable!(votes -> positions (position_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    candidates,
    positions,
    results,
    users,
    votes,
);
