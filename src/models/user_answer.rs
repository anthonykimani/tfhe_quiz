use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAnswer {
    pub id: i32,
    pub user_id: i32,
    pub question_id: i32,
    pub answer_id: i32,
    pub selected_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserAnswer {
    pub id: i32,
    pub user_id: i32,
    pub question_id: i32,
    pub answer_id: i32,
}