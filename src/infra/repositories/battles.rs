use diesel::prelude::*;

use crate::{
    domain::models::battle::Battle,
    infra::db::{database::Database, schema::battles::dsl::*},
};

pub fn get_battles(db: &Database) -> Vec<Battle> {
    let mut connection = db.get_connection();
    battles
        .load::<Battle>(&mut connection)
        .expect("error getting all battles")
}

pub fn get_battle_by_id(db: &Database, battle_id: &str) -> Option<Battle> {
    let mut connection = db.get_connection();
    match battles
        .find(battle_id)
        .get_result::<Battle>(&mut connection)
    {
        Ok(battle) => Some(battle),
        Err(_) => None,
    }
}

pub fn create_battle(db: &Database, battle: Battle) -> Result<Battle, diesel::result::Error> {
    let mut connection = db.get_connection();
    let battle = Battle {
        id: uuid::Uuid::new_v4().to_string(),
        ..battle
    };
    diesel::insert_into(battles)
        .values(&battle)
        .execute(&mut connection)
        .expect("error creating a new battle");
    Ok(battle)
}

pub fn delete_battle_by_id(db: &Database, battle_id: &str) -> Option<usize> {
    let mut connection = db.get_connection();

    if let Ok(_existing_battle) = battles
        .find(battle_id)
        .get_result::<Battle>(&mut connection)
    {
        let count = diesel::delete(battles.find(battle_id))
            .execute(&mut connection)
            .expect("error deleting battle by id");
        Some(count)
    } else {
        None
    }
}
