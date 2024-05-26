use actix_web::{web, App, HttpServer};
use monster_brawl::{config, healthcheck, infra, not_found};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = infra::db::database::Database::new();
    let app_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(healthcheck)
            .configure(config)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((config::CONFIG.server_host(), config::CONFIG.server_port()))?
    .run()
    .await
}
