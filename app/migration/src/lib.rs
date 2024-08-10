mod migrations;
mod migration;

use core::{
    error::Error,
    config::Config
};
use crate::{
    migration::{
        MigrationTrait,
        MigrationRow
    },
    migrations::{
        migration_base00000::Migration as BaseMigration,
        migration_init000000::Migration as FirstMigration
    }
};

pub struct Migrator {}

impl Migrator {
    pub async fn migrate() -> Result<(), Error> {
        Config::ping_database().await?;

        let migrations = Vec::from([
            FirstMigration {}
        ]);
        Self::execute_migrations(migrations).await?;

        Ok(())
    }
}

impl Migrator {
    async fn execute_migrations(migrations: Vec<impl MigrationTrait>) -> Result<(), Error> {
        Self::create_migration_table_if_needed().await?;

        for mut migration in migrations {
            if !Self::already_executed(migration.get_name()).await? {
                migration.up().await?;
                Self::add_migration(migration.get_name()).await?
            }
        }
        Ok(())
    }

    async fn add_migration(migration_name: String) -> Result<(), Error> {
        sqlx::query(
            r#"INSERT INTO `migrations` (`version_name`) VALUES (?)"#,
        )
            .bind::<String>(migration_name)
            .execute(&Config::get_database_conn().await?)
            .await?;
        Ok(())
    }

    async fn already_executed(migration_name: String) -> Result<bool, Error> {
        let result = sqlx::query_as::<_, MigrationRow>(
            r#"SELECT * FROM `migrations` WHERE `version_name` = ?"#,
        )
            .bind::<String>(migration_name)
            .fetch_optional(&Config::get_database_conn().await?)
            .await?;

        Ok(!result.is_none())
    }

    async fn create_migration_table_if_needed() -> Result<(), Error> {
        let mut migration = BaseMigration {};
        Ok(migration.up().await?)
    }
}