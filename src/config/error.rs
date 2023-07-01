use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::{ValidationErrors};

pub type Result<T> = std::result::Result<T, EdgeError>;

/// 平台统一异常
pub enum EdgeError {
    /// sqlx异常
    SqlxError(sqlx::error::Error),
    ValidationErrors(ValidationErrors),
    Message(String),
}

/// sqlx异常转换为EdgeError
/// into an `EdgeError`.
impl From<sqlx::error::Error> for EdgeError {
    fn from(inner: sqlx::error::Error) -> Self {
        EdgeError::SqlxError(inner)
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
                (StatusCode::BAD_REQUEST, err.to_string())
            }
            EdgeError::SqlxError(err) => {
                tracing::error!("数据库未知异常:{:}",&err);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库异常:{}", err.to_string()))
            }

            EdgeError::Message(err) => {
                tracing::error!("数据错误:{:}",&err);
                (StatusCode::BAD_REQUEST, format!("数据错误:{}", err))
            }

            EdgeError::ValidationErrors(validation_errors) => {
                (StatusCode::BAD_REQUEST, validation_errors.to_string())
            }
        };
        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}