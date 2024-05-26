use actix_web::{post, web, HttpResponse};

use crate::{
    domain::models::monster::Monster,
    infra::{db::database::Database, repositories::monsters},
};

#[post("/monsters")]
pub async fn create_monster(
    db: web::Data<Database>,
    new_monster: web::Json<Monster>,
) -> HttpResponse {
    let monster = monsters::create_monster(&db, new_monster.into_inner());
    match monster {
        Ok(monster) => HttpResponse::Created().json(monster),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
