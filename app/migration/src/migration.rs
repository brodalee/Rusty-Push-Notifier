use core::{
    error::Error,
};
use sqlx::{FromRow};

pub trait MigrationTrait {
    async fn up(&mut self) -> Result<(), Error>;
    fn get_name(&mut self) -> String;
}

#[derive(FromRow)]
pub struct MigrationRow {}