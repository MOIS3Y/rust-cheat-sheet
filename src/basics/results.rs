//! Result<T, E> operations.
//!
//! Demonstrates common methods for working with `Result<T, E>`,
//! the type for error handling in Rust.

#![allow(clippy::all)]

/// Unwrap Ok value or panic.
///
/// Returns the value inside `Ok`, or panics on `Err`.
/// Use `expect()` for better error messages.
pub fn unwrap_ok() {
    let ok_val: Result<i32, &str> = Ok(42);
    assert_eq!(ok_val.unwrap(), 42);
}

/// Check if Result is Ok or Err.
///
/// Returns `true` if the result is `Ok`, `false` if `Err`.
pub fn is_ok_check() {
    let ok_val: Result<i32, &str> = Ok(42);
    let err_val: Result<i32, &str> = Err("error");

    assert!(ok_val.is_ok());
    assert!(err_val.is_err());
}

/// Transform Ok value with map.
///
/// Applies a function to the contained `Ok` value,
/// leaves `Err` unchanged.
pub fn map_ok() {
    let ok_val: Result<i32, &str> = Ok(42);
    let mapped = ok_val.map(|x| x * 2);
    assert_eq!(mapped, Ok(84));
}

/// Transform Err value with map_err.
///
/// Applies a function to the contained `Err` value,
/// leaves `Ok` unchanged.
pub fn map_err() {
    let err_val: Result<i32, &str> = Err("error");
    let mapped_err =
        err_val.map_err(|e| format!("Converted: {}", e));
    assert_eq!(mapped_err, Err("Converted: error".to_string()));
}

/// Chain fallible operations with and_then.
///
/// Like `map`, but the function returns a `Result`.
/// Useful for chaining operations that can fail.
pub fn and_then_chain() {
    let ok_val: Result<i32, &str> = Ok(42);

    let chained = ok_val.and_then(|x| {
        if x > 10 {
            Ok(x * 2)
        } else {
            Err("too small")
        }
    });
    assert_eq!(chained, Ok(84));
}

/// Provide fallback Result with or.
///
/// Returns `self` if `Ok`, otherwise returns the fallback.
pub fn or_fallback() {
    let err_val: Result<i32, &str> = Err("error");
    let fallback: Result<i32, &str> = err_val.or(Ok(100));
    assert_eq!(fallback, Ok(100));
}

/// Convert Result to Option with ok.
///
/// Discards the error, returning `Some` for `Ok`, `None` for `Err`.
pub fn ok_to_option() {
    let ok_val: Result<i32, &str> = Ok(42);
    let err_val: Result<i32, &str> = Err("error");

    assert_eq!(ok_val.ok(), Some(42));
    assert_eq!(err_val.ok(), None);
}

/// Extract error with err.
///
/// Returns `Some` for `Err`, `None` for `Ok`.
pub fn err_extract() {
    let ok_val: Result<i32, &str> = Ok(42);
    let err_val: Result<i32, &str> = Err("error");

    assert_eq!(ok_val.err(), None);
    assert_eq!(err_val.err(), Some("error"));
}

/// Work with references using as_ref.
///
/// Converts `Result<T, E>` to `Result<&T, &E>` for non-consuming access.
pub fn as_ref_access() {
    let ok_val: Result<i32, &str> = Ok(42);
    let ref_val = ok_val.as_ref().map(|x| x + 1);
    assert_eq!(ref_val, Ok(43));
    // ok_val still available
}

/// Convert `Option<Result>` to `Result<Option>` with transpose.
///
/// Swaps the positions of `Option` and `Result`.
pub fn transpose_swap() {
    let opt_res: Option<Result<i32, &str>> = Some(Ok(42));
    assert_eq!(opt_res.transpose(), Ok(Some(42)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_ok() {
        unwrap_ok();
    }

    #[test]
    fn test_is_ok_check() {
        is_ok_check();
    }

    #[test]
    fn test_map_ok() {
        map_ok();
    }

    #[test]
    fn test_map_err() {
        map_err();
    }

    #[test]
    fn test_and_then_chain() {
        and_then_chain();
    }

    #[test]
    fn test_or_fallback() {
        or_fallback();
    }

    #[test]
    fn test_ok_to_option() {
        ok_to_option();
    }

    #[test]
    fn test_err_extract() {
        err_extract();
    }

    #[test]
    fn test_as_ref_access() {
        as_ref_access();
    }

    #[test]
    fn test_transpose_swap() {
        transpose_swap();
    }
}
