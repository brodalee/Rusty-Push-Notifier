use utoipa::{OpenApi};
use crate::{
    dto::{
        user_dto::{
            UpdateUserTokenDto,
            UserDto,
            PaginatedUserDto,
            SendUserNotificationDto,
            SendUsersNotificationDto
        },
        pagination_dto::PaginationDto
    }
};
use core::responses::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::super::controllers::health_check::health_check,
        super::super::controllers::users::update_user_token,
        super::super::controllers::users::send_user_notification,
        super::super::controllers::users::send_users_notifications,
    ),
    components(
        schemas(
            UpdateUserTokenDto,
            UserDto,
            SendUserNotificationDto,
            PaginatedUserDto,
            ErrorResponse,
            PaginationDto,
            SendUsersNotificationDto
        )
    ),
    tags((name = "Health-Check"), (name = "Users")),
)]
pub struct ApiDoc;