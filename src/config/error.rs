use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;

pub type Result<T> = std::result::Result<T, EdgeError>;

/// 平台统一异常
pub enum EdgeError {
    /// sqlx异常
    SqlxError(sqlx::error::Error),
}

/// sqlx异常转换为EdgeError
/// into an `EdgeError`.
impl From<sqlx::error::Error> for EdgeError {
    fn from(inner: sqlx::error::Error) -> Self {
        EdgeError::SqlxError(inner)
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
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}