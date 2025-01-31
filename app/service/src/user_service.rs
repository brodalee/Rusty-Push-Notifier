use repository::user_repository::UserRepository;
use core::error::Error;
use core::http::user_context::UserContext;
use core::models::users::User;

#[derive(Clone, Debug)]
pub struct UserService {
    pub(crate) user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> UserService {
        UserService {
            user_repository,
        }
    }
}

impl UserService {
    pub async fn update_user_token(&self, user_context: UserContext, token: String) -> Result<(), Error> {
        let user_exists = self
            .user_repository
            .clone()
            .user_exist(user_context.clone())
            .await?;

        if !user_exists {
            self
                .user_repository
                .create_user(user_context.clone(), token)
                .await?;
            return Ok(())
        }

        self
            .user_repository
            .update_user_token(user_context.clone(), token)
            .await?;

        Ok(())
    }

    pub async fn fetch_user_paginated(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<User>, Error> {
        Ok(
            self
                .user_repository
                .fetch_user_paginated(offset, limit)
                .await?
        )
    }

    pub async fn total_count(&self) -> Result<i32, Error> {
        Ok(
            self
                .user_repository
                .count()
                .await?
        )
    }

    pub async fn fetch_by_id(&self, id: i32) -> Result<User, Error> {
        Ok(
            self
                .user_repository
                .fetch_by_id(id)
                .await?
        )
    }
}