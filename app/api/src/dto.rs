use core::error::Error;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct PaginationDto {
    pub page: String,
    pub limit: String
}

impl PaginationDto {
    pub fn validate(&mut self) -> Result<&mut Self, Error> {
        self.validate_page()
            .map_err(|e| e)?;

        self.validate_limit()
            .map_err(|e| e)?;

        Ok(self)
    }

    pub fn get_offset(&mut self) -> i32 {
        self.page.parse::<i32>().unwrap() - 1
    }

    pub fn get_limit(&mut self) -> i32 {
        self.limit.parse::<i32>().unwrap()
    }

    pub fn validate_limit(&mut self) -> Result<(), Error> {
        let limit = self
            .limit
            .parse::<i32>()
            .map_err(|_| Error::ValidationError("Limit must be integer".to_string()))?;

        if limit > 100 || limit < 10 {
            return Err(Error::ValidationError("limit must be between 10 and 100".to_string()))
        }

        Ok(())
    }

    fn validate_page(&mut self) -> Result<(), Error> {
        let page = self
            .page
            .parse::<i32>()
            .map_err(|_| Error::ValidationError("Page must be integer".to_string()))?;

        if page <= 0 {
            return Err(Error::ValidationError("page must be superior than 0".to_string()))
        }

        Ok(())
    }
}