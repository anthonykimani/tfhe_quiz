// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        answer_text -> Text,
        is_correct -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        quiz_id -> Int4,
        question_text -> Text,
        question_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    quizzes (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    results (id) {
        id -> Int4,
        user_id -> Int4,
        quiz_id -> Int4,
        score -> Int4,
        completed_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_answers (id) {
        id -> Int4,
        user_id -> Int4,
        question_id -> Int4,
        answer_id -> Int4,
        selected_at -> Timestamp,
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

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(questions -> quizzes (quiz_id));
diesel::joinable!(quizzes -> users (user_id));
diesel::joinable!(results -> quizzes (quiz_id));
diesel::joinable!(results -> users (user_id));
diesel::joinable!(user_answers -> answers (answer_id));
diesel::joinable!(user_answers -> questions (question_id));
diesel::joinable!(user_answers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
    quizzes,
    results,
    user_answers,
    users,
);
