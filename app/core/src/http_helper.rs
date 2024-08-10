use std::str::FromStr;
use actix_web::HttpRequest;
use types::{device_id::DeviceId, enums::DeviceType};
use crate::error::Error;
use crate::user_context::UserContext;

struct UserContextHeader {}

impl UserContextHeader {
    const DEVICE_ID_HEADER_NAME: &'static str = "X-DEVICE-ID";
    const DEVICE_TYPE_HEADER_NAME: &'static str = "X-DEVICE-TYPE";
}

pub fn get_user_context(req: HttpRequest) -> Result<UserContext, Error> {
    let device_id_header = req.headers().get(UserContextHeader::DEVICE_ID_HEADER_NAME);
    let device_type_header = req.headers().get(UserContextHeader::DEVICE_TYPE_HEADER_NAME);

    if device_id_header.is_none() {
        return Err(Error::HeaderError("Missing X-DEVICE-ID header".to_string()))
    }

    if device_type_header.is_none() {
        return Err(Error::HeaderError("Missing X-DEVICE-TYPE header".to_string()))
    }

    let device_type = DeviceType::from_str(device_type_header.unwrap().to_str().unwrap());
    match device_type {
        Err(_) => {
            Err(Error::HeaderError("Bad Device Type given".to_string()))
        }
        Ok(device_type) => Ok(UserContext {
            device_id: DeviceId(String::from(device_id_header.unwrap().to_str().unwrap())),
            device_type
        })
    }
}