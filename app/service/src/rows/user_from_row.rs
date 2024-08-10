use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct UserFromRow {
    pub id: i32,
    pub device_id: String,
    pub device_type: String,
    pub token: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct CountUserFromRow {
    pub total_count: i32
}