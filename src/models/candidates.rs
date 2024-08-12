use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::candidates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candidate {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub position_id: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::candidates)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewCandidate {
    pub name: String,
    pub email: String,
    pub position_id: i32,
    pub password: String,
}