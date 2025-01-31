use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use crate::http::user_context::DeviceType;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub device_type: DeviceType,
    pub device_id: String,
    pub token: Option<String>,
    pub creation_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDto {
    pub id: i32,
    pub device_type: DeviceType,
    pub creation_date: String,
}

impl User {
    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.clone().id,
            device_type: self.clone().device_type.into(),
            creation_date: self.clone().creation_date.to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: i32,
    pub device_type: String,
    pub device_id: String,
    pub token: Option<String>,
    pub creation_date: DateTime<Utc>,
}

impl Into<User> for UserRow {
    fn into(self) -> User {
        User {
            id: self.id,
            token: self.token,
            creation_date: self.creation_date,
            device_type: DeviceType::try_from(self.device_type).expect("Bad device type"),
            device_id: self.device_id,
        }
    }
}

impl Into<User> for &UserRow {
    fn into(self) -> User {
        User {
            id: self.clone().id,
            token: self.clone().token,
            creation_date: self.clone().creation_date,
            device_type: self.clone().device_type.into(),
            device_id: self.clone().device_id,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct CountUserRow {
    pub count: i32,
}