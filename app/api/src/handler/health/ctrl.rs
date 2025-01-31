use actix_web::{get, web, HttpResponse};
use core::error::Error;
use crate::app_state::AppStateChecker;

#[utoipa::path(
    path = "/health-check",
    tag = "Health",
    responses(
        (status = 200, description = "OK", body = String, example = "OK"),
        (status = 500, description = "KO")
    )
)]
#[get("")]
pub async fn health_check(
    app_state: web::Data<AppStateChecker>,
) -> Result<HttpResponse, Error> {
    // TODO : ping cache service.
    match app_state.repository.ping().await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"status": "ok",}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "status": "ko"})))
    }
}