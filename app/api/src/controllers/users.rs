use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use core::{http_helper::get_user_context, error::Error};
use service::user_service::UserService;
use crate::{
    dto::{
        pagination_dto::PaginationDto,
        user_dto::{
            PaginatedUserDto, SendUserNotificationDto,
            UpdateUserTokenDto, SendUsersNotificationDto,
            UserIdPathParameterDto
        }
    },
};

#[utoipa::path(
    path = "/users",
    tag = "Users",
    responses(
        (status = 204, description = "User token is updated", body = String, content_type = "text/plain"),
        (status = 400, description = "Bad request", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorResponse, content_type = "application/json")
    )
)]
#[put("")]
pub async fn update_user_token(
    dto: web::Json<UpdateUserTokenDto>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let user_context = get_user_context(req)?;

    let mut user_service = UserService::new();
    user_service.update_user_token_service(&user_context, dto.clone().token.into()).await?;

    Ok(HttpResponse::NoContent().body(""))
}

#[utoipa::path(
    path = "/users/{user_id}/notifications",
    tag = "Users",
    responses(
        (status = 201, description = "Notification is sent", body = String, content_type = "text/plain"),
        (status = 400, description = "Bad request", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorResponse, content_type = "application/json")
    )
)]
#[post("/{user_id}/notifications")]
pub async fn send_user_notification(
    dto: web::Json<SendUserNotificationDto>,
    mut params: web::Path<UserIdPathParameterDto>
) -> Result<HttpResponse, Error> {
    let user_id = params.get_id_or_error()?;

    let mut user_service = UserService::new();
    user_service.send_user_notification(
        user_id.into(),
        dto.clone().notification_type.into(),
        dto.clone().extra_data,
        dto.clone().template_data
    ).await?;

    Ok(HttpResponse::Created().body(""))
}

#[utoipa::path(
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "List of users paginated", body = PaginatedUserDto, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorResponse, content_type = "application/json")
    )
)]
#[get("")]
pub async fn fetch_user_list_paginated(
    mut pagination_info: web::Query<PaginationDto>
) -> Result<HttpResponse, Error> {
    pagination_info.validate()?;

    let offset = pagination_info.get_offset();
    let max_result = pagination_info.get_limit();

    let mut user_service = UserService::new();
    let list_of_users = user_service.fetch_user_paginated(offset, max_result).await?;
    let total_count = user_service.count().await?;

    let dvs: f32 = (total_count / max_result) as f32;
    let total_page = dvs.ceil() as i32;

    let next_page: Option<i32> = if offset + 2 < total_page { Option::from(offset + 2) } else { None };
    let previous_page: Option<i32> = if offset > 0 { Option::from(offset) } else { None };

    Ok(HttpResponse::Ok().json(
        PaginatedUserDto {
            total_count: total_count.clone(),
            total_page,
            next_page,
            previous_page,
            users: list_of_users.iter().map(|u| u.into()).collect()
        }
    ))
}

#[utoipa::path(
    path = "/users/notifications",
    tag = "Users",
    responses(
        (status = 201, description = "Created", body = String, content_type = "plain/text"),
        (status = 400, description = "Bad request", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorResponse, content_type = "application/json")
    )
)]
#[post("/notifications")]
pub async fn send_users_notifications(
    mut dto: web::Json<SendUsersNotificationDto>
) -> Result<HttpResponse, Error> {
    dto.validate()?;

    let mut user_service = UserService::new();
    user_service.send_users_notification(dto.clone().users, dto.clone().notification_type.into()).await?;

    Ok(HttpResponse::Created().body(""))
}

pub fn web_users() -> actix_web::Scope {
    web::scope("/users")
        .service(update_user_token)
        .service(send_user_notification)
        .service(fetch_user_list_paginated)
        .service(send_users_notifications)
}