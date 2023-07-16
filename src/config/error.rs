use std::fmt::Display;
use std::sync::PoisonError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::{ValidationErrors};
use crate::models::R;

pub type Result<T> = std::result::Result<T, EdgeError>;

/// 平台统一异常
#[derive(Debug)]
pub enum EdgeError {
    /// sqlx异常
    SqlxError(sqlx::error::Error),
    ValidationErrors(ValidationErrors),
    AuthError(AuthError),
    Message(String),
}
use std::fmt;

// ... (之前的代码保持不变)

impl Display for EdgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeError::SqlxError(sqlx::error::Error::Io(err)) => write!(f, "数据库处理异常: {}", err),
            EdgeError::SqlxError(err) => write!(f, "数据库未知异常: {}", err),
            EdgeError::Message(err) => write!(f, "{}", err),
            EdgeError::ValidationErrors(validation_errors) => write!(f, "{}", validation_errors),
            EdgeError::AuthError(auth) => {
                match auth {
                    AuthError::WrongCredentials => write!(f, "认证失败: Wrong credentials"),
                    AuthError::MissingCredentials => write!(f, "认证失败: Missing credentials"),
                    AuthError::TokenCreation => write!(f, "认证失败: Token creation error"),
                    AuthError::InvalidToken => write!(f, "认证失败: Invalid token"),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for EdgeError {
    fn from(value: AuthError) -> Self {
        EdgeError::AuthError(value)
    }
}

/// sqlx异常转换为EdgeError
/// into an `EdgeError`.
impl From<sqlx::error::Error> for EdgeError {
    fn from(inner: sqlx::error::Error) -> Self {
        EdgeError::SqlxError(inner)
    }
}

impl<T> From<PoisonError<T>> for EdgeError {
    fn from(value: PoisonError<T>) -> Self {
        EdgeError::Message(value.to_string())
    }
}

impl From<libloading::Error> for EdgeError {
    fn from(value: libloading::Error) -> Self {
        EdgeError::Message(value.to_string())
    }
}

/// String异常转换为EdgeError
/// into an `EdgeError`.
impl From<String> for EdgeError {
    fn from(inner: String) -> Self {
        EdgeError::Message(inner)
    }
}


/// ValidationErrors异常转换为EdgeError
/// into an `EdgeError`.
impl From<ValidationErrors> for EdgeError {
    fn from(inner: ValidationErrors) -> Self {
        EdgeError::ValidationErrors(inner)
    }
}

/// 异常统一转换为response
impl IntoResponse for EdgeError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            EdgeError::SqlxError(sqlx::error::Error::Io(err)) => {
                tracing::error!("数据库处理异常:{:}",&err);
                (StatusCode::BAD_REQUEST, R::<String>::fail(err.to_string()))
            }
            EdgeError::SqlxError(err) => {
                tracing::error!("数据库未知异常:{:}",&err);
                (StatusCode::INTERNAL_SERVER_ERROR, R::fail(format!("数据库异常:{}", err.to_string())))
            }

            EdgeError::Message(err) => {
                tracing::error!("数据错误:{:}",&err);
                (StatusCode::BAD_REQUEST, R::bad_request(format!("{}", err)))
            }

            EdgeError::ValidationErrors(validation_errors) => {
                (StatusCode::BAD_REQUEST, R::bad_request(validation_errors.to_string()))
            }
            EdgeError::AuthError(auth) => {
                match auth {
                    AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, R::fail_with_code(401,"Wrong credentials".to_string())),
                    AuthError::MissingCredentials => (StatusCode::FORBIDDEN, R::fail_with_code(403,"Missing credentials".to_string())),
                    AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, R::fail_with_code(500,"Token creation error".to_string())),
                    AuthError::InvalidToken => (StatusCode::BAD_REQUEST, R::fail_with_code(400,"Invalid token".to_string())),
                }
            }
        };
        let body = Json(error_message);

        (status, body).into_response()
    }
}