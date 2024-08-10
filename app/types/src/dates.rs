use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreationDate<T = DateTime<Utc>>(pub T);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UpdateDate<T = DateTime<Utc>>(pub T);