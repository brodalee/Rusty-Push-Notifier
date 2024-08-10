use std::{
    collections::HashMap,
    fs
};
use types::{
    string::FirebaseToken,
    enums::DeviceType,
};
use crate::rows::notification_row::NotificationRow;
use core::{
    error::Error,
    config::Config,
};
use fcm::{
    FcmClient,
    message::{
        Notification,
        AndroidConfig,
        AndroidMessagePriority,
        AndroidNotification,
        ApnsConfig,
        WebpushConfig,
        Message,
        Target
    },
};
use serde::{Deserialize, Serialize};

pub struct NotificationService {
    firebase_client: FcmClient,
}

impl NotificationService {
    pub async fn new() -> Self {
        NotificationService {
            firebase_client: FcmClient::builder()
                .service_account_key_json_path(Config::get_google_service_account_credentials_path())
                .build()
                .await
                .expect("Bad Google credentials given.")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotificationInformation {
    pub title: String,
    pub body: String,
    pub params: Option<Vec<String>>
}

impl NotificationInformation {
    pub fn parse_with_template_data(&mut self, extra_data: &Option<HashMap<String, String>>) -> &Self {
        if extra_data.is_none() {
            return self;
        }

        for (key, value) in extra_data.clone().unwrap() {
            let pattern = format!("%{}%", key);
            self.title = self.title.replace(pattern.clone().as_str(), value.as_str());
            self.body = self.body.replace(pattern.clone().as_str(), value.as_str());
        }

        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotificationsRule<T = HashMap<String, NotificationInformation>>(pub T);

impl NotificationsRule<HashMap<String, NotificationInformation>> {
    pub fn get_by_key(&mut self, key: &str) -> Option<&NotificationInformation> {
        self.0.get(key)
    }
}

impl NotificationService {
    pub async fn send_notification(
        &mut self,
        device_token: &FirebaseToken,
        device_type: &DeviceType,
        notification_row: &NotificationRow,
    ) -> Result<(), Error> {
        let message = self.get_message(device_token, device_type, notification_row)?;
        let response = self.firebase_client.send(message).await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::ProviderError(err.to_string()))
        }
    }
}

impl NotificationService {
    fn get_message(
        &mut self,
        device_token: &FirebaseToken,
        device_type: &DeviceType,
        notification_row: &NotificationRow,
    ) -> Result<Message, Error> {

        let notification_information: Option<NotificationInformation> = self.get_notification_information_by_key(
            notification_row.notification_type.as_str()
        )?;

        if notification_information.is_none() {
            return Err(
                Error::MissingDataError(
                    format!(
                        "Missing key for notifications: {}",
                        notification_row.notification_type.as_str()
                    )
                )
            )
        }

        let extra_data = if notification_row.extra_data.is_none() { None } else {
            let hm: HashMap<String, String> = serde_json::from_str(&notification_row.clone().extra_data.unwrap()).unwrap();
            Some(serde_json::to_value::<HashMap<String, String>>(hm).unwrap())
        };
        let template_data = if notification_row.template_data.is_none() { None } else { Some(serde_json::from_str::<HashMap<String, String>>(&notification_row.clone().template_data.unwrap()).unwrap()) };

        let info = notification_information.unwrap().parse_with_template_data(&template_data).clone();
        let notification = Some(Notification {
            title: Some(info.clone().title),
            body: Some(info.clone().body),
            ..Default::default()
        });

        if device_type.to_string() == DeviceType::Android.to_string() {
            return Ok(Message {
                data: extra_data.clone(),
                notification,
                android: Some(AndroidConfig {
                    data: extra_data.clone(),
                    priority: Some(AndroidMessagePriority::High),
                    notification: Some(AndroidNotification {
                        title: Some(info.clone().title),
                        body: Some(info.clone().body),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                webpush: None,
                apns: None,
                fcm_options: None,
                target: Target::Token(device_token.into()),
            });
        }
        
        if device_type.to_string() == DeviceType::IOS.to_string() {
            return Ok(Message {
                data: extra_data.clone(),
                notification,
                android: None,
                webpush: None,
                apns: Some(ApnsConfig {
                    payload: extra_data.clone(),
                    ..Default::default()
                }),
                fcm_options: None,
                target: Target::Token(device_token.into()),
            });
        }

        Ok(Message {
            data: None,
            notification,
            android: None,
            webpush: Some(WebpushConfig {
                data: extra_data.clone(),
                ..Default::default()
            }),
            apns: None,
            fcm_options: None,
            target: Target::Token(device_token.into()),
        })
    }

    fn get_notification_information_by_key(&mut self, key: &str) -> Result<Option<NotificationInformation>, Error> {
        let contents = fs::read_to_string(Config::get_notification_resources_path())
            .expect("Could not read file");

        let mut content: NotificationsRule = serde_yaml::from_str::<NotificationsRule>(&contents).unwrap();
        Ok(content.get_by_key(key).cloned())
    }
}