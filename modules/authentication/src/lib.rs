mod domain;
mod extractor;
mod http;
mod repository;
mod services;

use axum::Router;
use chrono::TimeDelta;
use deadpool_postgres::Pool;

use repository::AuthRepository;
use services::{auth::AuthService, token::JwtService};

pub use extractor::CurrentUser;

pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_token_lifetime: TimeDelta,
    pub refresh_token_lifetime: TimeDelta,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        if jwt_secret.len() < 32 {
            panic!("JWT_SECRET must be at least 32 characters long");
        }

        Self {
            jwt_secret,
            access_token_lifetime: TimeDelta::minutes(15),
            refresh_token_lifetime: TimeDelta::days(7),
        }
    }
}

pub fn router(db: Pool, config: AuthConfig) -> Router {
    let repository = AuthRepository::new(db);
    let jwt = JwtService::new(&config.jwt_secret, config.access_token_lifetime);
    let service = AuthService::new(repository, jwt, config.refresh_token_lifetime);

    http::router(service)
}
