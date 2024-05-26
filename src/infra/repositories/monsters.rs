use chrono::prelude::*;
use diesel::prelude::*;

use crate::{
    domain::models::monster::Monster,
    infra::db::{database::Database, schema::monsters::dsl::*},
};

pub fn get_monsters(db: &Database) -> Vec<Monster> {
    let mut connection = db.get_connection();
    monsters
        .load::<Monster>(&mut connection)
        .expect("error getting all monsters")
}

pub fn get_monster_by_id(db: &Database, monster_id: &str) -> Option<Monster> {
    let mut connection = db.get_connection();
    match monsters
        .find(monster_id)
        .get_result::<Monster>(&mut connection)
    {
        Ok(monster) => Some(monster),
        Err(_) => None,
    }
}

pub fn create_monster(db: &Database, monster: Monster) -> Result<Monster, diesel::result::Error> {
    let mut connection = db.get_connection();
    let monster = Monster {
        id: uuid::Uuid::new_v4().to_string(),
        ..monster
    };
    diesel::insert_into(monsters)
        .values(&monster)
        .execute(&mut connection)
        .expect("error creating a new monster");
    Ok(monster)
}

pub fn update_monster_by_id(
    db: &Database,
    monster_id: &str,
    mut monster: Monster,
) -> Option<Monster> {
    let mut connection = db.get_connection();

    if let Ok(_existing_monster) = monsters
        .find(monster_id)
        .get_result::<Monster>(&mut connection)
    {
        monster.updated_at = Some(Utc::now().naive_utc());
        let updated_monster = diesel::update(monsters.find(monster_id))
            .set(&monster)
            .get_result::<Monster>(&mut connection)
            .expect("error updating monster by id");
        Some(updated_monster)
    } else {
        None
    }
}

pub fn delete_monster_by_id(db: &Database, monster_id: &str) -> Option<usize> {
    let mut connection = db.get_connection();

    if let Ok(_existing_monster) = monsters
        .find(monster_id)
        .get_result::<Monster>(&mut connection)
    {
        let count = diesel::delete(monsters.find(monster_id))
            .execute(&mut connection)
            .expect("error deleting monster by id");
        Some(count)
    } else {
        None
    }
}
