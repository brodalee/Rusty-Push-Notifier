use std::collections::HashMap;
use std::convert::Infallible;
use serde::{Serialize, Deserialize};
use service::rows::user_from_row::UserFromRow;
use utoipa::ToSchema;
use core::error::Error;
use types::{
    enums::NotificationType,
    user::ListOfUsersWithExtraData
};

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct UpdateUserTokenDto {
    pub token: String
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SendUserNotificationDto {
    pub notification_type: String,
    pub extra_data: Option<HashMap<String, String>>,
    pub template_data: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct PaginatedUserDto {
    pub total_count: i32,
    pub total_page: i32,
    pub next_page: Option<i32>,
    pub previous_page: Option<i32>,
    pub users: Vec<UserDto>
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct UserDto {
    pub id: i32,
    pub device_id: String,
    pub device_type: String
}

impl From<&UserFromRow> for UserDto {
    fn from(value: &UserFromRow) -> Self {
        UserDto {
            id: value.id,
            device_id: value.clone().device_id,
            device_type: value.clone().device_type
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersWithExtraDataDto {
    pub id: i32,
    pub extra_data: Option<HashMap<String, String>>,
    pub template_data: Option<HashMap<String, String>>
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SendUsersNotificationDto {
    pub notification_type: String,
    pub users: ListOfUsersWithExtraData
}

impl SendUsersNotificationDto {
    pub fn validate(&mut self) -> Result<&mut Self, Error> {
        if self.users.is_empty() {
            return Err(Error::ValidationError("ids cannot be empty".to_string()))
        }

        let notification_type: Result<NotificationType, Infallible> = self.clone().notification_type.try_into();
        match notification_type {
            Ok(_) => Ok(self),
            Err(_) => Err(Error::ValidationError("Bad notification type given".to_string()))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserIdPathParameterDto {
    pub user_id: String
}

impl UserIdPathParameterDto {
    pub fn get_id_or_error(&mut self) -> Result<i32, Error> {
        match self.user_id.parse::<i32>() {
            Ok(id) => Ok(id),
            Err(_) => Err(Error::ValidationError("user_id must be integer".to_string()))
        }
    }
}