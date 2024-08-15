use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub answer_text: String,
    pub is_correct: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAnswer {
    pub id: i32,
    pub question_id: i32,
    pub answer_text: String,
    pub is_correct: bool,
}