use actix_web::{get, web, HttpResponse};

use crate::infra::{db::database::Database, repositories::monsters};

#[get("/monsters")]
pub async fn get_monsters(db: web::Data<Database>) -> HttpResponse {
    let monsters = monsters::get_monsters(&db);
    HttpResponse::Ok().json(monsters)
}
