use core::error::Error;
use sqlx::{Executor, MySql, Pool};

#[derive(Clone, Debug)]
pub struct Repository {
    pub(crate) pool: Pool<MySql>,
}

pub trait BaseRepository {
    fn new(pool_connection: Pool<MySql>) -> Self;
}

impl Repository {
    pub fn new(pool_connection: Pool<MySql>) -> Self {
        Repository {
            pool: pool_connection
        }
    }
}

impl Repository {
    pub async fn ping(&self) -> Result<(), Error> {
        self
            .pool
            .execute("SELECT 1+1 as result")
            .await
            .map_err(|_| Error::ProviderError("Could not ping db".to_string()))?;

        Ok(())
    }
}