use core::error::Error;
use sqlx::{MySql, Pool};
use crate::repository::BaseRepository;
use core::models::notifications::{
    NotificationRow,
    NotificationWithParameters,
    Notification,
    NotificationParameterRow,
    NotificationParameter,
};
use core::models::CountRow;

#[derive(Debug, Clone)]
pub struct NotificationRepository {
    pub(crate) pool: Pool<MySql>,
}

impl BaseRepository for NotificationRepository {
    fn new(pool_connection: Pool<MySql>) -> Self {
        NotificationRepository {
            pool: pool_connection
        }
    }
}

impl NotificationRepository {
    pub async fn exist_by_name(&self, name: String) -> Result<bool, Error> {
        let result = sqlx::query_as::<_, NotificationRow>(
            "SELECT * FROM `notifications` WHERE name = ?"
        )
            .bind::<String>(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(format!("Could not fetch notifications : {e:?}")))?;

        Ok(result.is_some())
    }

    pub async fn create_notification(
        &self,
        notification: NotificationWithParameters,
    ) -> Result<String, Error> {
        let result = sqlx::query(
            "INSERT INTO `notifications` (name, content, title) VALUES(?, ?, ?)"
        )
            .bind::<String>(notification.name)
            .bind::<String>(notification.content)
            .bind::<String>(notification.title)
            .execute(&self.pool)
            .await?;

        let notification_id = result.last_insert_id();

        for parameter in notification.parameters {
            sqlx::query(
                "INSERT INTO `notification_parameters` (name, notification_id) VALUES(?, ?)"
            )
                .bind::<String>(parameter.name.clone())
                .bind::<i32>(notification_id.clone() as i32)
                .execute(&self.pool)
                .await?;
        }

        Ok(result.last_insert_id().to_string())
    }

    pub async fn fetch_paginated(&self, offset: i32, limit: i32) -> Result<Vec<Notification>, Error> {
        Ok(
            sqlx::query_as::<_, NotificationRow>(
                r#"SELECT *
                FROM `notifications`
                ORDER BY id DESC
                LIMIT ?
                OFFSET ?"#
            )
                .bind::<i32>(limit)
                .bind::<i32>(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| Error::ProviderError(format!("Could not fetch notifications : {e:?}")))?
                .iter()
                .map(|n| n.into())
                .collect()
        )
    }

    pub async fn fetch_notification_parameters(&self, notification_id: i32) -> Result<Vec<NotificationParameter>, Error> {
        Ok(
            sqlx::query_as::<_, NotificationParameterRow>(
                "SELECT *
                FROM `notification_parameters`
                WHERE notification_id = ?"
            )
                .bind::<i32>(notification_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| Error::ProviderError(format!("Could not fetch notifications parameters : {e:?}")))?
                .iter()
                .map(|np| np.into())
                .collect()
        )
    }

    pub async fn total_count(&self) -> Result<i32, Error> {
        let result = sqlx::query_as::<_, CountRow>(
            r#"
                SELECT COUNT(*) as total_count FROM `notifications`
            "#
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?;

        Ok(result.total_count)
    }

    pub async fn fetch_by_id(&self, notification_id: i32) -> Result<Notification, Error> {
        let result = sqlx::query_as::<_, NotificationRow>(
            "SELECT * FROM `notifications` WHERE id = ?"
        )
            .bind::<i32>(notification_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::ProviderError(format!("Could not fetch notifications : {e:?}")))?;

        if result.is_none() {
            return Err(Error::NotFoundError("Notification not found".to_string()));
        }

        Ok(
            result.unwrap().into()
        )
    }
}