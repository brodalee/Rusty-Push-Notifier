use chrono::Utc;
use redis::{from_redis_value, Value};
use serde::{Deserialize, Serialize};
use types::{
    enums::NotificationStatus,
    dates::{UpdateDate, CreationDate}
};
use crate::notification_history_service::NotificationHistory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRow {
    pub user_id: i32,
    pub notification_type: String,
    pub number_of_tries: i32,
    pub extra_data: Option<String>,
    pub template_data: Option<String>,
}

impl From<&Value> for NotificationRow {
    fn from(value: &Value) -> Self {
        let json_str: String = from_redis_value(value).ok().unwrap();
        serde_json::from_str::<NotificationRow>(&json_str).unwrap()
    }
}

impl From<std::option::Option<&redis::Value>> for NotificationRow {
    fn from(value: Option<&Value>) -> Self {
        let json_str: String = from_redis_value(value.unwrap()).ok().unwrap();
        serde_json::from_str::<NotificationRow>(&json_str).unwrap()
    }
}

impl NotificationRow {
    pub fn as_sent_notification(&mut self) -> NotificationHistory {
        self.transform_into_notification_history(NotificationStatus::Sent)
    }

    pub fn as_failed_notification(&mut self) -> NotificationHistory {
        self.transform_into_notification_history(NotificationStatus::Failed)
    }

    fn transform_into_notification_history(
        &mut self,
        notification_status: NotificationStatus
    ) -> NotificationHistory {
        NotificationHistory {
            user_id: self.clone().user_id.into(),
            notification_type: self.clone().notification_type.into(),
            notification_status,
            creation_date: CreationDate { 0: Utc::now() },
            update_date: UpdateDate { 0: Utc::now() },
        }
    }
}