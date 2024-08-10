use crate::migration::MigrationTrait;
use core::error::Error;

pub struct Migration {}

impl MigrationTrait for Migration {
    async fn up(&mut self) -> Result<(), Error> {
        let conn = Self::get_connection().await;
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `migrations` (
                `id` INTEGER unsigned NOT NULL auto_increment primary key,
                `version_name` VARCHAR(255) NOT NULL
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
        "Migration_base000000".to_string()
    }

    fn new() -> impl MigrationTrait {
        Self {}
    }
}