#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeviceId<T = String>(pub T);

impl From<DeviceId> for String {
    fn from(value: DeviceId) -> Self {
        value.0.into()
    }
}