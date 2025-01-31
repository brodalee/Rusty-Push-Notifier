use actix_web::web;
use crate::handler::notifications::ctrl::{create_notification, fetch_notification_paginated, send_notification_to_user, send_notification_to_users};

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/notifications")
                .service(create_notification)
                .service(fetch_notification_paginated)
                .service(send_notification_to_user)
                .service(send_notification_to_users)
        );
}