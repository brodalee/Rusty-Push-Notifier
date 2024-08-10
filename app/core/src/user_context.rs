use types::{device_id::DeviceId, enums::DeviceType};

#[derive(Clone, Debug)]
pub struct UserContext {
    pub device_id: DeviceId,
    pub device_type: DeviceType,
}