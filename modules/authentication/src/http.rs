use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use shared::AppResult;
use uuid::Uuid;

use crate::{
    domain::{AuthResult, LoginInput, RegisterInput, Tokens, User},
    extractor::CurrentUser,
    services::auth::AuthService,
};

pub fn router(service: AuthService) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/me", get(me))
        .with_state(service)
}

// ---------- Requests (JSON coming in) ----------

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

// ---------- Responses (JSON going out) ----------

#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    username: String,
    email: String,
    role: String,
    is_active: bool,
    created_at: DateTime<FixedOffset>,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Serialize)]
struct AuthResponse {
    user: UserResponse,
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

impl From<Tokens> for TokenResponse {
    fn from(tokens: Tokens) -> Self {
        Self {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: tokens.expires_in_seconds,
        }
    }
}

impl From<AuthResult> for AuthResponse {
    fn from(result: AuthResult) -> Self {
        Self {
            user: UserResponse::from(result.user),
            access_token: result.tokens.access_token,
            refresh_token: result.tokens.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: result.tokens.expires_in_seconds,
        }
    }
}

// ---------- Handlers ----------

async fn register(
    State(service): State<AuthService>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    let input = RegisterInput {
        username: request.username,
        email: request.email,
        password: request.password,
    };

    let result = service.register(input).await?;

    Ok((StatusCode::CREATED, Json(AuthResponse::from(result))))
}

async fn login(
    State(service): State<AuthService>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let input = LoginInput {
        email: request.email,
        password: request.password,
    };

    let result = service.login(input).await?;

    Ok(Json(AuthResponse::from(result)))
}

async fn refresh(
    State(service): State<AuthService>,
    Json(request): Json<RefreshRequest>,
) -> AppResult<Json<TokenResponse>> {
    let tokens = service.refresh(&request.refresh_token).await?;

    Ok(Json(TokenResponse::from(tokens)))
}

async fn me(
    State(service): State<AuthService>,
    current_user: CurrentUser,
) -> AppResult<Json<UserResponse>> {
    let user = service.get_active_user(current_user.id).await?;

    Ok(Json(UserResponse::from(user)))
}
