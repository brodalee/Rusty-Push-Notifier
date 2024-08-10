use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersWithExtraDataDto {
    pub id: i32,
    pub extra_data: Option<HashMap<String, String>>,
    pub template_data: Option<HashMap<String, String>>
}

pub type ListOfUsersWithExtraData = Vec<UsersWithExtraDataDto>;