use actix_web::{put, web, HttpResponse};

use crate::{
    domain::models::monster::Monster,
    infra::{db::database::Database, repositories::monsters},
    Response,
};

#[put("/monsters/{id}")]
pub async fn update_monster_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_monster: web::Json<Monster>,
) -> HttpResponse {
    let monster = monsters::update_monster_by_id(&db, &id, updated_monster.into_inner());
    match monster {
        Some(monster) => HttpResponse::Ok().json(monster),
        None => HttpResponse::NotFound().json(Response {
            status: "error".to_string(),
            message: "Monster not found".to_string(),
        }),
    }
}
