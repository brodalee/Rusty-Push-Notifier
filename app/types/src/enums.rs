use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeviceType {
    Android,
    IOS,
}

impl From<DeviceType> for String {
    fn from(value: DeviceType) -> Self {
        value.to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NotificationType {
    Test,
    TestWithTemplate,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NotificationStatus {
    InProgress,
    Canceled,
    Failed,
    Sent
}

impl From<NotificationStatus> for String {
    fn from(value: NotificationStatus) -> Self {
        value.to_string()
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for DeviceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Android" => Ok(DeviceType::Android),
            "iOS" => Ok(DeviceType::IOS),
            _ => Err(())
        }
    }
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for NotificationType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Test" => Ok(NotificationType::Test),
            "TestWithTemplate" => Ok(NotificationType::TestWithTemplate),
            _ => Err(())
        }
    }
}

impl From<String> for NotificationType {
    fn from(value: String) -> Self {
        NotificationType::from_str(value.as_str()).unwrap()
    }
}

impl From<NotificationType> for String {
    fn from(value: NotificationType) -> Self {
        value.to_string()
    }
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for NotificationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sent" => Ok(NotificationStatus::Sent),
            "Canceled" => Ok(NotificationStatus::Canceled),
            "Failed" => Ok(NotificationStatus::Failed),
            "InProgress" => Ok(NotificationStatus::InProgress),
            _ => Err(())
        }
    }
}

impl From<String> for DeviceType {
    fn from(value: String) -> Self {
        DeviceType::from(value.as_str())
    }
}

impl From<&str> for DeviceType {
    fn from(value: &str) -> Self {
        DeviceType::from_str(value).unwrap()
    }
}