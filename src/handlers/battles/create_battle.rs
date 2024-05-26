use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::models::battle::Battle,
    infra::{
        db::database::Database,
        repositories::{battles, monsters},
    },
    Response,
};

#[derive(Serialize, Deserialize)]
pub struct CreateBattleRequest {
    monster_a: Option<String>,
    monster_b: Option<String>,
}

#[post("/battles")]
pub async fn create_battle(
    db: web::Data<Database>,
    new_battle: web::Json<CreateBattleRequest>,
) -> HttpResponse {
    if new_battle.monster_a.is_none() || new_battle.monster_b.is_none() {
        return HttpResponse::BadRequest().json(Response {
            status: "error".to_string(),
            message: "Invalid monster parameters.".to_string(),
        });
    }

    let monster_a_id = new_battle.monster_a.clone().unwrap();
    let monster_b_id = new_battle.monster_b.clone().unwrap();

    if monster_a_id == monster_b_id {
        return HttpResponse::BadRequest().json("Invalid monster parameters.");
    }

    let monster_a = monsters::get_monster_by_id(&db, &monster_a_id);
    let monster_b = monsters::get_monster_by_id(&db, &monster_b_id);

    let mut monster_a = match monster_a {
        Some(monster) => monster,
        None => return HttpResponse::NotFound().json("Monster A not found"),
    };

    let mut monster_b = match monster_b {
        Some(monster) => monster,
        None => return HttpResponse::NotFound().json("Monster B not found"),
    };

    while monster_a.hp > 0 && monster_b.hp > 0 {
        let (attacker, defender) = if monster_a.speed > monster_b.speed
            || (monster_a.speed == monster_b.speed && monster_a.attack == monster_b.attack)
        {
            (&monster_a, &mut monster_b)
        } else {
            (&monster_b, &mut monster_a)
        };

        let damage = if attacker.attack > defender.defense {
            attacker.attack - defender.defense
        } else {
            1
        };

        defender.hp = defender.hp.saturating_sub(damage);
    }

    let winner = if monster_a.hp > 0 {
        monster_a
    } else {
        monster_b
    };

    let current_time = Utc::now().naive_utc();

    let new_battle = Battle {
        id: Uuid::new_v4().to_string(),
        monster_a: monster_a_id,
        monster_b: monster_b_id,
        winner: winner.id,
        created_at: Some(current_time),
        updated_at: Some(current_time),
    };

    let battle = battles::create_battle(&db, new_battle);
    match battle {
        Ok(battle) => HttpResponse::Created().json(battle),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
