use actix_web::{HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use bcrypt::{DEFAULT_COST, hash, verify};
use serde_json::json;
use crate::db_operations::users::{add_user, get_a_user_by_mail};
use crate::models::app_state::AppState;
use crate::models::users::{LoginForm, NewUser, RegisterForm};

async fn handle_register_error(error: &str) -> HttpResponse {
    let response_body = json!({
        "status": "error",
        "message": error,
    });
    HttpResponse::Ok().content_type(ContentType::json()).body(response_body.to_string())
}

pub async fn login_user(json: web::Json<LoginForm>, state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_a_user_by_mail(&mut *connection_guard, json.email.clone());
    match user_exist {
        Some(user) => {
            if verify(&json.password, &user.password).unwrap_or(false) {
                println!("user_email:{:?}", &json.email);
                Ok(HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "success",
                        "message": "Login successful.",
                        "redirect_url": "/dashboard"
                    })))
            } else {
                // Error response for wrong password
                Ok(HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "error",
                        "message": "Wrong password."
                    })))
            }
        }
        None => {
            // Error response for email not found
            Ok(HttpResponse::NotFound()
                .content_type(ContentType::json())
                .json(json!({
                    "status": "error",
                    "message": "Email not found."
                })))
        }
    }
}

pub async fn register_user(state: web::Data<AppState>, json: web::Json<RegisterForm>) -> HttpResponse {
    println!("Data is {:#?}", json);

    if json.name.is_empty() || json.email.is_empty() || json.password.is_empty() {
        println!("Empty fields detected");
        return handle_register_error("All fields are required").await;
    }

    println!("All fields have content");

    // Hash and salt the password
    let hashed_password = match hash(&json.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(er) => {
            println!("error hashing password: {}", er);
            return HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .json(json!({
                    "status": "error",
                    "message": "Error hashing password."
                }));
        }
    };

    let new_user = NewUser {
        name: json.name.clone(),
        email: json.email.clone(),
        public_key: "hello".to_string(),
        password: hashed_password,
    };

    let mut connection_guard = state.db_connection.lock().unwrap();
    let res = add_user(new_user, &mut *connection_guard);

    match res {
        Ok(user) => {
            HttpResponse::Created()
                .content_type(ContentType::json())
                .json(json!({
                    "status": "success",
                    "message": "Account created, please login to continue."
                }))
        }
        Err(err) => {
            println!("db error {:#?}", err);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .json(json!({
                    "status": "error",
                    "message": "Error creating account."
                }))
        }
    }
}
