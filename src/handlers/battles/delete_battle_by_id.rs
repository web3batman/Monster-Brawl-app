use actix_web::{delete, web, HttpResponse};

use crate::{
    infra::{db::database::Database, repositories::battles},
    Response,
};

#[delete("/battles/{id}")]
pub async fn delete_battle_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let battle = battles::delete_battle_by_id(&db, &id);
    match battle {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().json(Response {
            status: "error".to_string(),
            message: "Battle not found".to_string(),
        }),
    }
}
