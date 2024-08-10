use core::{
    error::Error,
    config::Config
};
use dotenv::dotenv;
use service::{
    notification_history_service::{NotificationHistoryService},
    notification_service::NotificationService,
    user_service::UserService,
    redis_service::{
        RedisService,
        ListOfRedisIdWithNotificationRow,
        ListOfRedisIdWithNotificationRowRef
    }
};
use std::thread;

pub type ListOfRedisId<'a> = Vec<&'a String>;
pub type Services = (RedisService, UserService, NotificationHistoryService, NotificationService);

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    Config::check_config().await?;

    println!("Starting consumer");
    let redis_service = RedisService::new();
    let user_service = UserService::new();
    let notification_history_service = NotificationHistoryService::new();
    let notification_service = NotificationService::new().await;
    let services: &mut Services = &mut (redis_service, user_service, notification_history_service, notification_service);

    loop {
        consume(services).await?;
        thread::sleep_ms(5000);
    }
}

async fn consume<'a>(
    services: &mut Services
) -> Result<(), Error> {
    let notifications: ListOfRedisIdWithNotificationRow = services.0.fetch_notification_to_send().await?;
    consume_notifications(notifications, services).await?;

    Ok({})
}

async fn consume_notifications<'a>(
    notifications: ListOfRedisIdWithNotificationRow,
    services: &mut Services,
) -> Result<(), Error> {
    let mut succeeded: ListOfRedisId = Vec::from([]);
    let mut failed: ListOfRedisIdWithNotificationRowRef = Vec::from([]);

    for (id, notification) in &notifications {
        let user = services.1.find_user_by_id(notification.user_id.into()).await?;
        if user.token.is_none() {
            failed.push((id, &notification));
            continue;
        }

        services.3.send_notification(
            &user.token.unwrap().into(),
            &user.device_type.into(),
            &notification
        ).await?;

        services.2.create(notification.clone().as_sent_notification()).await?;
        succeeded.push(id);
    }

    services.0.delete_ids(succeeded.clone()).await?;
    println!("Consumed {} notifications", succeeded.clone().len());

    handle_failed_notifications(failed, services).await?;

    Ok(())
}

async fn handle_failed_notifications<'a>(
    notifications: ListOfRedisIdWithNotificationRowRef<'a>,
    services: &mut Services,
) -> Result<(), Error> {
    if notifications.clone().len() > 0 {
        for (_, &ref failed_notification) in &notifications {
            services.2.create(failed_notification.clone().as_failed_notification()).await?;
        }

        println!("Failed to consume {} notifications", notifications.clone().len());
        let failed_ids: ListOfRedisId = notifications.clone().iter().map(|(&ref id, _)| id).collect();
        services.0.delete_ids(failed_ids).await?;
    }

    Ok(())
}