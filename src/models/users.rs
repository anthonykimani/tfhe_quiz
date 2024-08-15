use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub public_key: String,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub public_key: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  email: String,
    pub  password: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AuthSuccess {
    pub message: String,
    pub status: String,
    pub token: String,
    pub user_reference: String,
}