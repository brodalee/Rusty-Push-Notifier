use sqlx::{MySql, Pool};
use crate::repository::BaseRepository;
use core::error::Error;
use core::models::notifications::{
    Notification,
    SendUserParameter,
    SendUserNotificationExtraData,
    NotificationHistoryStatus,
    NotificationParamIdWithValue,
};

#[derive(Debug, Clone)]
pub struct NotificationHistoryRepository {
    pub(crate) pool: Pool<MySql>,
}

impl BaseRepository for NotificationHistoryRepository {
    fn new(pool_connection: Pool<MySql>) -> Self {
        NotificationHistoryRepository {
            pool: pool_connection
        }
    }
}

impl NotificationHistoryRepository {
    pub async fn create_sent(
        &self,
        notification: Notification,
        parameters: Vec<NotificationParamIdWithValue>,
        extra_data: Vec<SendUserNotificationExtraData>,
        owner_id: i32,
    ) -> Result<i32, Error> {
        let result = sqlx::query(
            "INSERT INTO `history_notifications` (status, owner, notification_id) VALUES(?, ?, ?)"
        )
            .bind::<String>(NotificationHistoryStatus::Sent.into())
            .bind::<i32>(owner_id)
            .bind::<i32>(notification.id)
            .execute(&self.pool)
            .await?;

        if parameters.len() > 0 {
            self
                .create_hnotification_params(
                    result.last_insert_id() as i32,
                    parameters
                )
                .await?;
        }

        if extra_data.len() > 0 {
            self
                .create_hnotification_extra_data(
                    result.last_insert_id() as i32,
                    extra_data
                )
                .await?;
        }

        Ok(result.last_insert_id() as i32)
    }
}

impl NotificationHistoryRepository {
    async fn create_hnotification_params(
        &self,
        hnotification_id: i32,
        parameters: Vec<NotificationParamIdWithValue>,
    ) -> Result<(), Error> {
        // TODO : il faut récupérer l'ID
        for param in parameters {
            sqlx::query(
                "INSERT INTO `history_notifications_params` (parameter_id, value, notification_id) VALUES(?, ?, ?)"
            )
                .bind::<i32>(param.id)
                .bind::<String>(param.value)
                .bind::<i32>(hnotification_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    async fn create_hnotification_extra_data(
        &self,
        hnotification_id: i32,
        extra_data: Vec<SendUserNotificationExtraData>,
    ) -> Result<(), Error> {
        for param in extra_data {
            sqlx::query(
                "INSERT INTO `history_notifications_extra_data` (parameter_name, value, notification_id) VALUES(?, ?, ?)"
            )
                .bind::<String>(param.name)
                .bind::<String>(param.value)
                .bind::<i32>(hnotification_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}