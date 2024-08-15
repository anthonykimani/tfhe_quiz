use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::db_operations::quizzes::*;
use crate::models::app_state::AppState;
use crate::models::quiz::{Quiz, NewQuiz};

pub async fn create_quiz(state: web::Data<AppState>, json: web::Json<NewQuiz>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    let result = add_quiz(json.into_inner(), &mut *connection_guard);

    match result {
        Ok(quiz) => HttpResponse::Created().json(quiz),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Error creating quiz: {}", err)
        })),
    }
}
pub async fn update_quiz(state: web::Data<AppState>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    // Implement update functionality here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Quiz updated successfully"
    }))
}

pub async fn delete_quiz(state: web::Data<AppState>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    // Implement delete functionality here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Quiz deleted successfully"
    }))
}
