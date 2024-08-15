use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::db_operations::user_answers::*;
use crate::models::app_state::AppState;
use crate::models::result::ResultModel;
use crate::models::user_answer::{UserAnswer, NewUserAnswer};

pub async fn create_user_answer(state: web::Data<AppState>, json: web::Json<NewUserAnswer>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    let result = add_user_answer(json.into_inner(), &mut *connection_guard);

    match result {
        Ok(user_answer) => HttpResponse::Created().json(user_answer),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Error creating user answer: {}", err)
        })),
    }
}

pub async fn get_user_answer_by_id(state: web::Data<AppState>, json: web::Json<UserAnswer>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match get_a_user_answer_by_id(&mut *connection_guard, json.id) {
        Some(user_answer) => HttpResponse::Ok().json(user_answer),
        None => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "User answer not found"
        })),
    }
}

// pub async fn update_user_answer(state: web::Data<AppState>, json: web::Json<UserAnswer>) -> HttpResponse {
//     let mut connection_guard = state.db_connection.lock().unwrap();
//     let updated_user_answer = update_user_answer_by_id(json.id, json.into_inner(), &mut *connection_guard);
//
//     match updated_user_answer {
//         Ok(user_answer) => HttpResponse::Ok().json(user_answer),
//         Err(err) => HttpResponse::InternalServerError().json(json!({
//             "status": "error",
//             "message": format!("Error updating user answer: {}", err)
//         })),
//     }
// }
//
// pub async fn delete_user_answer(state: web::Data<AppState>, json: web::Json<UserAnswer>) -> HttpResponse {
//     let mut connection_guard = state.db_connection.lock().unwrap();
//     let deletion_result = delete_user_answer_by_id(json.id, &mut *connection_guard);
//
//     match deletion_result {
//         Ok(_) => HttpResponse::Ok().json(json!({
//             "status": "success",
//             "message": "User answer deleted successfully"
//         })),
//         Err(err) => HttpResponse::InternalServerError().json(json!({
//             "status": "error",
//             "message": format!("Error deleting user answer: {}", err)
//         })),
//     }
// }
