use actix_web::{get, web, HttpResponse};

use crate::infra::{db::database::Database, repositories::battles};

#[get("/battles/{id}")]
pub async fn get_battle_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let battle = battles::get_battle_by_id(&db, &id);
    match battle {
        Some(battle) => HttpResponse::Ok().json(battle),
        None => HttpResponse::NotFound().json("Battle not found"),
    }
}
