use actix_web::{get, web, HttpResponse};
use core::{
    config::Config,
    error::Error
};

#[utoipa::path(
    path = "/health_check",
    tag = "Health-Check",
    responses(
        (status = 200, description = "Api is working fully", body = String, content_type = "text/plain"),
        (status = 500, description = "Api cant serve correctly requests", body = String, content_type = "text/plain")
    )
)]
#[get("")]
pub async fn health_check() -> Result<HttpResponse, Error> {
    Config::ping_database().await?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("OK"))
}

pub fn web_health_check() -> actix_web::Scope {
    web::scope("/health_check")
        .service(health_check)
}