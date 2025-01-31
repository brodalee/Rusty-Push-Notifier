use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::users::ctrl::update_user_token,
        crate::handler::users::ctrl::fetch_user_by_id,
        crate::handler::users::ctrl::fetch_user_list_paginated,

        crate::handler::health::ctrl::health_check,

        crate::handler::notifications::ctrl::create_notification,
        crate::handler::notifications::ctrl::fetch_notification_paginated,
        crate::handler::notifications::ctrl::send_notification_to_user,
        crate::handler::notifications::ctrl::send_notification_to_users,
    ),
    components(
        schemas()
    ),
)]
pub struct ApiDoc;