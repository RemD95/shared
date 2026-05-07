use crate::ErrorResponse;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ApiError {
    InvalidUuid(String),
    ItemNotFound,
    NameCannotBeEmpty,
    MutexPoisoned,
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::InvalidUuid(details) => write!(f, "invalid uuid: {}", details),
            ApiError::ItemNotFound => write!(f, "item not found"),
            ApiError::NameCannotBeEmpty => write!(f, "name cannot be empty"),
            ApiError::MutexPoisoned => write!(f, "internal server error"),
            ApiError::InternalError(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    pub fn to_response(&self) -> (u16, ErrorResponse) {
        match self {
            ApiError::InvalidUuid(_) => (
                400,
                ErrorResponse {
                    message: self.to_string(),
                    code: Some("INVALID_UUID".to_string()),
                },
            ),
            ApiError::ItemNotFound => (
                404,
                ErrorResponse {
                    message: self.to_string(),
                    code: Some("NOT_FOUND".to_string()),
                },
            ),
            ApiError::NameCannotBeEmpty => (
                400,
                ErrorResponse {
                    message: self.to_string(),
                    code: Some("VALIDATION_ERROR".to_string()),
                },
            ),
            ApiError::MutexPoisoned | ApiError::InternalError(_) => (
                500,
                ErrorResponse {
                    message: "internal server error".to_string(),
                    code: Some("INTERNAL_ERROR".to_string()),
                },
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_invalid_uuid() {
        let err = ApiError::InvalidUuid("not-a-uuid".to_string());
        let (status, resp) = err.to_response();
        assert_eq!(status, 400);
        assert_eq!(resp.code, Some("INVALID_UUID".to_string()));
    }

    #[test]
    fn test_error_response_not_found() {
        let err = ApiError::ItemNotFound;
        let (status, resp) = err.to_response();
        assert_eq!(status, 404);
        assert_eq!(resp.code, Some("NOT_FOUND".to_string()));
    }

    #[test]
    fn test_error_response_empty_name() {
        let err = ApiError::NameCannotBeEmpty;
        let (status, resp) = err.to_response();
        assert_eq!(status, 400);
        assert_eq!(resp.code, Some("VALIDATION_ERROR".to_string()));
    }

    #[test]
    fn test_error_response_mutex_poisoned() {
        let err = ApiError::MutexPoisoned;
        let (status, _resp) = err.to_response();
        assert_eq!(status, 500);
    }
}
