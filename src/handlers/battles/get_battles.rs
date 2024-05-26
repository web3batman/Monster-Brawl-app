use actix_web::{get, web, HttpResponse};

use crate::infra::{db::database::Database, repositories::battles};

#[get("/battles")]
pub async fn get_battles(db: web::Data<Database>) -> HttpResponse {
    let battles = battles::get_battles(&db);
    HttpResponse::Ok().json(battles)
}
