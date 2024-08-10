use std::env;
use sqlx::{Pool, MySql, MySqlPool, Executor};
use crate::error::Error;

pub struct Config {}

impl Config {
    const DATABASE_URL_ENV_NAME: &'static str = "DATABASE_URL";
    const WEB_PORT_ENV_NAME: &'static str = "WEB_PORT";
    const APP_MODE_ENV_NAME: &'static str = "APP_MODE";
    const REDIS_HOST_ENV_NAME: &'static str = "REDIS_HOST";
    const GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH_ENV_NAME: &'static str = "GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH";

    pub async fn check_config() -> Result<(), Error> {
        env::var(Config::DATABASE_URL_ENV_NAME).expect("DATABASE_URL must be set");
        Self::ping_database().await?;

        env::var(Config::WEB_PORT_ENV_NAME).expect("WEB_PORT must be set");
        env::var(Config::APP_MODE_ENV_NAME).expect("WEB_PORT must be set");
        env::var(Config::REDIS_HOST_ENV_NAME).expect("REDIS_HOST must be set");
        env::var(Config::GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH_ENV_NAME).expect("GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH must be set");

        Ok(())
    }

    pub fn get_web_port() -> u16 {
        let web_port = env::var(Config::WEB_PORT_ENV_NAME).unwrap();
        web_port.parse::<u16>().unwrap()
    }

    pub async fn get_database_conn() -> Result<Pool<MySql>, Error> {
        let pool = MySqlPool::connect(
            &env::var(Config::DATABASE_URL_ENV_NAME).unwrap()
        )
        .await?;

        Ok(pool)
    }

    pub async fn ping_database() -> Result<(), Error> {
        let db_conn = Self::get_database_conn().await?;
        let result = db_conn.execute("SELECT 1+1 as result").await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::ProviderError("Could not ping db".to_string()))
        }
    }

    pub fn is_dev() -> bool {
        env::var(Config::APP_MODE_ENV_NAME).unwrap() == "dev"
    }

    pub fn get_redis_uri() -> String {
        env::var(Config::REDIS_HOST_ENV_NAME).unwrap()
    }

    pub fn get_google_service_account_credentials_path() -> String {
        env::var(Config::GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH_ENV_NAME).unwrap()
    }

    pub fn get_notification_resources_path() -> String {
        "../resources/notifications.yml".to_string()
    }
}