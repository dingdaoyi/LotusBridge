use crate::config::auth::get_auth_config;
use crate::config::error::{AuthError, EdgeError};
use crate::models::R;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, Json};
use chrono::{Duration, Utc};
use headers::authorization::{Bearer, Credentials};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

// 用户登录
pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Json<R<String>>, EdgeError> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(EdgeError::AuthError(AuthError::MissingCredentials));
    }

    let auth_config = match get_auth_config() {
        Some(config) => config,
        None => return Err(EdgeError::AuthError(AuthError::MissingCredentials)),
    };

    if !(auth_config.username == payload.username && auth_config.password == payload.password) {
        return Err(EdgeError::Message("用户名或密码错误".into()));
    }

    let token = generate_jwt_token(
        &payload.username,
        &auth_config.jwt_secret,
        auth_config.expire_minutes,
    )?;
    // 将 JWT 令牌发送到响应中
    Ok(Json(R::success_with_data(token)))
}

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send,
{
    type Rejection = EdgeError;

    async fn from_request_parts(parts: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let token = match parts.headers.get("Authorization").and_then(Bearer::decode) {
            Some(token) => token,
            None => return Err(EdgeError::AuthError(AuthError::MissingCredentials)),
        };
        // 解码用户数据
        let auth_config = match get_auth_config() {
            Some(config) => config,
            None => return Err(EdgeError::AuthError(AuthError::MissingCredentials)),
        };

        let token_data = decode::<Claims>(
            token.token(),
            &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|error| {
            tracing::error!("转换错误{}", error);
            AuthError::InvalidToken
        })?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    exp: usize, // 强制性过期时间，使用 UTC 时间戳
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

fn generate_jwt_token(
    username: &str,
    jwt_secret: &str,
    expire_minutes: usize,
) -> Result<String, AuthError> {
    let current_time = Utc::now();
    let expiration_time = current_time + Duration::minutes(expire_minutes as i64);

    let claims = Claims {
        username: username.to_owned(),
        exp: expiration_time.timestamp() as usize,
    };

    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreation)
}
