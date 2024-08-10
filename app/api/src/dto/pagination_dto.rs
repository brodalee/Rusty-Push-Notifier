use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use core::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct PaginationDto {
    pub page: String,
    pub limit: String
}

impl PaginationDto {
    pub fn validate(&mut self) -> Result<&mut Self, Error> {
        match self.validate_page() {
            Ok(_) => {
                match self.validate_limit() {
                    Ok(_) => Ok(self),
                    Err(err) => Err(err)
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_offset(&mut self) -> i32 {
        self.page.parse::<i32>().unwrap() - 1
    }

    pub fn get_limit(&mut self) -> i32 {
        self.limit.parse::<i32>().unwrap()
    }

    pub fn validate_limit(&mut self) -> Result<(), Error> {
        match self.limit.parse::<i32>() {
            Ok(limit) => {
                if limit > 100 {
                    return Err(Error::ValidationError("limit must be inferior or equal than 100".to_string()));
                } else if limit < 10 {
                    return Err(Error::ValidationError("limit must be superior or equal than 10".to_string()));
                }

                Ok(())
            },
            Err(_) => Err(Error::ValidationError("limit must be int".to_string()))
        }
    }

    fn validate_page(&mut self) -> Result<(), Error> {
        match self.page.parse::<i32>() {
            Ok(page) => {
                if page <= 0 {
                    return Err(Error::ValidationError("page must be superior than 0".to_string()))
                }

                Ok(())
            },
            Err(_) => Err(Error::ValidationError("page must be int".to_string()))
        }
    }
}