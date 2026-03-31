//! Converting errors with `From` trait.
//!
//! Demonstrates how implementing `From` allows
//! automatic error conversion with `?` operator.

/// Custom error type for user operations.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum UserError {
    NotFound,
    InvalidAge,
    Database(String),
}

/// Implement `From` for standard library error.
impl From<std::num::ParseIntError> for UserError {
    fn from(_err: std::num::ParseIntError) -> Self {
        UserError::InvalidAge
    }
}

/// Parse age with automatic error conversion.
///
/// `From` trait allows `?` to convert errors automatically.
pub fn from_parse_int() {
    fn parse_user_age(input: &str) -> Result<u32, UserError> {
        // ParseIntError automatically converted to UserError
        let age: u32 = input.parse()?;
        Ok(age)
    }

    let result = parse_user_age("25");
    assert_eq!(result, Ok(25));

    let invalid = parse_user_age("not_a_number");
    assert!(matches!(invalid, Err(UserError::InvalidAge)));
}

/// Multiple error sources with `From`.
///
/// Different error types unified into one.
pub fn from_multiple_sources() {
    #[derive(Debug, PartialEq)]
    enum AppError {
        IoError,
        ParseError,
    }

    impl From<std::io::Error> for AppError {
        fn from(_err: std::io::Error) -> Self {
            AppError::IoError
        }
    }

    impl From<std::num::ParseIntError> for AppError {
        fn from(_err: std::num::ParseIntError) -> Self {
            AppError::ParseError
        }
    }

    fn read_and_parse() -> Result<i32, AppError> {
        // Could return IoError or ParseError
        let content = "42";
        let number: i32 = content.parse()?;
        Ok(number)
    }

    let result = read_and_parse();
    assert_eq!(result, Ok(42));
}

/// `From` for custom error conversion.
///
/// Convert between custom error types.
pub fn from_custom_conversion() {
    #[derive(Debug, PartialEq)]
    struct ConfigError {
        message: String,
    }

    #[derive(Debug, PartialEq)]
    struct DatabaseError {
        message: String,
    }

    impl From<DatabaseError> for ConfigError {
        fn from(err: DatabaseError) -> Self {
            ConfigError {
                message: format!("database failed: {}", err.message),
            }
        }
    }

    fn load_config() -> Result<(), ConfigError> {
        let db_result: Result<(), DatabaseError> = Err(DatabaseError {
            message: String::from("connection timeout"),
        });
        db_result?; // Automatically converted
        Ok(())
    }

    let result = load_config();
    assert!(matches!(
        result,
        Err(ConfigError { message }) if message.contains("database failed")
    ));
}

/// `From` never fails.
///
/// `From::from` must be infallible — no `Result` return.
#[allow(dead_code)]
pub fn from_infallible() {
    #[derive(Debug)]
    enum Error {
        Wrapper(String),
    }

    impl From<&str> for Error {
        fn from(msg: &str) -> Self {
            Error::Wrapper(msg.to_string())
        }
    }

    fn validate_age(age: u32) -> Result<(), Error> {
        if age == 0 {
            return Err("age cannot be zero".into());
        }
        Ok(())
    }

    let result = validate_age(0);
    assert!(result.is_err());
}

/// `From` vs `map_err`.
///
/// `From` is cleaner when conversion is straightforward.
pub fn from_vs_map_err() {
    #[derive(Debug, PartialEq)]
    enum Error {
        Custom(String),
    }

    impl From<std::num::ParseIntError> for Error {
        fn from(_err: std::num::ParseIntError) -> Self {
            Error::Custom("parse failed".to_string())
        }
    }

    // With `From` — automatic
    fn with_from(s: &str) -> Result<i32, Error> {
        let n: i32 = s.parse()?;
        Ok(n)
    }

    // Without `From` — manual `map_err`
    fn without_from(s: &str) -> Result<i32, Error> {
        let n: i32 = s.parse().map_err(|_| Error::Custom("parse failed".to_string()))?;
        Ok(n)
    }

    assert_eq!(with_from("42"), Ok(42));
    assert_eq!(without_from("42"), Ok(42));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_parse_int() {
        from_parse_int();
    }

    #[test]
    fn test_from_multiple_sources() {
        from_multiple_sources();
    }

    #[test]
    fn test_from_custom_conversion() {
        from_custom_conversion();
    }

    #[test]
    fn test_from_infallible() {
        from_infallible();
    }

    #[test]
    fn test_from_vs_map_err() {
        from_vs_map_err();
    }
}
