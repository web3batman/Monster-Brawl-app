use actix_web::{delete, web, HttpResponse};

use crate::{
    infra::{db::database::Database, repositories::monsters},
    Response,
};

#[delete("/monsters/{id}")]
pub async fn delete_monster_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let monster = monsters::delete_monster_by_id(&db, &id);
    match monster {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().json(Response {
            status: "error".to_string(),
            message: "Monster not found".to_string(),
        }),
    }
}
