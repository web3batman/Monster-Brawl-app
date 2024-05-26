use actix_web::{get, web, App, HttpResponse, Responder, Result};
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

pub mod config;
pub mod domain;
pub mod handlers;
pub mod infra;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "success".to_string(),
        message: "Server is running".to_string(),
    })
}

pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "error".to_string(),
        message: "Not found".to_string(),
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
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
    );
}
