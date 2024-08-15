use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::db_operations::answers::*;
use crate::models::answer::{Answer, NewAnswer};
use crate::models::app_state::AppState;

pub async fn create_answer(state: web::Data<AppState>, json: web::Json<NewAnswer>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    let result = add_answer(json.into_inner(), &mut *connection_guard);

    match result {
        Ok(answer) => HttpResponse::Created().json(answer),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Error creating answer: {}", err)
        })),
    }
}

pub async fn get_answer_by_id(state: web::Data<AppState>, json: web::Json<NewAnswer>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match get_an_answer_by_id(&mut *connection_guard, json.id) {
        Some(answer) => HttpResponse::Ok().json(answer),
        None => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Answer not found"
        })),
    }
}

pub async fn update_answer(state: web::Data<AppState>, json: web::Json<Answer>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    // Implement update functionality here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Answer updated successfully"
    }))
}

pub async fn delete_answer(state: web::Data<AppState>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    // Implement delete functionality here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Answer deleted successfully"
    }))
}
