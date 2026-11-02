use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<FixedOffset>,
}

pub struct UserWithPassword {
    pub user: User,
    pub password_hash: String,
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub struct RefreshToken {
    pub user_id: Uuid,
    pub expires_at: DateTime<FixedOffset>,
    pub revoked_at: Option<DateTime<FixedOffset>>,
}

pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in_seconds: i64,
}

pub struct AuthResult {
    pub user: User,
    pub tokens: Tokens,
}
