use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shared::{AppError, AppResult};
use uuid::Uuid;

use crate::domain::User;

/// The data stored inside the access token (JWT).
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    lifetime: TimeDelta,
}

impl JwtService {
    pub fn new(secret: &str, lifetime: TimeDelta) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            lifetime,
        }
    }

    pub fn lifetime_in_seconds(&self) -> i64 {
        self.lifetime.num_seconds()
    }

    pub fn create_access_token(&self, user: &User) -> AppResult<String> {
        let now = Utc::now();
        let expires_at = now + self.lifetime;

        let claims = Claims {
            sub: user.id,
            role: user.role.clone(),
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
        };

        let header = Header::new(Algorithm::HS256);

        jsonwebtoken::encode(&header, &claims, &self.encoding_key).map_err(AppError::internal)
    }

    /// Checks the signature and the expiry (`exp`), then returns the claims.
    pub fn verify_access_token(&self, token: &str) -> AppResult<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "exp"]);

        match jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &validation) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(AppError::Unauthorized),
        }
    }
}

/// A random string given to the client. Only its hash is saved in the database.
pub fn generate_refresh_token() -> String {
    let random_bytes: [u8; 32] = rand::random();
    URL_SAFE_NO_PAD.encode(random_bytes)
}

pub fn hash_refresh_token(token: &str) -> String {
    let hash = Sha256::digest(token.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}
