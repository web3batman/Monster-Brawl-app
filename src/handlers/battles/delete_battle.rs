use actix_web::{delete, web, HttpResponse};

use crate::infra::{db::database::Database, repositories::battles};

#[delete("/battles/{id}")]
pub async fn delete_battle_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let battle = battles::delete_battle_by_id(&db, &id);
    match battle {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().json("Battle not found"),
    }
}
