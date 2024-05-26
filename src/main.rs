use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use handlers::{
    battles::{
        create_battle::create_battle, delete_battle_by_id::delete_battle_by_id,
        get_battle_by_id::get_battle_by_id, get_battles::get_battles,
    },
    monsters::{
        create_monster::create_monster, delete_monster_by_id::delete_monster_by_id,
        get_monster_by_id::get_monster_by_id, get_monsters::get_monsters, import_csv::import_csv,
        update_monster_by_id::update_monster_by_id,
    },
};
use serde::Serialize;

mod config;
mod domain;
mod handlers;
mod infra;
mod utils;

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
    let db = infra::db::database::Database::new();
    let app_data = web::Data::new(db).clone();

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(healthcheck)
            .service(
                web::scope("/api")
                    .service(create_monster)
                    .service(delete_monster_by_id)
                    .service(get_monster_by_id)
                    .service(get_monsters)
                    .service(import_csv)
                    .service(update_monster_by_id)
                    .service(create_battle)
                    .service(delete_battle_by_id)
                    .service(get_battle_by_id)
                    .service(get_battles),
            )
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((config::CONFIG.server_host(), config::CONFIG.server_port()))?
    .run()
    .await
}
