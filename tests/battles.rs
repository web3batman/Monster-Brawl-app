use actix_web::web::Data;
use actix_web::{http, test, App};
use utils::{init_test_battle, init_test_monsters};

use monster_brawl::{
    domain::models::battle::{Battle, CreateBattleRequest},
    handlers::battles::{
        create_battle::create_battle, delete_battle_by_id::delete_battle_by_id,
        get_battle_by_id::get_battle_by_id, get_battles::get_battles,
    },
    infra::db::database::Database,
};

mod utils;

#[actix_rt::test]
async fn test_should_get_all_battles_correctly() {
    let db = Database::new();
    let app = App::new().app_data(Data::new(db)).service(get_battles);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/battles").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_should_get_404_error_if_battle_does_not_exists() {
    let db = Database::new();
    let app = App::new().app_data(Data::new(db)).service(get_battles);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/battles/99999").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_should_get_a_single_battle_correctly() {
    let mut db = Database::new();
    let test_battle = init_test_battle(&mut db).await;

    let app = App::new().app_data(Data::new(db)).service(get_battle_by_id);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri(format!("/battles/{}", test_battle.id).as_str())
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_should_delete_a_battle_correctly() {
    let mut db = Database::new();
    let _test_battle = init_test_battle(&mut db).await;

    let app = App::new()
        .app_data(Data::new(db))
        .service(delete_battle_by_id);

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::delete()
        .uri(format!("/battles/{}", _test_battle.id).as_str())
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
}

#[actix_rt::test]
async fn test_should_create_a_battle_with_404_error_if_one_parameter_has_a_monster_id_does_not_exists(
) {
    let mut db = Database::new();
    let _test_battle = init_test_battle(&mut db).await;

    let app = App::new().app_data(Data::new(db)).service(create_battle);

    let mut app = test::init_service(app).await;

    let new_battle = CreateBattleRequest {
        monster_a: Some(_test_battle.monster_a.clone()),
        monster_b: Some("123".to_string()),
    };

    let req = test::TestRequest::post()
        .uri("/battles")
        .set_json(&new_battle)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_should_create_a_battle_with_a_bad_request_response_if_one_parameter_is_null() {
    let mut db = Database::new();
    let _test_battle = init_test_battle(&mut db).await;

    let app = App::new().app_data(Data::new(db)).service(create_battle);

    let mut app = test::init_service(app).await;

    let new_battle = CreateBattleRequest {
        monster_a: Some(_test_battle.monster_a.clone()),
        monster_b: None,
    };

    let req = test::TestRequest::post()
        .uri("/battles")
        .set_json(&new_battle)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_client_error());
}

#[actix_rt::test]
async fn test_should_create_battle_correctly_with_monster_a_winning() {
    let mut db = Database::new();
    let _test_battle = init_test_battle(&mut db).await;

    let app = App::new().app_data(Data::new(db)).service(create_battle);

    let mut app = test::init_service(app).await;

    let new_battle = CreateBattleRequest {
        monster_a: Some(_test_battle.monster_a.clone()),
        monster_b: Some(_test_battle.monster_b.clone()),
    };

    let req = test::TestRequest::post()
        .uri("/battles")
        .set_json(&new_battle)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let battle: Battle = test::read_body_json(resp).await;
    assert_eq!(battle.winner, _test_battle.monster_a);
}

#[actix_rt::test]
async fn test_should_create_battle_correctly_with_monster_b_winning_if_theirs_speeds_same_and_monster_b_has_higher_attack(
) {
    let db = Database::new();
    let _test_monsters = init_test_monsters(&db).await;

    let app = App::new().app_data(Data::new(db)).service(create_battle);

    let mut app = test::init_service(app).await;

    let new_battle = CreateBattleRequest {
        monster_a: Some(_test_monsters[5].id.clone()),
        monster_b: Some(_test_monsters[2].id.clone()),
    };

    let req = test::TestRequest::post()
        .uri("/battles")
        .set_json(&new_battle)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::CREATED);

    let battle: Battle = test::read_body_json(resp).await;
    assert_eq!(battle.winner, new_battle.monster_b.unwrap());
}
