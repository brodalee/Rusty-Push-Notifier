use std::collections::HashMap;
use std::sync::{Arc};
use fcm::FcmClient;
use fcm::message::{AndroidConfig, AndroidMessagePriority, AndroidNotification, ApnsConfig, Message, Notification, Target, WebpushConfig};
use core::error::Error;
use core::http::user_context::DeviceType;
use core::models::notifications::{
    Notification as NotificationModel,
    SendUserNotificationExtraData,
    SendUserParameter,
};

#[derive(Clone)]
pub struct FirebaseNotificationService {
    pub(crate) firebase_client: Arc<FcmClient>,
}

impl FirebaseNotificationService {
    pub fn new(firebase_client: Arc<FcmClient>) -> Self {
        FirebaseNotificationService {
            firebase_client
        }
    }
}

impl FirebaseNotificationService {
    pub async fn send_notification(
        &self,
        device_type: DeviceType,
        device_token: String,
        notification: NotificationModel,
        params: Vec<SendUserParameter>,
        extra_data: Vec<SendUserNotificationExtraData>,
    ) -> Result<(), Error> {
        let msg = self.make_message(
            device_token,
            device_type,
            notification,
            params,
            extra_data,
        )?;

        self
            .firebase_client
            .send(msg)
            .await
            .map_err(|e| Error::ProviderError(e.to_string()))?;

        Ok(())
    }
}

impl FirebaseNotificationService {
    fn make_message(
        &self,
        device_token: String,
        device_type: DeviceType,
        notification_info: NotificationModel,
        params: Vec<SendUserParameter>,
        extra_data: Vec<SendUserNotificationExtraData>,
    ) -> Result<Message, Error> {
        let mut hm_extra_data: HashMap<String, String> = HashMap::new();
        extra_data.iter().for_each(|ed| {
            hm_extra_data.insert(ed.name.clone(), ed.value.clone());
        });

        let mut final_content = notification_info.clone().content;
        params.iter().for_each(|p| {
            final_content = final_content.replace(p.name.as_str(), p.value.as_str());
        });

        let extra_data = Some(serde_json::to_value::<HashMap<String, String>>(hm_extra_data).unwrap());

        let info = notification_info.clone();
        let notification = Some(Notification {
            title: Some(info.clone().title),
            body: Some(info.clone().content),
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
                        title: Some(info.title),
                        body: Some(info.content),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                webpush: None,
                apns: None,
                fcm_options: None,
                target: Target::Token(device_token),
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
                target: Target::Token(device_token),
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
            target: Target::Token(device_token),
        })
    }
}