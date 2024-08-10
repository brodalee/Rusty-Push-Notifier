use redis::{Client, Commands, Connection, RedisResult};
use core::{
    config::Config,
    error::Error
};
use redis::streams::StreamRangeReply;
use crate::rows::notification_row::NotificationRow;

pub type ListOfRedisIdWithNotificationRow = Vec<(String, NotificationRow)>;
pub type ListOfRedisIdWithNotificationRowRef<'a> = Vec<(&'a String, &'a NotificationRow)>;

pub struct RedisService {
    client: Connection
}

impl RedisService {
    pub fn new() -> Self {
        RedisService { client: Self::get_client().unwrap() }
    }
}

impl RedisService {
    pub async fn create_notification(&mut self, notification: NotificationRow) -> Result<(), Error> {
        let _: () = self.client
            .xadd(
                Self::NOTIFICATION_STREAM_KEY,
                "*",
                &[("notification", serde_json::to_string(&notification).unwrap())]
            )
            .expect("Failed to create notification");

        Ok(())
    }

    pub async fn fetch_notification_to_send<'a>(&mut self) -> Result<ListOfRedisIdWithNotificationRow, Error> {
        let result: RedisResult<StreamRangeReply> = self.client
            .xrange_count(
                Self::NOTIFICATION_STREAM_KEY,
                "-",
                "+",
                100
            );

        match result {
            Ok(data) => {
                let mut notifications: ListOfRedisIdWithNotificationRow = Vec::from([]);
                data.ids.iter().for_each(|i| {
                    let data = i.map.get("notification").into();
                    notifications.push((i.clone().id, data))
                });

                Ok(notifications)
            },
            Err(err) => Err(Error::ProviderError(err.to_string()))
        }
    }

    pub async fn delete_ids(&mut self, ids: Vec<&String>) -> Result<(), Error> {
        if ids.is_empty() {
            return Ok(())
        }

        let _: () = self.client.xdel(
            Self::NOTIFICATION_STREAM_KEY,
            &*ids
        )
        .expect("Failed to delete notification");

        Ok(())
    }
}

impl RedisService {
    const NOTIFICATION_STREAM_KEY: &'static str = "notifications";
}

impl RedisService {
    fn get_client() -> Result<Connection, Error> {
        let conn = Client::open(Config::get_redis_uri())
            .expect("Invalid connection URL")
            .get_connection()
            .expect("failed to connect to Redis");

        Ok(conn)
    }
}