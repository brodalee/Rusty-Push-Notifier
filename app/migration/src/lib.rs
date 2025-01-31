mod migrations;
mod migration;

use core::{
    error::Error,
};
use sqlx::{MySql, Pool};
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

pub struct Migrator {
    pub(crate) pool: Pool<MySql>,
}

impl Migrator {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

impl Migrator {
    pub async fn migrate(&self) -> Result<(), Error> {
        let migrations = Vec::from([
            FirstMigration { pool: self.pool.clone() },
        ]);
        self.execute_migrations(migrations).await?;

        Ok(())
    }

    async fn execute_migrations(&self, migrations: Vec<impl MigrationTrait>) -> Result<(), Error> {
        self.create_migration_table_if_needed(self.pool.clone()).await?;

        for mut migration in migrations {
            if !self.already_executed(migration.get_name()).await? {
                migration.up().await?;
                self.add_migration(migration.get_name()).await?
            }
        }
        Ok(())
    }

    async fn add_migration(&self, migration_name: String) -> Result<(), Error> {
        sqlx::query(
            r#"INSERT INTO `migrations` (`version_name`) VALUES (?)"#,
        )
            .bind::<String>(migration_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn already_executed(&self, migration_name: String) -> Result<bool, Error> {
        let result = sqlx::query_as::<_, MigrationRow>(
            r#"SELECT * FROM `migrations` WHERE `version_name` = ?"#,
        )
            .bind::<String>(migration_name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(!result.is_none())
    }

    async fn create_migration_table_if_needed(&self, pool: Pool<MySql>) -> Result<(), Error> {
        let mut migration = BaseMigration { pool };
        Ok(migration.up().await?)
    }
}