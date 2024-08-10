use chrono::Utc;
use core::{
    error::Error,
    config::Config
};
use types::{
    dates::{CreationDate, UpdateDate},
    enums::{NotificationStatus, NotificationType},
    identifier::Identifier
};
use crate::rows::notification_row::NotificationRow;

pub struct NotificationHistoryService {}

impl NotificationHistoryService {
    pub fn new() -> Self {
        NotificationHistoryService {}
    }
}

pub struct NotificationHistory {
    pub user_id: Identifier,
    pub notification_type: NotificationType,
    pub notification_status: NotificationStatus,
    pub creation_date: CreationDate,
    pub update_date: UpdateDate
}

impl From<&NotificationRow> for NotificationHistory {
    fn from(notification: &NotificationRow) -> Self {
        NotificationHistory {
            user_id: notification.clone().user_id.into(),
            notification_status: NotificationStatus::Failed,
            update_date: UpdateDate { 0: Utc::now() },
            creation_date: CreationDate { 0: Utc::now()},
            notification_type: notification.clone().notification_type.into()
        }
    }
}

impl NotificationHistoryService {
    pub async fn create(&mut self, notification: NotificationHistory) -> Result<(), Error> {
        let conn = Config::get_database_conn().await?;
        sqlx::query(
            r#"
                INSERT INTO `history_notifications`
                    (owner, creation_date, update_date, notification_type, status)
                    VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind::<String>(notification.user_id.into())
            .bind::<chrono::DateTime<Utc>>(notification.creation_date.0)
            .bind::<chrono::DateTime<Utc>>(notification.update_date.0)
            .bind::<String>(notification.notification_type.into())
            .bind::<String>(notification.notification_status.into())
            .execute(&conn)
            .await?;

        Ok(())
    }
}