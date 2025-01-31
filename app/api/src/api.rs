use std::env;
use std::sync::{Arc};
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use fcm::FcmClient;
use sqlx::mysql::MySqlPoolOptions;
use migration::Migrator;
use repository::notification_history_repository::NotificationHistoryRepository;
use repository::notification_repository::NotificationRepository;
use repository::repository::{BaseRepository, Repository};
use repository::user_repository::UserRepository;
use service::notification_service::NotificationService;
use service::user_service::UserService;
use crate::app_state::{AppState, AppStateChecker};
use crate::handler;
use service::firebase_notification_service::FirebaseNotificationService;

#[actix_web::main]
pub async fn start_api() -> std::io::Result<()> {
    dotenv().ok();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let web_port = env::var("WEB_PORT").expect("WEB_PORT must be set").parse::<u16>().unwrap();
    let gsacp = env::var("GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH").expect("GOOGLE_SERVICE_ACCOUNT_CREDENTIALS_PATH must be set");

    // TODO
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info")
    );

    let db_pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect(&db_uri)
        .await
        .expect("Could not connect to database");

    Migrator::new(db_pool.clone())
        .migrate()
        .await
        .expect("Could not migrate");

    let fcm_client = Arc::new(
        FcmClient::builder()
            .service_account_key_json_path(gsacp)
            .build()
            .await
            .expect("Bad Google credentials given.")
    );

    HttpServer::new(move || {
        let pool = db_pool.clone();

        let repository = Repository::new(pool.clone());
        let user_repository = UserRepository::new(pool.clone());
        let notification_repository = NotificationRepository::new(pool.clone());
        let notification_history_repository = NotificationHistoryRepository::new(pool.clone());

        let user_service = UserService::new(user_repository);
        let notification_service = NotificationService::new(
            notification_repository,
            notification_history_repository,
            user_service.clone(),
            FirebaseNotificationService::new(fcm_client.clone()),
        );

        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(
                web::Data::new(
                    AppState {
                        user_service,
                        notification_service
                    }
                )
            )
            .app_data(
                web::Data::new(
                    AppStateChecker { repository }
                )
            )
            .configure(handler::swagger::router::configure)
            .configure(handler::notifications::router::configure)
            .configure(handler::users::router::configure)
            .configure(handler::health::router::configure)
    })
        .workers(10)
        .bind(("0.0.0.0", web_port))?
        .run()
        .await
}