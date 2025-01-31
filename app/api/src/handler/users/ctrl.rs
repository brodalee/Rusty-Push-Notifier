use actix_web::{get, put, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use core::error::Error;
use crate::app_state::{AppState};
use core::http::user_context::get_user_context;
use crate::dto::PaginationDto;
use core::models::users::UserDto;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct UpdateUserToken {
    pub token: String,
}

#[utoipa::path(
    path = "/users/token",
    tag = "Users",
    params(
        (
            "X-DEVICE-TYPE" = String,
            Header,
            description = "Device Type ( android / ios / web )",
            example = "android",
        ),
        (
            "X-DEVICE-ID" = String,
            Header,
            description = "Device uniq identifier",
            example = "FIB567FBE8",
        )
    ),
    responses(
        (status = 204, description = "User token is updated"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[put("/token")]
pub async fn update_user_token(
    app_state: web::Data<AppState>,
    request: HttpRequest,
    input: web::Json<UpdateUserToken>,
) -> Result<HttpResponse, Error> {
    let user_context = get_user_context(request)?;

    app_state
        .user_service
        .update_user_token(user_context, input.token.to_string())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}


#[utoipa::path(
    path = "/users",
    tag = "Users",
    params(
        (
            "limit" = i32,
            Query,
            description = "Max user to retrieve ( >= 10 && <= 100 )",
            example = "50",
        ),
        (
            "page" = i32,
            Query,
            description = "Page of the list",
            example = "1",
        )
    ),
    responses(
        (status = 200, description = "Fetch users paginated", content_type = "application/json"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[get("")]
pub async fn fetch_user_list_paginated(
    app_state: web::Data<AppState>,
    mut input: web::Query<PaginationDto>
) -> Result<HttpResponse, Error> {
    input.validate()?;
    let users = app_state
        .user_service
        .fetch_user_paginated(input.get_offset(), input.get_limit())
        .await?;

    let total_count = app_state
        .user_service
        .total_count()
        .await?;

    let total_page = ((total_count / input.get_limit()) as f32).ceil();

    let response = HttpResponse::Ok()
        .insert_header(("TOTAL-COUNT", total_count.to_string()))
        .insert_header(("TOTAL-PAGE", total_page.to_string()))
        .json(
            users
                .iter()
                .map(|u| u.to_dto())
                .collect::<Vec<UserDto>>()
        );

    Ok(response)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserIdPathParameterDto {
    pub user_id: String,
}

impl UserIdPathParameterDto {
    pub fn get_id_or_error(&self) -> Result<i32, Error> {
        Ok(
            self
                .user_id
                .parse::<i32>()
                .map_err(|_| Error::ValidationError("user_id must be integer".to_string()))?
        )
    }
}

#[utoipa::path(
    path = "/users/{user_id}",
    tag = "Users",
    responses(
        (status = 200, description = "Fetch user by id"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[get("/{user_id}")]
pub async fn fetch_user_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<UserIdPathParameterDto>,
) -> Result<HttpResponse, Error> {
    let user = app_state
        .user_service
        .fetch_by_id(params.get_id_or_error()?)
        .await?;

    Ok(
        HttpResponse::Ok()
            .json(
                user.to_dto()
            )
    )
}

#[get("/{user_id}/notifications")]
pub async fn fetch_user_notifications_paginated(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    todo!()
}