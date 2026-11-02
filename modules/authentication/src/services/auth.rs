use chrono::{TimeDelta, Utc};
use shared::{AppError, AppResult};
use uuid::Uuid;

use crate::{
    domain::{AuthResult, LoginInput, NewUser, RegisterInput, Tokens, User},
    repository::AuthRepository,
    services::{
        password,
        token::{self, Claims, JwtService},
    },
};

const USERNAME_MIN_LENGTH: usize = 3;
const USERNAME_MAX_LENGTH: usize = 50;
const EMAIL_MAX_LENGTH: usize = 255;
const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 128;

#[derive(Clone)]
pub struct AuthService {
    repository: AuthRepository,
    jwt: JwtService,
    refresh_token_lifetime: TimeDelta,
}

impl AuthService {
    pub fn new(
        repository: AuthRepository,
        jwt: JwtService,
        refresh_token_lifetime: TimeDelta,
    ) -> Self {
        Self {
            repository,
            jwt,
            refresh_token_lifetime,
        }
    }

    pub async fn register(&self, input: RegisterInput) -> AppResult<AuthResult> {
        let username = input.username.trim().to_string();
        let email = normalize_email(&input.email);

        validate_username(&username)?;
        validate_email(&email)?;
        validate_password(&input.password)?;

        let password_hash = password::hash(input.password).await?;

        let new_user = NewUser {
            username,
            email,
            password_hash,
        };
        let user = self.repository.create_user(&new_user).await?;

        let tokens = self.create_tokens(&user).await?;

        Ok(AuthResult { user, tokens })
    }

    pub async fn login(&self, input: LoginInput) -> AppResult<AuthResult> {
        let email = normalize_email(&input.email);

        // Same error for "wrong email" and "wrong password",
        // so nobody can find out which emails are registered.
        let found = match self.repository.find_user_by_email(&email).await? {
            Some(found) => found,
            None => return Err(AppError::Unauthorized),
        };

        let password_is_correct = password::verify(input.password, found.password_hash).await?;
        if !password_is_correct {
            return Err(AppError::Unauthorized);
        }

        if !found.user.is_active {
            return Err(AppError::Unauthorized);
        }

        let tokens = self.create_tokens(&found.user).await?;

        Ok(AuthResult {
            user: found.user,
            tokens,
        })
    }

    pub async fn refresh(&self, refresh_token: &str) -> AppResult<Tokens> {
        let old_token_hash = token::hash_refresh_token(refresh_token);

        let stored_token = match self.repository.find_refresh_token(&old_token_hash).await? {
            Some(stored_token) => stored_token,
            None => return Err(AppError::Unauthorized),
        };

        // A revoked token used again means it was probably stolen,
        // so we log the user out of every device.
        if stored_token.revoked_at.is_some() {
            self.repository
                .revoke_all_refresh_tokens(stored_token.user_id)
                .await?;
            return Err(AppError::Unauthorized);
        }

        if stored_token.expires_at < Utc::now() {
            return Err(AppError::Unauthorized);
        }

        let user = self.get_active_user(stored_token.user_id).await?;

        let new_refresh_token = token::generate_refresh_token();
        let new_token_hash = token::hash_refresh_token(&new_refresh_token);
        let expires_at = Utc::now() + self.refresh_token_lifetime;

        let replaced = self
            .repository
            .replace_refresh_token(
                &old_token_hash,
                user.id,
                &new_token_hash,
                expires_at.fixed_offset(),
            )
            .await?;

        // Another request used the same token at the same moment.
        if !replaced {
            self.repository.revoke_all_refresh_tokens(user.id).await?;
            return Err(AppError::Unauthorized);
        }

        let access_token = self.jwt.create_access_token(&user)?;

        Ok(Tokens {
            access_token,
            refresh_token: new_refresh_token,
            expires_in_seconds: self.jwt.lifetime_in_seconds(),
        })
    }

    pub async fn get_active_user(&self, user_id: Uuid) -> AppResult<User> {
        let user = match self.repository.find_user_by_id(user_id).await? {
            Some(user) => user,
            None => return Err(AppError::Unauthorized),
        };

        if !user.is_active {
            return Err(AppError::Unauthorized);
        }

        Ok(user)
    }

    pub fn verify_access_token(&self, access_token: &str) -> AppResult<Claims> {
        self.jwt.verify_access_token(access_token)
    }

    async fn create_tokens(&self, user: &User) -> AppResult<Tokens> {
        let refresh_token = token::generate_refresh_token();
        let refresh_token_hash = token::hash_refresh_token(&refresh_token);
        let expires_at = Utc::now() + self.refresh_token_lifetime;

        self.repository
            .save_refresh_token(user.id, &refresh_token_hash, expires_at.fixed_offset())
            .await?;

        let access_token = self.jwt.create_access_token(user)?;

        Ok(Tokens {
            access_token,
            refresh_token,
            expires_in_seconds: self.jwt.lifetime_in_seconds(),
        })
    }
}

fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

fn validate_username(username: &str) -> AppResult<()> {
    let length = username.chars().count();

    if length < USERNAME_MIN_LENGTH {
        return Err(AppError::Validation(format!(
            "Username must be at least {USERNAME_MIN_LENGTH} characters"
        )));
    }

    if length > USERNAME_MAX_LENGTH {
        return Err(AppError::Validation(format!(
            "Username must be at most {USERNAME_MAX_LENGTH} characters"
        )));
    }

    for character in username.chars() {
        let allowed = character.is_ascii_alphanumeric()
            || character == '_'
            || character == '.'
            || character == '-';

        if !allowed {
            return Err(AppError::Validation(
                "Username may only contain letters, digits, '_', '.' and '-'".to_string(),
            ));
        }
    }

    Ok(())
}

fn validate_email(email: &str) -> AppResult<()> {
    if email.len() > EMAIL_MAX_LENGTH {
        return Err(AppError::Validation("Email is too long".to_string()));
    }

    let parts: Vec<&str> = email.split('@').collect();
    let is_valid = parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.');

    if !is_valid {
        return Err(AppError::Validation("Email is not valid".to_string()));
    }

    Ok(())
}

fn validate_password(password: &str) -> AppResult<()> {
    let length = password.chars().count();

    if length < PASSWORD_MIN_LENGTH {
        return Err(AppError::Validation(format!(
            "Password must be at least {PASSWORD_MIN_LENGTH} characters"
        )));
    }

    if length > PASSWORD_MAX_LENGTH {
        return Err(AppError::Validation(format!(
            "Password must be at most {PASSWORD_MAX_LENGTH} characters"
        )));
    }

    Ok(())
}
