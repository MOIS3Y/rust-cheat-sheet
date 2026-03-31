//! Documentation examples.
//!
//! Demonstrates idiomatic Rust documentation patterns
//! following the Rust API Guidelines.

// Allow dead_code and clippy warnings in this module - 
// types and examples are for documentation demonstration only.
#![allow(dead_code)]
#![allow(clippy::unnecessary_literal_unwrap)]

/// Unwrap an Option or panic.
///
/// Returns the value inside `Some`.
///
/// # Panics
///
/// Panics if the option is `None` with a default message.
///
/// # Example
///
/// ```
/// let some_val = Some(42);
/// assert_eq!(some_val.unwrap(), 42);
/// ```
///
/// # See Also
///
/// - [`Option::expect`] for a custom panic message
/// - [`Option::is_some`] to check before unwrapping
pub fn doc_unwrap_with_panics() {
    let some_val = Some(42);
    assert_eq!(some_val.unwrap(), 42);
}

/// Parse a string as an integer.
///
/// Attempts to parse the string as a signed integer.
/// Leading and trailing whitespace is ignored.
///
/// # Errors
///
/// Returns [`ParseIntError`] if the string contains
/// non-numeric characters or the value is out of range.
///
/// # Example
///
/// ```
/// let result = "42".parse::<i32>();
/// assert_eq!(result, Ok(42));
///
/// let err = "not a number".parse::<i32>();
/// assert!(err.is_err());
/// ```
///
/// [`ParseIntError`]: std::num::ParseIntError
pub fn doc_with_errors() {
    let result = "42".parse::<i32>();
    assert_eq!(result, Ok(42));
}

/// Divide two numbers safely.
///
/// Returns the result of dividing `a` by `b`,
/// or `None` if `b` is zero.
///
/// # Arguments
///
/// * `a` - The dividend
/// * `b` - The divisor
///
/// # Returns
///
/// Returns `Some(a / b)` if `b != 0`, `None` otherwise.
///
/// # Example
///
/// ```
/// use rust_cheat_sheet::basics::documentation::safe_divide;
///
/// assert_eq!(safe_divide(10, 2), Some(5));
/// assert_eq!(safe_divide(10, 0), None);
/// ```
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Configuration for a network client.
///
/// Contains settings for establishing connections.
///
/// # Fields
///
/// * `timeout` - Connection timeout in seconds
/// * `retries` - Number of retry attempts
/// * `host` - Target hostname
///
/// # Example
///
/// ```
/// use rust_cheat_sheet::basics::documentation::ClientConfig;
///
/// let config = ClientConfig {
///     timeout: 30,
///     retries: 3,
///     host: String::from("localhost"),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Connection timeout in seconds.
    pub timeout: u32,
    /// Number of retry attempts on failure.
    pub retries: u32,
    /// Target hostname or IP address.
    pub host: String,
}

impl Default for ClientConfig {
    /// Creates default configuration.
    ///
    /// # Defaults
    ///
    /// - `timeout`: 30 seconds
    /// - `retries`: 3 attempts
    /// - `host`: "localhost"
    fn default() -> Self {
        Self {
            timeout: 30,
            retries: 3,
            host: String::from("localhost"),
        }
    }
}

/// Result type for I/O operations.
///
/// Convenience type alias for operations that
/// return a value or an I/O error.
///
/// # Type Parameters
///
/// * `T` - The success type
///
/// # Example
///
/// ```
/// type IoResult<T> = Result<T, std::io::Error>;
///
/// fn read_data() -> IoResult<String> {
///     Ok(String::from("data"))
/// }
/// ```
pub type IoResult<T> = Result<T, std::io::Error>;

/// Trait for types that can be validated.
///
/// Types implementing this trait can check their
/// internal state for correctness.
///
/// # Implementors
///
/// - [`ClientConfig`]
/// - Custom configuration types
///
/// # Example
///
/// ```
/// pub trait Validate {
///     fn is_valid(&self) -> bool;
/// }
/// ```
pub trait Validate {
    /// Returns `true` if the instance is valid.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_cheat_sheet::basics::documentation::{Validate, ClientConfig};
    ///
    /// let config = ClientConfig::default();
    /// assert!(config.is_valid());
    /// ```
    fn is_valid(&self) -> bool;
}

impl Validate for ClientConfig {
    fn is_valid(&self) -> bool {
        self.timeout > 0 && self.retries > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_unwrap_with_panics() {
        doc_unwrap_with_panics();
    }

    #[test]
    fn test_doc_with_errors() {
        doc_with_errors();
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.timeout, 30);
        assert_eq!(config.retries, 3);
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_validate() {
        let config = ClientConfig::default();
        assert!(config.is_valid());
    }
}
