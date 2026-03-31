//! Type-erased errors with `Box<dyn Error>`.
//!
//! Demonstrates how to handle multiple error types
//! without defining a custom enum.

/// Return any error with `Box<dyn Error>`.
///
/// Useful for quick prototyping or simple tools.
pub fn box_dyn_error_basic() {
    use std::error::Error;
    use std::fs;

    fn read_config() -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string("config.txt")?;
        Ok(content)
    }

    // Function signature accepts any error type
    let result: Result<String, Box<dyn Error>> = read_config();
    // Will fail (file doesn't exist), but that's ok for demo
    assert!(result.is_err());
}

/// Multiple error sources without enum.
///
/// Different errors unified by trait object.
pub fn box_dyn_error_multiple() {
    use std::error::Error;
    use std::fs;

    fn process_file(path: &str) -> Result<i32, Box<dyn Error>> {
        let content = fs::read_to_string(path)?; // io::Error
        let number: i32 = content.trim().parse()?; // ParseIntError
        Ok(number)
    }

    let result: Result<i32, Box<dyn Error>> = process_file("test.txt");
    assert!(result.is_err());
}

/// `Box<dyn Error>` with `source()`.
///
/// Access underlying error for debugging.
pub fn box_dyn_error_source() {
    use std::error::Error;
    use std::fs;

    fn read_and_parse() -> Result<i32, Box<dyn Error>> {
        let content = fs::read_to_string("data.txt")?;
        let number: i32 = content.trim().parse()?;
        Ok(number)
    }

    let result: Result<i32, Box<dyn Error>> = read_and_parse();
    if let Err(e) = result {
        // Can access source error chain
        let _source = e.source();
    }
}

/// `Box<dyn Error + Send + Sync>` for threads.
///
/// Thread-safe error trait object.
pub fn box_dyn_error_send_sync() {
    use std::error::Error;

    fn thread_safe_fn() -> Result<(), Box<dyn Error + Send + Sync>> {
        // Accepts errors that can cross thread boundaries
        let result: Result<i32, std::num::ParseIntError> = "42".parse();
        let _num = result?;
        Ok(())
    }

    let result: Result<(), Box<dyn Error + Send + Sync>> = thread_safe_fn();
    assert!(result.is_ok());
}

/// When to use `Box<dyn Error>`.
///
/// Trade-offs vs custom error types.
pub fn box_dyn_error_tradeoffs() {
    use std::error::Error;

    //  Good for:
    // - Quick prototypes
    // - Simple CLI tools
    // - Examples and tests

    //  Avoid for:
    // - Libraries (users can't match on errors)
    // - Production code needing specific handling
    // - When error context matters

    fn prototype() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    let result: Result<(), Box<dyn Error>> = prototype();
    assert!(result.is_ok());
}

/// Convert to `Box<dyn Error>` from specific error.
///
/// Specific errors auto-convert to trait object.
pub fn box_dyn_error_conversion() {
    use std::error::Error;
    use std::num::ParseIntError;

    fn specific_error() -> Result<i32, ParseIntError> {
        "42".parse()
    }

    fn generic_error() -> Result<i32, Box<dyn Error>> {
        Ok(specific_error()?) // Auto-converted
    }

    let result: Result<i32, Box<dyn Error>> = generic_error();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_dyn_error_basic() {
        box_dyn_error_basic();
    }

    #[test]
    fn test_box_dyn_error_multiple() {
        box_dyn_error_multiple();
    }

    #[test]
    fn test_box_dyn_error_source() {
        box_dyn_error_source();
    }

    #[test]
    fn test_box_dyn_error_send_sync() {
        box_dyn_error_send_sync();
    }

    #[test]
    fn test_box_dyn_error_tradeoffs() {
        box_dyn_error_tradeoffs();
    }

    #[test]
    fn test_box_dyn_error_conversion() {
        box_dyn_error_conversion();
    }
}
