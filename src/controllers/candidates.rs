use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use serde_json::json;

async fn handle_register_error(error: &str) -> HttpResponse {
    let response_body = json!({
        "status": "error",
        "message": error,
    });
    HttpResponse::Ok().content_type(ContentType::json()).body(response_body.to_string())
}

