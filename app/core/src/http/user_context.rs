use serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Android,
    IOS,
    Web,
    Unknown,
}

impl DeviceType {
    pub fn to_string(&self) -> String {
        self.clone().into()
    }
}

impl Into<DeviceType> for String {
    fn into(self) -> DeviceType {
        match self.as_str() {
            "android" => DeviceType::Android,
            "ios" => DeviceType::IOS,
            "web" => DeviceType::Web,
            _ => DeviceType::Unknown,
        }
    }
}

impl Into<String> for DeviceType {
    fn into(self) -> String {
        match self {
            DeviceType::Android => "android".to_string(),
            DeviceType::IOS => "ios".to_string(),
            DeviceType::Web => "web".to_string(),
            DeviceType::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserContext {
    pub device_id: String,
    pub device_type: DeviceType,
}

// TODO : place into middleware.
pub fn get_user_context(req: actix_web::HttpRequest) -> Result<UserContext, Error> {
    let device_id_header = req.headers().get("X-DEVICE-ID")
        .ok_or_else(|| Error::HeaderError("Missing X-DEVICE-ID header".to_string()))
        .map_err(|e| e)?
        .to_str()
        .unwrap();

    let device_type_header = req.headers().get("X-DEVICE-TYPE")
        .ok_or_else(|| Error::HeaderError("Missing X-DEVICE-TYPE header".to_string()))
        .map_err(|e| e)?
        .to_str()
        .unwrap();

    let device_type = DeviceType::try_from(String::from(device_type_header))
        .map_err(|_| Error::HeaderError("Bad Device Type given".to_string()))?;

    Ok(
        UserContext {
            device_id: String::from(device_id_header),
            device_type,
        }
    )
}