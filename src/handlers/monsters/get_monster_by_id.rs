use actix_web::{get, web, HttpResponse};

use crate::{
    infra::{db::database::Database, repositories::monsters},
    Response,
};

#[get("/monsters/{id}")]
pub async fn get_monster_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let monster = monsters::get_monster_by_id(&db, &id);
    match monster {
        Some(monster) => HttpResponse::Ok().json(monster),
        None => HttpResponse::NotFound().json(Response {
            status: "error".to_string(),
            message: "Monster not found".to_string(),
        }),
    }
}
