use actix_web::{error, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use core::config::Config;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use migration::Migrator;
use crate::{
    controllers::{
        health_check::web_health_check,
        users::web_users
    },
    swagger::api_doc::ApiDoc
};

mod controllers;
mod dto;
mod swagger;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();

    match Config::check_config().await {
        Err(_) => {
            panic!("Opps, error occured")
        }
        _ => {
            match Migrator::migrate().await {
                Err(err) => {
                    panic!("Could not migrate database : {err:?}")
                },
                _ => {}
            }
        }
    }

    HttpServer::new(|| {
        App::new()
            .service(web_health_check())
            .service(web_users())
            .service(
                SwaggerUi::new("/swagger/{_:.*}")
                    .url("/swagger/doc.json", ApiDoc::openapi()),
            )
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                ).into()
                })
            )
    })
        .bind(("0.0.0.0", Config::get_web_port()))?
        .run()
        .await
}