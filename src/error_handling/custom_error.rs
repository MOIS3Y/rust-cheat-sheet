//! Custom error types with `thiserror`-like pattern.
//!
//! Demonstrates how to create ergonomic error types
//! without external dependencies.

/// Application error type.
///
/// Enum variants represent different error cases.
#[derive(Debug)]
pub enum AppError {
    /// User was not found.
    NotFound { user_id: u32 },
    /// Invalid input provided.
    InvalidInput { field: String, reason: String },
    /// External service failed.
    ServiceError { service: String, code: u32 },
}

/// Implement `Display` for user-friendly messages.
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound { user_id } => {
                write!(f, "User {user_id} not found")
            }
            AppError::InvalidInput { field, reason } => {
                write!(f, "Invalid {field}: {reason}")
            }
            AppError::ServiceError { service, code } => {
                write!(f, "Service {service} failed with code {code}")
            }
        }
    }
}

/// Implement `Error` trait for std compatibility.
impl std::error::Error for AppError {}

/// Create errors with context.
///
/// Variants can hold structured data.
pub fn custom_error_with_context() {
    fn find_user(user_id: u32) -> Result<(), AppError> {
        if user_id == 0 {
            return Err(AppError::NotFound { user_id });
        }
        Ok(())
    }

    let result = find_user(0);
    assert!(matches!(result, Err(AppError::NotFound { user_id: 0 })));
}

/// Multiple validation errors.
///
/// Collect context about what went wrong.
pub fn custom_error_validation() {
    fn validate_email(email: &str) -> Result<(), AppError> {
        if !email.contains('@') {
            return Err(AppError::InvalidInput {
                field: "email".to_string(),
                reason: "must contain @".to_string(),
            });
        }
        Ok(())
    }

    let result = validate_email("invalid");
    assert!(matches!(result, Err(AppError::InvalidInput { .. })));
}

/// Error from external service.
///
/// Wrap third-party errors with context.
pub fn custom_error_service() {
    fn call_api(status: u32) -> Result<(), AppError> {
        if status >= 400 {
            return Err(AppError::ServiceError {
                service: "payment".to_string(),
                code: status,
            });
        }
        Ok(())
    }

    let result = call_api(500);
    assert!(matches!(
        result,
        Err(AppError::ServiceError { service, code })
            if service == "payment" && code == 500
    ));
}

/// Display trait for logging.
///
/// Human-readable error messages.
pub fn custom_error_display() {
    let err = AppError::NotFound { user_id: 42 };
    let message = format!("{err}");
    assert_eq!(message, "User 42 not found");

    let err = AppError::InvalidInput {
        field: "age".to_string(),
        reason: "must be positive".to_string(),
    };
    let message = format!("{err}");
    assert_eq!(message, "Invalid age: must be positive");
}

/// Error trait for generic handling.
///
/// `Error` trait enables std library integration.
pub fn custom_error_trait() {
    fn handle_error(e: &dyn std::error::Error) -> String {
        e.to_string()
    }

    let err = AppError::ServiceError {
        service: "email".to_string(),
        code: 503,
    };
    let message = handle_error(&err);
    assert!(message.contains("email"));
}

/// Match on error variants.
///
/// Pattern matching for error-specific handling.
pub fn custom_error_matching() {
    fn handle_app_error(err: &AppError) -> &'static str {
        match err {
            AppError::NotFound { .. } => "return 404",
            AppError::InvalidInput { .. } => "return 400",
            AppError::ServiceError { .. } => "retry later",
        }
    }

    let err = AppError::NotFound { user_id: 1 };
    assert_eq!(handle_app_error(&err), "return 404");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_error_with_context() {
        custom_error_with_context();
    }

    #[test]
    fn test_custom_error_validation() {
        custom_error_validation();
    }

    #[test]
    fn test_custom_error_service() {
        custom_error_service();
    }

    #[test]
    fn test_custom_error_display() {
        custom_error_display();
    }

    #[test]
    fn test_custom_error_trait() {
        custom_error_trait();
    }

    #[test]
    fn test_custom_error_matching() {
        custom_error_matching();
    }
}
