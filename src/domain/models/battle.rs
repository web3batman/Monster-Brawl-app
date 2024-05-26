use crate::domain::models::monster::Monster;

use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Queryable,
    Insertable,
    AsChangeset,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(Monster, foreign_key = winner))]
#[diesel(table_name = crate::infra::db::schema::battles)]
pub struct Battle {
    pub id: String,
    pub monster_a: String,
    pub monster_b: String,
    pub winner: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::NaiveDateTime>,
}
