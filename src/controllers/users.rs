use actix_web::{HttpResponse, web};
use actix_web::http::header::ContentType;
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde_json::json;
use crate::db_operations::users::{add_user, get_a_user_by_mail};
use crate::models::app_state::AppState;
use crate::models::users::{Claims, LoginForm, NewUser, RegisterForm};
use std::env;
use base64::{Engine as _, engine::general_purpose};
use dotenv::dotenv;
use jsonwebtoken::errors::ErrorKind;

async fn handle_register_error(error: &str) -> HttpResponse {
    let response_body = json!({
        "status": "error",
        "message": error,
    });
    HttpResponse::Ok().content_type(ContentType::json()).body(response_body.to_string())
}

fn check_base64_length_and_padding(encoded: &str) -> bool {
    let length = encoded.len();
    let valid_length = length % 4 == 0;
    let ends_with_padding = encoded.ends_with('=') || encoded.ends_with("==");

    length == 44 && valid_length && ends_with_padding
}


pub fn create_jwt(user_id: i32) -> String {
    dotenv().ok();
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}


pub async fn login_user(json: web::Json<LoginForm>, state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_a_user_by_mail(&mut *connection_guard, json.email.clone());
    match user_exist {
        Some(user) => {
            let token = create_jwt(user.id);
            if verify(&json.password, &user.password).unwrap_or(false) {
                Ok(HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "success",
                        "message": "Login successful.",
                        "user_reference" : &user.id.to_string(),
                        "token": token,
                        "email": &user.email.to_string(),
                        "username": &user.name.to_string(),
                    })))
            } else {
                // Error response for wrong password
                Ok(HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "error",
                        "message": "Invalid Logins"
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
pub fn verify_user(token: &str, user_reference: &str) -> Result<HttpResponse, HttpResponse> {

    match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256)) {
        Ok(token_data) => {
            println!("{:?}",token_data);
            // Compare the `user_reference` from the token with the provided reference
            if token_data.claims.sub == user_reference {
                Ok(HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "success",
                        "message": "User verified."
                    })))
            } else {
                Err(HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "error",
                        "message": "Invalid user reference."
                    })))
            }
        }
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => {
                // Token is invalid (e.g., wrong signature)
                Err(HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "error",
                        "message": "Invalid token."
                    })))
            }
            ErrorKind::ExpiredSignature => {
                // Token has expired
                Err(HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .json(json!({
                        "status": "error",
                        "message": "Token has expired."
                    })))
            }
            _ => {
                // Other errors
                Err(HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .json(json!({
                        "error": err.to_string(),
                        "message": "Token verification failed."
                    })))
            }
        },
    }
}

pub async fn auth_verify(token: String, user_reference: String) -> HttpResponse {
    verify_user(&token, &user_reference).unwrap_or_else(|response| response)
}