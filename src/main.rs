use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

mod domain;
mod infra;
mod config;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "success".to_string(),
        message: "Server is running".to_string(),
    })
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "error".to_string(),
        message: "Not found".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
