use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier},
};
use shared::{AppError, AppResult};

// Argon2 is slow on purpose, so it runs on a background thread
// instead of blocking the async server.

pub async fn hash(password: String) -> AppResult<String> {
    let task = tokio::task::spawn_blocking(move || hash_now(&password));

    match task.await {
        Ok(result) => result,
        Err(error) => Err(AppError::internal(error)),
    }
}

pub async fn verify(password: String, password_hash: String) -> AppResult<bool> {
    let task = tokio::task::spawn_blocking(move || verify_now(&password, &password_hash));

    match task.await {
        Ok(is_valid) => Ok(is_valid),
        Err(error) => Err(AppError::internal(error)),
    }
}

fn hash_now(password: &str) -> AppResult<String> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(AppError::internal)?;

    Ok(hash.to_string())
}

fn verify_now(password: &str, password_hash: &str) -> bool {
    let result = Argon2::default().verify_password(password.as_bytes(), password_hash);
    result.is_ok()
}
