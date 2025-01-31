use core::error::Error;
use repository::notification_history_repository::NotificationHistoryRepository;
use repository::notification_repository::NotificationRepository;
use core::models::notifications::NotificationWithParameters;
use crate::user_service::UserService;
use core::models::notifications::SendUserNotification;
use crate::firebase_notification_service::FirebaseNotificationService;
use core::models::notifications::SendUserNotificationExtraData;
use core::models::notifications::SendUserParameter;
use core::models::notifications::NotificationParamIdWithValue;

#[derive(Clone)]
pub struct NotificationService {
    pub notification_repository: NotificationRepository,
    pub notification_history_repository: NotificationHistoryRepository,
    pub user_service: UserService,
    pub firebase_service: FirebaseNotificationService
}

impl NotificationService {
    pub fn new(
        notification_repository: NotificationRepository,
        notification_history_repository: NotificationHistoryRepository,
        user_service: UserService,
        firebase_service: FirebaseNotificationService
    ) -> NotificationService {
        NotificationService {
            notification_repository,
            notification_history_repository,
            user_service,
            firebase_service,
        }
    }
}

impl NotificationService {
    pub async fn create_notification(
        &self,
        notification: NotificationWithParameters
    ) -> Result<String, Error> {
        let already_exist = self
            .notification_repository
            .exist_by_name(String::from(notification.clone().name))
            .await?;

        if already_exist {
            return Err(Error::AlreadyExistError(format!("Notification with name '{}' already exist", String::from(notification.clone().name))));
        }

        let id = self
            .notification_repository
            .create_notification(notification)
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?;

        Ok(id)
    }

    pub async fn fetch_notifications_paginated(
        &self,
        offset: i32,
        limit: i32
    ) -> Result<Vec<NotificationWithParameters>, Error> {
        if offset < 0 {
            return Err(Error::ValidationError("Offset must be superior than 0".to_string()));
        }

        let notifications = self
            .notification_repository
            .fetch_paginated(offset * limit, limit)
            .await?;

        let mut map_notifications: Vec<NotificationWithParameters> = vec![];
        for notification in notifications {
            let parameters = self
                .notification_repository
                .fetch_notification_parameters(notification.clone().id).await?;

            map_notifications.push(NotificationWithParameters {
                id: notification.clone().id,
                name: notification.clone().name,
                title: notification.clone().title,
                content: notification.clone().content,
                parameters,
            })
        }

        Ok(map_notifications)
    }

    pub async fn total_count(&self) -> Result<i32, Error> {
        Ok(
            self
                .notification_repository
                .total_count()
                .await?
        )
    }

    pub async fn send_user_notification(
        &self,
        notification_id: i32,
        user_id: i32,
        notification_input: SendUserNotification
    ) -> Result<(), Error> {
        let notification = self
            .notification_repository
            .fetch_by_id(notification_id)
            .await?;

        let user = self
            .user_service
            .fetch_by_id(user_id)
            .await?;

        if user.token.is_none() {
            return Err(Error::MissingDataError("User dont have device token".to_string()))
        }

        let notification_parameters = self
            .notification_repository
            .fetch_notification_parameters(notification_id)
            .await?;

        let mapped_parameters: Vec<String> = notification_parameters
            .clone()
            .iter()
            .map(|n| n.name.clone())
            .collect();

        let mut parameters: Vec<SendUserParameter> = vec![];
        if notification_input.params.is_some() {
            let params = notification_input.params.unwrap();
            for p in params {
                parameters.push(p.into());
            }
        }

        let current_parameters: Vec<String> = parameters
            .iter()
            .map(|n| n.name.clone())
            .collect();

        if !mapped_parameters.iter().all(|np| current_parameters.contains(np)) {
            return Err(Error::ValidationError("Missing parameters".to_string()));
        }

        let mut extra_data: Vec<SendUserNotificationExtraData> = vec![];
        if notification_input.extra_data.is_some() {
            let ed = notification_input.extra_data.unwrap();
            for ned in ed {
                extra_data.push(ned.into())
            }
        }

        self
            .firebase_service
            .send_notification(
                user.device_type,
                user.token.unwrap(),
                notification.clone(),
                parameters.clone(),
                extra_data.clone()
            )
            .await?;

        let mut mapped_params: Vec<NotificationParamIdWithValue> = vec![];
        for params in parameters.clone() {
            for nparam in notification_parameters.clone() {
                if params.name == nparam.name {
                    mapped_params.push(NotificationParamIdWithValue {
                        id: nparam.id,
                        value: params.clone().value
                    });
                    break;
                }
            }
        }

        self
            .notification_history_repository
            .create_sent(
                notification.clone(),
                mapped_params,
                extra_data.clone(),
                user.id
            )
            .await?;

        // TODO : il faudrat voir pour que le créateur de la notif
        //  puisse envoyer le type de priorité ( Hight, Medium .. )

        // TODO : si on arrive pas à enregistrer, il
        //  faudrait proposer un fallback en mode "retry_later_on_fail": true/false
        Ok(())
    }
}

impl NotificationService {

}