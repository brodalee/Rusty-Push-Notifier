use serde::{Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Clone, Debug, ToSchema)]
pub struct ErrorResponse {
    pub message: String
}