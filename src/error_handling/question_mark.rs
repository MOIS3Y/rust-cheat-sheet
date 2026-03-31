//! The `?` operator for error propagation.
//!
//! Demonstrates how `?` simplifies error handling
//! by automatically propagating errors up the call stack.

/// Read config from environment variable.
///
/// `?` replaces explicit `match` on `Result`.
/// Returns `Err` if variable is not set.
pub fn question_mark_env() {
    fn get_home_dir() -> Result<String, std::env::VarError> {
        // Without `?` would need verbose match:
        // match std::env::var("HOME") {
        //     Ok(val) => Ok(val),
        //     Err(e) => Err(e),
        // }
        let home = std::env::var("HOME")?;
        Ok(home)
    }

    let result = get_home_dir();
    assert!(result.is_ok());
}

/// Parse number from string with `?`.
///
/// `?` propagates parse errors automatically.
pub fn question_mark_parse() {
    fn parse_age(input: &str) -> Result<u32, std::num::ParseIntError> {
        let age: u32 = input.parse()?;
        Ok(age)
    }

    let result = parse_age("25");
    assert_eq!(result, Ok(25));

    let invalid = parse_age("not_a_number");
    assert!(invalid.is_err());
}

/// Chain operations with `?`.
///
/// Multiple `?` in sequence for fallible operations.
pub fn question_mark_chain() {
    fn process_input(input: &str) -> Result<i32, std::num::ParseIntError> {
        let trimmed = input.trim();
        let number: i32 = trimmed.parse()?;
        Ok(number * 2)
    }

    let result = process_input("  42  ");
    assert_eq!(result, Ok(84));
}

/// `?` with Option.
///
/// `?` works on `Option` too, returns `None` early.
pub fn question_mark_option() {
    fn get_first_char(s: &str) -> Option<char> {
        let first = s.chars().next()?;
        Some(first)
    }

    assert_eq!(get_first_char("hello"), Some('h'));
    assert_eq!(get_first_char(""), None);
}

/// `?` in main function.
///
/// `main` can return `Result` to exit with error code.
pub fn question_mark_in_main() {
    fn run() -> Result<(), std::env::VarError> {
        let _home = std::env::var("HOME")?;
        Ok(())
    }

    let result = run();
    assert!(result.is_ok());
}

/// Convert error types with `map_err`.
///
/// `map_err` transforms error before `?` propagates it.
pub fn question_mark_map_err() {
    fn read_config() -> Result<String, String> {
        std::env::var("HOME").map_err(|e| format!("env error: {e}"))
    }

    let result = read_config();
    assert!(result.is_ok());
}

/// Early return with `?` vs verbose match.
///
/// Shows how `?` reduces boilerplate.
pub fn question_mark_vs_match() {
    // With `?` — clean and simple
    fn with_question_mark(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.parse()?;
        Ok(n * 2)
    }

    // Without `?` — verbose
    fn without_question_mark(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n = match s.parse::<i32>() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        Ok(n * 2)
    }

    assert_eq!(with_question_mark("5"), Ok(10));
    assert_eq!(without_question_mark("5"), Ok(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_mark_env() {
        question_mark_env();
    }

    #[test]
    fn test_question_mark_parse() {
        question_mark_parse();
    }

    #[test]
    fn test_question_mark_chain() {
        question_mark_chain();
    }

    #[test]
    fn test_question_mark_option() {
        question_mark_option();
    }

    #[test]
    fn test_question_mark_in_main() {
        question_mark_in_main();
    }

    #[test]
    fn test_question_mark_map_err() {
        question_mark_map_err();
    }

    #[test]
    fn test_question_mark_vs_match() {
        question_mark_vs_match();
    }
}
