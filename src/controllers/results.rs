use actix_web::{web, HttpResponse};
use serde_json::json;
use tfhe::{ConfigBuilder, FheUint8, generate_keys, set_server_key};
use tfhe::prelude::{FheDecrypt, FheEncrypt};
use crate::db_operations::results::{add_result};
use crate::models::app_state::AppState;
use crate::models::result::{ResultModel, NewResult};

pub async fn create_result(state: web::Data<AppState>, json: web::Json<NewResult>) -> HttpResponse {
    println!("Data is {:#?}", json);

    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    let encrypted_score = FheUint8::encrypt(json.score as u8, &client_key);

    //Server-side
    set_server_key(server_key);
    let result =  (encrypted_score * 100) / 12;

    //Client-side
    let decrypted_result: u8 = result.decrypt(&client_key);

    let new_result = NewResult {
        user_id: json.user_id.clone(),
        quiz_id: json.quiz_id.clone(),
        score: decrypted_result.clone() as i32,
    };

    let mut connection_guard = state.db_connection.lock().unwrap();
    let result = add_result(new_result, &mut *connection_guard);

    match result {
        Ok(result) => HttpResponse::Created().json(result),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Error creating result: {}", err)
        })),
    }
}

// pub async fn update_result(state: web::Data<AppState>, json: web::Json<ResultModel>) -> HttpResponse {
//     let mut connection_guard = state.db_connection.lock().unwrap();
//     let updated_result = update_result_by_id(json.id, json.into_inner(), &mut *connection_guard);
//
//     match updated_result {
//         Ok(result) => HttpResponse::Ok().json(result),
//         Err(err) => HttpResponse::InternalServerError().json(json!({
//             "status": "error",
//             "message": format!("Error updating result: {}", err)
//         })),
//     }
// }
//
// pub async fn delete_result(state: web::Data<AppState>, json: web::Json<ResultModel>) -> HttpResponse {
//     let mut connection_guard = state.db_connection.lock().unwrap();
//     let deletion_result = delete_result_by_id(json.id, &mut *connection_guard);
//
//     match deletion_result {
//         Ok(_) => HttpResponse::Ok().json(json!({
//             "status": "success",
//             "message": "Result deleted successfully"
//         })),
//         Err(err) => HttpResponse::InternalServerError().json(json!({
//             "status": "error",
//             "message": format!("Error deleting result: {}", err)
//         })),
//     }
// }
