use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationParameter {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationWithParameters {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub content: String,
    pub parameters: Vec<NotificationParameter>,
}

// ROWS Definitions
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct NotificationRow {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub content: String,
}

impl Into<Notification> for NotificationRow {
    fn into(self) -> Notification {
        Notification {
            id: self.id,
            name: self.name,
            content: self.content,
            title: self.title,
        }
    }
}

impl Into<Notification> for &NotificationRow {
    fn into(self) -> Notification {
        Notification {
            id: self.clone().id,
            name: self.clone().name,
            content: self.clone().content,
            title: self.clone().title,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct NotificationParameterRow {
    pub id: i32,
    pub name: String,
}

impl Into<NotificationParameter> for &NotificationParameterRow {
    fn into(self) -> NotificationParameter {
        NotificationParameter {
            id: self.id,
            name: self.clone().name,
        }
    }
}

impl Into<NotificationParameter> for NotificationRow {
    fn into(self) -> NotificationParameter {
        NotificationParameter {
            name: self.content,
            id: self.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SendUserParameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct SendUserNotificationExtraData {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct SendUserNotification {
    pub params: Option<Vec<SendUserParameter>>,
    pub extra_data: Option<Vec<SendUserNotificationExtraData>>,
}

pub enum NotificationHistoryStatus {
    Sent,
    InWait,
    Failed,
    Canceled,
}

impl Into<String> for NotificationHistoryStatus {
    fn into(self) -> String {
        match self {
            NotificationHistoryStatus::Sent => "Sent".to_string(),
            NotificationHistoryStatus::InWait => "InWait".to_string(),
            NotificationHistoryStatus::Failed => "Failed".to_string(),
            NotificationHistoryStatus::Canceled => "Canceled".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotificationParamIdWithValue {
    pub id: i32,
    pub value: String,
}