use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vote {
    pub id: i32,
    pub user_id: i32,
    pub candidate_id: i32,
    pub position_id: i32,
    pub encrypted_vote: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewVote {
    pub user_id: i32,
    pub candidate_id: i32,
    pub position_id: i32,
    pub encrypted_vote: String,
}