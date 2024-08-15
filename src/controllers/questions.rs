use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::db_operations::questions::*;
use crate::models::app_state::AppState;
use crate::models::question::{NewQuestion, Question};

pub async fn create_question(state: web::Data<AppState>, json: web::Json<NewQuestion>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    let result = add_question(json.into_inner(), &mut *connection_guard);

    match result {
        Ok(question) => HttpResponse::Created().json(question),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Error creating question: {}", err)
        })),
    }
}
pub async fn update_question(state: web::Data<AppState>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    // Implement update functionality here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Question updated successfully"
    }))
}

pub async fn delete_question(state: web::Data<AppState>) -> HttpResponse {
    let mut connection_guard = state.db_connection.lock().unwrap();
    // Implement delete functionality here
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Question deleted successfully"
    }))
}
