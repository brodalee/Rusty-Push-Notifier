use crate::migration::MigrationTrait;
use core::error::Error;
use sqlx::{MySql, Pool};

pub struct Migration {
    pub(crate) pool: Pool<MySql>
}

impl MigrationTrait for Migration {
    async fn up(&mut self) -> Result<(), Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `users` (
                `id` INTEGER NOT NULL auto_increment primary key,
                `device_type` VARCHAR(255) NOT NULL,
                `device_id` VARCHAR(255) NOT NULL,
                `token` VARCHAR(255) NULL,
                `creation_date` DATETIME NOT NULL
                )"#
        )
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `notifications` (
                `id` INTEGER NOT NULL auto_increment primary key,
                `name` VARCHAR(255) NOT NULL,
                `title` VARCHAR(255) NOT NULL,
                `content` TEXT NOT NULL,
                `creation_date` DATETIME NOT NULL DEFAULT NOW(),
                `update_date` DATETIME NOT NULL DEFAULT NOW(),
                UNIQUE (name)
                )"#
        )
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `notification_parameters`(
                `id` INTEGER NOT NULL auto_increment primary key,
                `name` VARCHAR(255) NOT NULL,
                `creation_date` DATETIME NOT NULL DEFAULT NOW(),
                `update_date` DATETIME NOT NULL DEFAULT NOW(),
                `notification_id` INTEGER NOT NULL,
                CONSTRAINT FK_notification_id_parameter_id FOREIGN KEY (notification_id) REFERENCES notifications(id)
                )"#
        )
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `history_notifications`(
                `id` INTEGER NOT NULL auto_increment primary key,
                `status` VARCHAR(255) NOT NULL,
                `owner` INTEGER NOT NULL,
                `creation_date` DATETIME NOT NULL DEFAULT NOW(),
                `update_date` DATETIME NOT NULL DEFAULT NOW(),
                `notification_id` INTEGER NOT NULL,
                CONSTRAINT FK_owner_id_history_notification FOREIGN KEY (owner) REFERENCES users(id),
                CONSTRAINT FK_notification_id_id FOREIGN KEY (notification_id) REFERENCES notifications(id)
                )"#
        )
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `history_notifications_params`(
                `id` INTEGER NOT NULL auto_increment primary key,
                `parameter_id` INTEGER NOT NULL,
                `value` VARCHAR(255) NOT NULL,
                `notification_id` INTEGER NOT NULL,
                CONSTRAINT FK_parameter_id_notification_parameter_id FOREIGN KEY (parameter_id) REFERENCES notification_parameters(id),
                CONSTRAINT FK_parameter_hnotification_id_to_hnotification_id FOREIGN KEY (notification_id) REFERENCES history_notifications(id)
                )"#
        )
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `history_notifications_extra_data`(
                `id` INTEGER NOT NULL auto_increment primary key,
                `parameter_name` VARCHAR(255) NOT NULL,
                `value` VARCHAR(255) NOT NULL,
                `notification_id` INTEGER NOT NULL,
                CONSTRAINT FK_extra_data_hnotification_id_to_hnotification_id FOREIGN KEY (notification_id) REFERENCES history_notifications(id)
                )"#
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    fn get_name(&mut self) -> String {
        "Migration_init000000".to_string()
    }
}