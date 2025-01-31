use core::error::Error;
use sqlx::{MySql, Pool};
use crate::repository::BaseRepository;
use core::http::user_context::UserContext;
use core::models::users::CountUserRow;
use core::models::users::{
    User,
    UserRow,
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub(crate) pool: Pool<MySql>,
}

impl BaseRepository for UserRepository {
    fn new(pool_connection: Pool<MySql>) -> Self {
        UserRepository {
            pool: pool_connection
        }
    }
}

impl UserRepository {
    pub async fn update_user_token(
        &self,
        user_context: UserContext,
        token: String
    ) -> Result<(), Error> {
        sqlx::query(
            "UPDATE `users` SET token = ? WHERE device_id = ? AND device_type = ?"
        )
            .bind::<String>(token)
            .bind::<String>(user_context.device_id)
            .bind::<String>(user_context.device_type.into())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?;

        Ok(())
    }

    pub async fn create_user(
        &self,
        user_context: UserContext,
        token: String
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO `users` (device_id, device_type, token, creation_date) VALUES(?, ?, ?, NOW())"
        )
            .bind::<String>(user_context.device_id)
            .bind::<String>(user_context.device_type.into())
            .bind::<String>(token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn count(&self) -> Result<i32, Error> {
        let result = sqlx::query_as::<_, CountUserRow>(
            r#"
                SELECT COUNT(*) as count FROM `users`
            "#
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?;

        Ok(result.count)
    }

    pub async fn user_exist(self, user_context: UserContext) -> Result<bool, Error> {
        let result = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM `users` WHERE device_id = ? AND device_type = ?"
        )
            .bind::<String>(user_context.device_id)
            .bind::<String>(user_context.device_type.into())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(format!("Could not fetch user : {e:?}")))?;

        Ok(result.is_some())
    }

    pub async fn user_exist_by_id(&self, user_id: String) -> Result<bool, Error> {
        let result = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM `users` WHERE id = ?"
        )
            .bind::<String>(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(format!("Could not fetch user : {e:?}")))?;

        Ok(result.is_some())
    }

    pub async fn fetch_by_id(&self, user_id: i32) -> Result<User, Error> {
        let result = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM `users` WHERE id = ?"
        )
            .bind::<i32>(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(format!("Could not fetch user : {e:?}")))?;

        if result.is_none() {
            return Err(Error::NotFoundError("User not found".to_string()));
        }

        Ok(result.unwrap().into())
    }

    pub async fn fetch_user_paginated(
        &self,
        offset: i32,
        limit: i32
    ) -> Result<Vec<User>, Error> {
        let result = sqlx::query_as::<_, UserRow>(
            r#"
                SELECT * FROM `users`
                ORDER BY id DESC
                LIMIT ?
                OFFSET ?
            "#
        )
            .bind::<i32>(limit)
            .bind::<i32>(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?
            .iter()
            .map(|u| u.into())
            .collect();

        Ok(result)
    }
}