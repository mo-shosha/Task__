use chrono::{DateTime, FixedOffset};
use cornucopia::queries::authentication as queries;
use deadpool_postgres::{Object, Pool};
use shared::{AppError, AppResult};
use tokio_postgres::error::SqlState;
use uuid::Uuid;

use crate::domain::{NewUser, RefreshToken, User, UserWithPassword};

#[derive(Clone)]
pub struct AuthRepository {
    db: Pool,
}

impl AuthRepository {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }

    async fn client(&self) -> AppResult<Object> {
        self.db.get().await.map_err(AppError::internal)
    }

    pub async fn create_user(&self, input: &NewUser) -> AppResult<User> {
        let client = self.client().await?;

        let result = queries::create_user()
            .bind(&client, &input.username, &input.email, &input.password_hash)
            .one()
            .await;

        let row = match result {
            Ok(row) => row,
            Err(error) => return Err(create_user_error(error)),
        };

        Ok(User {
            id: row.id,
            username: row.username,
            email: row.email,
            role: row.role,
            is_active: row.is_active,
            created_at: row.created_at,
        })
    }

    pub async fn find_user_by_email(&self, email: &str) -> AppResult<Option<UserWithPassword>> {
        let client = self.client().await?;

        let row = queries::get_user_by_email()
            .bind(&client, &email)
            .opt()
            .await
            .map_err(AppError::database)?;

        match row {
            Some(row) => Ok(Some(UserWithPassword {
                user: User {
                    id: row.id,
                    username: row.username,
                    email: row.email,
                    role: row.role,
                    is_active: row.is_active,
                    created_at: row.created_at,
                },
                password_hash: row.password_hash,
            })),
            None => Ok(None),
        }
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let client = self.client().await?;

        let row = queries::get_user_by_id()
            .bind(&client, &id)
            .opt()
            .await
            .map_err(AppError::database)?;

        match row {
            Some(row) => Ok(Some(User {
                id: row.id,
                username: row.username,
                email: row.email,
                role: row.role,
                is_active: row.is_active,
                created_at: row.created_at,
            })),
            None => Ok(None),
        }
    }

    pub async fn save_refresh_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<FixedOffset>,
    ) -> AppResult<()> {
        let client = self.client().await?;

        queries::create_refresh_token()
            .bind(&client, &user_id, &token_hash, &expires_at)
            .one()
            .await
            .map_err(AppError::database)?;

        Ok(())
    }

    pub async fn find_refresh_token(&self, token_hash: &str) -> AppResult<Option<RefreshToken>> {
        let client = self.client().await?;

        let row = queries::get_refresh_token()
            .bind(&client, &token_hash)
            .opt()
            .await
            .map_err(AppError::database)?;

        match row {
            Some(row) => Ok(Some(RefreshToken {
                user_id: row.user_id,
                expires_at: row.expires_at,
                revoked_at: row.revoked_at,
            })),
            None => Ok(None),
        }
    }

    /// Revokes the old token and saves the new one in a single transaction.
    /// Returns `false` if the old token was already revoked.
    pub async fn replace_refresh_token(
        &self,
        old_token_hash: &str,
        user_id: Uuid,
        new_token_hash: &str,
        expires_at: DateTime<FixedOffset>,
    ) -> AppResult<bool> {
        let mut client = self.client().await?;
        let transaction = client.transaction().await.map_err(AppError::database)?;

        let revoked_rows = queries::revoke_refresh_token()
            .bind(&transaction, &old_token_hash)
            .await
            .map_err(AppError::database)?;

        if revoked_rows == 0 {
            return Ok(false);
        }

        queries::create_refresh_token()
            .bind(&transaction, &user_id, &new_token_hash, &expires_at)
            .one()
            .await
            .map_err(AppError::database)?;

        transaction.commit().await.map_err(AppError::database)?;

        Ok(true)
    }

    pub async fn revoke_all_refresh_tokens(&self, user_id: Uuid) -> AppResult<()> {
        let client = self.client().await?;

        queries::revoke_all_user_refresh_tokens()
            .bind(&client, &user_id)
            .await
            .map_err(AppError::database)?;

        Ok(())
    }
}

/// Turns "duplicate email/username" database errors into a 409 Conflict.
fn create_user_error(error: tokio_postgres::Error) -> AppError {
    if error.code() != Some(&SqlState::UNIQUE_VIOLATION) {
        return AppError::database(error);
    }

    let mut constraint = "";
    if let Some(db_error) = error.as_db_error() {
        constraint = db_error.constraint().unwrap_or("");
    }

    let message = match constraint {
        "users_email_key" => "Email is already registered",
        "users_username_key" => "Username is already taken",
        _ => "User already exists",
    };

    AppError::Conflict(message.to_string())
}
