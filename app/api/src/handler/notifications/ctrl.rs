use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use core::error::Error;
use crate::app_state::AppState;
use core::models::notifications::{
    NotificationWithParameters,
    NotificationParameter,
    SendUserNotification,
    SendUserNotificationExtraData,
    SendUserParameter,
};
use crate::dto::PaginationDto;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct CreateNotificationParameterDto {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct CreateNotificationDto {
    pub name: String,
    pub title: String,
    pub content: String,
    pub parameters: Vec<CreateNotificationParameterDto>,
}

impl Into<NotificationParameter> for &CreateNotificationParameterDto {
    fn into(self) -> NotificationParameter {
        NotificationParameter {
            name: self.clone().name,
            id: 0
        }
    }
}

impl Into<NotificationWithParameters> for CreateNotificationDto {
    fn into(self) -> NotificationWithParameters {
        NotificationWithParameters {
            title: self.title,
            name: self.name,
            content: self.content,
            id: 0,
            parameters: self.parameters.iter().map(|p| p.into()).collect(),
        }
    }
}

#[utoipa::path(
    path = "/notifications",
    tag = "Notifications",
    responses(
        (status = 201, description = "Notification created"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[post("")]
pub async fn create_notification(
    app_state: web::Data<AppState>,
    input: web::Json<CreateNotificationDto>
) -> Result<HttpResponse, Error> {
    let notification_id = app_state
        .notification_service
        .create_notification(input.0.into())
        .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": notification_id})))
}

pub async fn update_notification() -> Result<HttpResponse, Error> {
    // TODO
    Ok(HttpResponse::Ok().json(serde_json::json!({})))
}

#[utoipa::path(
    path = "/notifications",
    tag = "Notifications",
    responses(
        (status = 200, description = "Fetched notifications"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[get("")]
pub async fn fetch_notification_paginated(
    app_state: web::Data<AppState>,
    mut input: web::Query<PaginationDto>
) -> Result<HttpResponse, Error> {
    input.validate()?;
    let notifications = app_state
        .notification_service
        .fetch_notifications_paginated(
            input.get_offset(),
            input.get_limit()
        )
        .await?;

    let total_count_notifications = app_state
        .notification_service
        .total_count()
        .await?;

    let total_page = ((total_count_notifications / input.get_limit()) as f32).ceil();

    let response = HttpResponse::Ok()
        .insert_header(("TOTAL-COUNT", total_count_notifications.to_string()))
        .insert_header(("TOTAL-PAGE", total_page.to_string()))
        .json(notifications);

    Ok(response)
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserIdWithNotificationIdPathParameterDto {
    pub user_id: String,
    pub notification_id: String
}

impl UserIdWithNotificationIdPathParameterDto {
    pub fn get_user_id_or_error(&self) -> Result<i32, Error> {
        Ok(
            self
                .user_id
                .parse::<i32>()
                .map_err(|_| Error::ValidationError("user_id must be integer".to_string()))?
        )
    }

    pub fn get_notification_id_or_error(&self) -> Result<i32, Error> {
        Ok(
            self
                .notification_id
                .parse::<i32>()
                .map_err(|_| Error::ValidationError("notification_id must be integer".to_string()))?
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct NotificationParameterDto {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct NotificationExtraDataDto {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct NotificationDto {
    pub params: Option<Vec<NotificationParameterDto>>,
    pub extra_data: Option<Vec<NotificationExtraDataDto>>,
}

impl Into<SendUserNotification> for NotificationDto {
    fn into(self) -> SendUserNotification {
        let mut extra_data = vec![];
        if self.extra_data.is_some() {
            for ed in self.extra_data.unwrap() {
                extra_data.push(SendUserNotificationExtraData {
                    value: ed.value,
                    name: ed.name,
                })
            }
        }

        let mut params = vec![];
        if self.params.is_some() {
            for ed in self.params.unwrap() {
                params.push(SendUserParameter {
                    value: ed.value,
                    name: ed.name,
                })
            }
        }

        SendUserNotification {
            extra_data: Some(extra_data),
            params: Some(params),
        }
    }
}

impl Into<SendUserParameter> for &NotificationParameterDto {
    fn into(self) -> SendUserParameter {
        SendUserParameter {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

impl Into<SendUserNotificationExtraData> for &NotificationExtraDataDto {
    fn into(self) -> SendUserNotificationExtraData {
        SendUserNotificationExtraData {
            value: self.value.clone(),
            name: self.name.clone(),
        }
    }
}

#[utoipa::path(
    path = "/notifications/{notification_id}/users/{user_id}",
    tag = "Notifications",
    responses(
        (status = 201, description = "Notification sent to user"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[post("/{notification_id}/users/{user_id}")]
pub async fn send_notification_to_user(
    app_state: web::Data<AppState>,
    params: web::Path<UserIdWithNotificationIdPathParameterDto>,
    input: web::Json<NotificationDto>,
) -> Result<HttpResponse, Error> {
    let user_id = params.get_user_id_or_error()?;
    let notification_id = params.get_notification_id_or_error()?;

    app_state
        .notification_service
        .send_user_notification(
            notification_id,
            user_id,
            input.0.clone().into(),
        )
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NotificationIdPathParameterDto {
    pub notification_id: String
}

impl NotificationIdPathParameterDto {
    pub fn get_notification_id_or_error(&self) -> Result<i32, Error> {
        Ok(
            self
                .notification_id
                .parse::<i32>()
                .map_err(|_| Error::ValidationError("notification_id must be integer".to_string()))?
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct UserNotificationDto {
    pub user_id: i32,
    pub params: Option<Vec<NotificationParameterDto>>,
    pub extra_data: Option<Vec<NotificationExtraDataDto>>,
}

#[utoipa::path(
    path = "/notifications/{notification_id}/users",
    tag = "Notifications",
    responses(
        (status = 201, description = "Notifications sent to users"),
        (status = 400, description = "Bad request", content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "application/json")
    )
)]
#[post("/{notification_id}/users")]
pub async fn send_notification_to_users(
    app_state: web::Data<AppState>,
    params: web::Path<NotificationIdPathParameterDto>,
    input: web::Json<Vec<UserNotificationDto>>
) -> Result<HttpResponse, Error> {
    let notification_id = params.get_notification_id_or_error()?;

    for user_notification in input.0.clone() {
        let mut params = vec![];
        if user_notification.params.is_some() {
            for ed in user_notification.params.unwrap() {
                params.push(SendUserParameter { value: ed.value, name: ed.name })
            }
        }

        let mut extra_data = vec![];
        if user_notification.extra_data.is_some() {
            for ed in user_notification.extra_data.unwrap() {
                extra_data.push(SendUserNotificationExtraData { value: ed.value, name: ed.name })
            }
        }

        app_state
            .notification_service
            .send_user_notification(
                notification_id,
                user_notification.user_id,
                SendUserNotification {
                    params: Some(params),
                    extra_data: Some(extra_data),
                }
            )
            .await?;
    }

    Ok(HttpResponse::NoContent().finish())
}