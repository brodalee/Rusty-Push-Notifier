use core::{
    error::Error,
    config::Config
};
use sqlx::{FromRow, MySql, Pool};

pub trait MigrationTrait {
    async fn up(&mut self) -> Result<(), Error>;
    async fn down(&mut self) -> Result<(), Error>;
    fn get_name(&mut self) -> String;
    fn new() -> impl MigrationTrait;

    async fn get_connection() -> Pool<MySql> {
        Config::get_database_conn()
            .await
            .expect("Could not connect to database")
    }
}

#[derive(FromRow, Clone, Debug)]
pub struct MigrationRow {
    id: u32,
    version_name: String
}