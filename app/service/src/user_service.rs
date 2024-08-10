use std::collections::HashMap;
use core::{
    user_context::UserContext,
    error::Error,
    config::Config
};
use types::{
    string::FirebaseToken,
    enums::NotificationType,
    identifier::Identifier,
    user::ListOfUsersWithExtraData
};
use crate::{
    redis_service::RedisService,
    rows::{
        notification_row::NotificationRow,
        user_from_row::{CountUserFromRow, UserFromRow}
    }
};

pub type ListOfUserFromRow = Vec<UserFromRow>;

pub struct UserService {
    redis_service: RedisService
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            redis_service: RedisService::new()
        }
    }
}

impl UserService {
    pub async fn update_user_token_service(&mut self, user_context: &UserContext, token: FirebaseToken) -> Result<(), Error> {
        match self.user_exist(&user_context).await {
            Ok(user_exist) => {
                if !user_exist {
                    self.create_user(&user_context, &token).await?;

                    return Ok(())
                }

                self.update_user_token(&user_context, &token).await?;
                Ok(())
            },
            Err(error) => Err(Error::ProviderError(error.into()))
        }
    }

    pub async fn send_user_notification(
        &mut self,
        user_id: Identifier,
        notification_type: NotificationType,
        extra_data: Option<HashMap<String, String>>,
        template_data: Option<HashMap<String, String>>
    ) -> Result<(), Error> {
        match self.user_exist_by_id(user_id.clone()).await {
            Ok(result) => {
                if !result {
                    return Err(Error::NotFoundError("User not found".to_string()));
                }
            },
            Err(result) => return Err(Error::ProviderError(result.into()))
        }

        let notification = NotificationRow {
            user_id: user_id.into(),
            notification_type: notification_type.into(),
            number_of_tries: 0,
            extra_data: if extra_data.is_none() { None } else { Option::from(serde_json::to_string(&extra_data.unwrap()).unwrap()) },
            template_data: if template_data.is_none() { None } else { Option::from(serde_json::to_string(&template_data.unwrap()).unwrap()) },
        };
        self.redis_service.create_notification(notification).await?;

        Ok(())
    }

    pub async fn find_user_by_id(&mut self, user_id: Identifier) -> Result<UserFromRow, Error> {
        let conn = Config::get_database_conn().await?;
        let result = sqlx::query_as::<_, UserFromRow>(
            "SELECT * FROM `users` WHERE id = ?"
        )
            .bind::<String>(user_id.into())
            .fetch_optional(&conn)
            .await;

        match result {
            Ok(result) => {
                if result.is_some() {
                    return Ok(result.unwrap())
                }

                Err(Error::NotFoundError(format!("User with id {} does not exists", user_id.0.clone().to_string())))
            },
            Err(err) => {
                Err(Error::ProviderError(err.to_string()))
            }
        }
    }

    pub async fn fetch_user_paginated(&mut self, offset: i32, limit: i32) -> Result<ListOfUserFromRow, Error> {
        let conn = Config::get_database_conn().await?;
        let result = sqlx::query_as::<_, UserFromRow>(
            r#"
                SELECT * FROM `users`
                ORDER BY id DESC
                LIMIT ?
                OFFSET ?
            "#
        )
            .bind::<i32>(limit)
            .bind::<i32>(offset)
            .fetch_all(&conn)
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(err) => {
                Err(Error::ProviderError(err.to_string()))
            }
        }
    }

    pub async fn count(&mut self) -> Result<i32, Error> {
        let conn = Config::get_database_conn().await?;
        let result = sqlx::query_as::<_, CountUserFromRow>(
            r#"
                SELECT COUNT(*) as total_count FROM `users`
            "#
        )
            .fetch_one(&conn)
            .await;

        match result {
            Ok(result) => Ok(result.total_count),
            Err(err) => {
                Err(Error::ProviderError(err.to_string()))
            }
        }
    }

    pub async fn send_users_notification(
        &mut self,
        users_with_extra_data: ListOfUsersWithExtraData,
        notification_type: NotificationType
    ) -> Result<(), Error> {
        for user in users_with_extra_data {
            self.send_user_notification(
                user.id.into(),
                notification_type.clone(),
                user.extra_data,
                user.template_data
            ).await?;
        }

        Ok(())
    }
}

impl UserService {
    async fn update_user_token(&mut self, user_context: &UserContext, token: &FirebaseToken) -> Result<(), Error> {
        let conn = Config::get_database_conn().await?;
        sqlx::query(
            "UPDATE `users` SET token = ? WHERE device_id = ? AND device_type = ?"
        )
            .bind::<String>(token.clone().0)
            .bind::<String>(user_context.clone().device_id.try_into().unwrap())
            .bind::<String>(user_context.clone().device_type.try_into().unwrap())
            .execute(&conn)
            .await?;

        Ok({})
    }

    async fn create_user(&mut self, user_context: &UserContext, token: &FirebaseToken) -> Result<(), Error> {
        let conn = Config::get_database_conn().await?;
        sqlx::query(
            "INSERT INTO `users` (device_id, device_type, token, creation_date) VALUES(?, ?, ?, NOW())"
        )
            .bind::<String>(user_context.clone().device_id.try_into().unwrap())
            .bind::<String>(user_context.clone().device_type.try_into().unwrap())
            .bind::<String>(token.clone().0)
            .execute(&conn)
            .await?;

        Ok({})
    }

    async fn user_exist(&mut self, user_context: &UserContext) -> Result<bool, Error> {
        let conn = Config::get_database_conn().await?;
        let result = sqlx::query_as::<_, UserFromRow>(
            "SELECT * FROM `users` WHERE device_id = ? AND device_type = ?"
        )
            .bind::<String>(user_context.clone().device_id.try_into().unwrap())
            .bind::<String>(user_context.clone().device_type.to_string())
            .fetch_optional(&conn)
            .await;

        match &result {
            Ok(result) => {
                if result.is_none() {
                    return Ok(false)
                }

                Ok(true)
            },
            Err(err) => {
                Err(Error::ProviderError(err.to_string()))
            }
        }
    }

    async fn user_exist_by_id(&mut self, user_id: Identifier) -> Result<bool, Error> {
        let conn = Config::get_database_conn().await?;
        let result = sqlx::query_as::<_, UserFromRow>(
            "SELECT * FROM `users` WHERE id = ?"
        )
            .bind::<String>(user_id.into())
            .fetch_optional(&conn)
            .await;

        match &result {
            Ok(result) => {
                if result.is_none() {
                    return Ok(false)
                }

                Ok(true)
            },
            Err(err) => {
                Err(Error::ProviderError(err.to_string()))
            }
        }
    }
}

