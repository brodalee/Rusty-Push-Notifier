use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod notifications;
pub mod users;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct CountRow {
    pub total_count: i32
}