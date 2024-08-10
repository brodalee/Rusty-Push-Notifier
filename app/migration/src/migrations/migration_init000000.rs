use crate::migration::MigrationTrait;
use core::error::Error;

pub struct Migration {}

impl MigrationTrait for Migration {
    async fn up(&mut self) -> Result<(), Error> {
        let conn = Self::get_connection().await;
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `users` (
                `id` INTEGER NOT NULL auto_increment primary key,
                `device_type` VARCHAR(255) NOT NULL,
                `device_id` VARCHAR(255) NOT NULL,
                `token` VARCHAR(255) NULL,
                `creation_date` DATETIME NOT NULL
                )"#
        )
            .execute(&conn)
            .await?;

        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `history_notifications` (
                `id` INTEGER NOT NULL auto_increment primary key,
                `status` VARCHAR(255) NOT NULL,
                `owner` INTEGER NOT NULL,
                `creation_date` DATETIME NOT NULL,
                `update_date` DATETIME NOT NULL,
                `notification_type` VARCHAR(255) NOT NULL,
                `extra_data` JSON NULL,
                CONSTRAINT FK_owner_id_history_notification FOREIGN KEY (owner) REFERENCES users(id)
                )"#
        )
            .execute(&conn)
            .await?;

        Ok(())
    }

    async fn down(&mut self) -> Result<(), Error> {
        // TODO sql
        Ok(())
    }

    fn get_name(&mut self) -> String {
        "Migration_init000000".to_string()
    }

    fn new() -> impl MigrationTrait {
        Self {}
    }
}