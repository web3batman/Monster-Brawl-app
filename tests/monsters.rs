use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{http, test, App};

use monster_brawl::{
    domain::models::monster::Monster,
    handlers::monsters::{
        create_monster::create_monster, delete_monster_by_id::delete_monster_by_id,
        get_monster_by_id::get_monster_by_id, get_monsters::get_monsters,
        import_monsters_csv::import_monsters_csv, update_monster_by_id::update_monster_by_id,
    },
    infra::db::database::Database,
    Response,
};

mod utils;

#[actix_rt::test]
async fn test_should_get_all_monsters_correctly() {
    let db = Database::new();
    let app = App::new().app_data(Data::new(db)).service(get_monsters);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/monsters").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_should_get_404_error_if_monster_does_not_exists() {
    let db = Database::new();
    let app = App::new()
        .app_data(Data::new(db))
        .service(get_monster_by_id);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/monsters/999999")
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_should_get_a_single_monster_correctly() {
    let mut db = Database::new();
    let test_monsters = utils::init_test_monsters(&mut db).await;

    let app = App::new()
        .app_data(Data::new(db))
        .service(get_monster_by_id);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri(format!("/monsters/{}", test_monsters[0].id).as_str())
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_should_create_a_new_monster() {
    let mut db = Database::new();
    let _test_monsters = utils::init_test_monsters(&mut db).await;

    let app = App::new().app_data(Data::new(db)).service(create_monster);

    let mut app = test::init_service(app).await;

    let new_monster_data = Monster {
        id: _test_monsters[0].id.clone(),
        name: _test_monsters[0].name.clone(),
        image_url: _test_monsters[0].image_url.clone(),
        attack: _test_monsters[0].attack.clone(),
        defense: _test_monsters[0].defense.clone(),
        speed: _test_monsters[0].speed.clone(),
        hp: _test_monsters[0].hp.clone(),
        created_at: _test_monsters[0].created_at.clone(),
        updated_at: _test_monsters[0].updated_at.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/monsters")
        .set_json(&new_monster_data)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);
}

#[actix_rt::test]
async fn test_should_update_a_monster_correctly() {
    let mut db = Database::new();
    let _test_monsters = utils::init_test_monsters(&mut db).await;

    let app = App::new()
        .app_data(Data::new(db))
        .service(update_monster_by_id);

    let mut app = test::init_service(app).await;

    let update_monster_data = Monster {
        id: _test_monsters[0].id.clone(),
        name: "Update name of monster".to_string(),
        image_url: _test_monsters[0].image_url.clone(),
        attack: _test_monsters[0].attack.clone(),
        defense: _test_monsters[0].defense.clone(),
        speed: _test_monsters[0].speed.clone(),
        hp: _test_monsters[0].hp.clone(),
        created_at: _test_monsters[0].created_at.clone(),
        updated_at: _test_monsters[0].updated_at.clone(),
    };
    let req = test::TestRequest::put()
        .uri(format!("/monsters/{}", _test_monsters[0].id).as_str())
        .set_json(&update_monster_data)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_should_update_with_404_error_if_monster_does_not_exists() {
    let mut db = Database::new();
    let _test_monsters = utils::init_test_monsters(&mut db).await;

    let app = App::new()
        .app_data(Data::new(db))
        .service(update_monster_by_id);

    let mut app = test::init_service(app).await;

    let update_monster_data = Monster {
        id: _test_monsters[0].id.clone(),
        name: "Update name of monster".to_string(),
        image_url: _test_monsters[0].image_url.clone(),
        attack: _test_monsters[0].attack.clone(),
        defense: _test_monsters[0].defense.clone(),
        speed: _test_monsters[0].speed.clone(),
        hp: _test_monsters[0].hp.clone(),
        created_at: _test_monsters[0].created_at.clone(),
        updated_at: _test_monsters[0].updated_at.clone(),
    };
    let req = test::TestRequest::put()
        .uri(format!("/monsters/{}", 99999).as_str())
        .set_json(&update_monster_data)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_should_delete_a_monster_correctly() {
    let mut db = Database::new();
    let _test_monsters = utils::init_test_monsters(&mut db).await;

    let app = App::new()
        .app_data(Data::new(db))
        .service(delete_monster_by_id);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::delete()
        .uri(format!("/monsters/{}", _test_monsters[0].id).as_str())
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}

#[actix_rt::test]
async fn test_should_delete_with_404_error_if_monster_does_not_exists() {
    let mut db = Database::new();
    let _test_monsters = utils::init_test_monsters(&mut db).await;

    let app = App::new()
        .app_data(Data::new(db))
        .service(delete_monster_by_id);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::delete()
        .uri(format!("/monsters/{}", 99999).as_str())
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_should_import_all_the_csv_objects_into_the_database_successfully() {
    let db = Database::new();

    let app = App::new()
        .app_data(Data::new(db))
        .service(import_monsters_csv);

    let mut app = test::init_service(app).await;

    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";

    let file_content =
        std::fs::read("./tests/files/monsters-correct.csv").expect("Failed to read file");

    let payload = format!(
        "--{boundary}\r\n\
            Content-Disposition: form-data; name=\"file\"; filename=\"monsters-correct.csv\"\r\n\
            Content-Type: text/csv\r\n\r\n\
            {file_content}\r\n\
            --{boundary}--\r\n",
        boundary = boundary,
        file_content = String::from_utf8_lossy(&file_content),
    );

    let req = test::TestRequest::post()
        .uri("/monsters/import_monsters_csv")
        .insert_header((
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        ))
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_should_fail_when_importing_csv_file_with_inexistent_columns() {
    let db = Database::new();

    let app = App::new()
        .app_data(Data::new(db))
        .service(import_monsters_csv);

    let mut app = test::init_service(app).await;

    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";

    let file_content =
        std::fs::read("./tests/files/monsters-wrong-column.csv").expect("failed to read file");

    let payload = format!(
        "--{boundary}\r\n\
            Content-Disposition: form-data; name=\"file\"; filename=\"monsters-wrong-column.csv\"\r\n\
            Content-Type: text/csv\r\n\r\n\
            {file_content}\r\n\
            --{boundary}--\r\n",
        boundary = boundary,
        file_content = String::from_utf8_lossy(&file_content),
    );

    let req = test::TestRequest::post()
        .uri("/monsters/import_monsters_csv")
        .insert_header((
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        ))
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

    let expected_response = Response {
        status: "error".to_string(),
        message: "Incomplete data, check your file.".to_string(),
    };
    let expected_body = serde_json::to_string(&expected_response).unwrap();

    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body, expected_body);
}
