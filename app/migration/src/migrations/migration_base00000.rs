use crate::migration::MigrationTrait;
use core::error::Error;
use sqlx::{MySql, Pool};

pub struct Migration {
    pub(crate) pool: Pool<MySql>
}

impl MigrationTrait for Migration {
    async fn up(&mut self) -> Result<(), Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `migrations` (
                `id` INTEGER unsigned NOT NULL auto_increment primary key,
                `version_name` VARCHAR(255) NOT NULL
                )"#
        )
            .execute(&self.pool.clone())
            .await?;

        Ok(())
    }

    fn get_name(&mut self) -> String {
        "Migration_base000000".to_string()
    }
}