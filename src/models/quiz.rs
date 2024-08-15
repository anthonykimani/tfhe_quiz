use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quiz {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::quizzes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQuiz {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
}