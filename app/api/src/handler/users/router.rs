use actix_web::web;
use crate::handler::users::ctrl::{
    fetch_user_by_id,
    fetch_user_list_paginated,
    update_user_token
};

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/users")
                .service(update_user_token)
                .service(fetch_user_list_paginated)
                .service(fetch_user_by_id)
        );
}