use actix_web::web;
use crate::handler::health::ctrl::health_check;

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/health-check")
                .service(health_check)
        );
}