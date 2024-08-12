use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::cookie::SameSite;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use crate::db_operations::db;
use crate::models::app_state::AppState;
use crate::controllers::users::{ login_user, register_user};

mod models;
mod db_operations;
mod schema;
mod controllers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();


    HttpServer::new(move || {
        let app_state = web::Data::new(AppState {db_connection: Mutex::new(db::establish_connection())  });
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000") // Allow requests from your frontend
                    .allowed_methods(vec!["GET", "POST"]) // Allow these HTTP methods
                    .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .route("/auth/login", web::post().to(login_user))
            .route("/auth/register", web::post().to(register_user))
    })
        .bind(("127.0.0.1", 7000))?.run().await
}
