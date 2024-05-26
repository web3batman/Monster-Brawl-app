use chrono::Utc;
use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::{
    domain::models::{battle::Battle, monster::Monster},
    infra::db::{
        database::Database,
        schema::{battles::dsl::battles, monsters::dsl::monsters},
    },
};

#[allow(dead_code)]
pub async fn init_test_monsters(db: &Database) -> Vec<Monster> {
    let mut connection = db.get_connection();
    let current_time = Utc::now().naive_utc();
    let monsters_data: Vec<Monster> = vec![
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-1".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 40,
            defense: 20,
            hp: 50,
            speed: 80,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-2".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 70,
            defense: 20,
            hp: 40,
            speed: 40,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-3".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 40,
            defense: 25,
            hp: 50,
            speed: 80,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-4".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 70,
            defense: 20,
            hp: 50,
            speed: 40,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-5".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 40,
            defense: 20,
            hp: 100,
            speed: 40,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-6".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 10,
            defense: 10,
            hp: 100,
            speed: 80,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
        Monster {
            id: uuid::Uuid::new_v4().to_string(),
            name: "monster-7".to_string(),
            image_url: "https://loremflickr.com/640/480".to_string(),
            attack: 60,
            defense: 10,
            hp: 150,
            speed: 40,
            created_at: Some(current_time),
            updated_at: Some(current_time),
        },
    ];

    let mut test_monsters = vec![];

    for monster_data in monsters_data {
        let inserted_monster = diesel::insert_into(monsters::table())
            .values(&monster_data)
            .get_result(&mut connection);

        match inserted_monster {
            Ok(monster) => {
                test_monsters.push(monster);
            }
            Err(err) => {
                eprintln!("Error inserting monster: {:?}", err);
            }
        }
    }

    test_monsters
}

#[allow(dead_code)]
pub async fn init_test_battle(db: &Database) -> Battle {
    let test_monsters = init_test_monsters(db).await;
    let mut connection = db.get_connection();
    let current_time = Utc::now().naive_utc();
    let battle_data = Battle {
        id: uuid::Uuid::new_v4().to_string(),
        monster_a: test_monsters[0].id.clone(),
        monster_b: test_monsters[1].id.clone(),
        winner: test_monsters[0].id.clone(),
        created_at: Some(current_time),
        updated_at: Some(current_time),
    };

    match diesel::insert_into(battles::table())
        .values(&battle_data)
        .get_result(&mut connection)
    {
        Ok(battle) => battle,
        Err(err) => {
            eprintln!("Error inserting battle: {:?}", err);
            panic!("Failed to insert battle");
        }
    }
}
