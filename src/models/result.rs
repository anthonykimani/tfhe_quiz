use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResultModel {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: i32,
    pub completed_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewResult {
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: i32,
}