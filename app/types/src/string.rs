#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FirebaseToken<T = String>(pub T);

impl From<FirebaseToken> for String {
    fn from(value: FirebaseToken) -> Self {
        value.0.into()
    }
}

impl Into<FirebaseToken> for String {
    fn into(self) -> FirebaseToken {
        FirebaseToken { 0: self }
    }
}

impl From<&FirebaseToken> for String {
    fn from(value: &FirebaseToken) -> Self {
        value.0.to_string()
    }
}