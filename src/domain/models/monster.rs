use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable,
)]
#[diesel(table_name = crate::infra::db::schema::monsters)]
pub struct Monster {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub attack: i32,
    pub defense: i32,
    pub hp: i32,
    pub speed: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::NaiveDateTime>,
}
