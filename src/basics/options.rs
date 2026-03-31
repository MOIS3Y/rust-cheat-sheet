//! `Option<T>` operations.
//!
//! Demonstrates common methods for working with `Option<T>`,
//! the type for optional values in Rust.

#![allow(clippy::all)]

/// Unwrap an Option or panic.
///
/// Returns the value inside `Some`, or panics on `None`.
/// Use `expect()` for better error messages.
pub fn unwrap_some() {
    let some_val = Some(42);
    assert_eq!(some_val.unwrap(), 42);
}

/// Check if Option is Some or None.
///
/// Returns `true` if the option is `Some`, `false` if `None`.
pub fn is_some_check() {
    let some_val = Some(42);
    let none_val: Option<i32> = None;

    assert!(some_val.is_some());
    assert!(none_val.is_none());
}

/// Transform value inside Option with map.
///
/// Applies a function to the contained value if `Some`,
/// returns `None` otherwise.
pub fn map_transform() {
    let some_val = Some(42);
    let none_val: Option<i32> = None;

    let mapped = some_val.map(|x| x * 2);
    assert_eq!(mapped, Some(84));

    let mapped_none = none_val.map(|x| x * 2);
    assert_eq!(mapped_none, None);
}

/// Chain operations returning Option with and_then.
///
/// Like `map`, but the function must return an `Option`.
/// Useful for chaining fallible operations.
pub fn and_then_chain() {
    let some_val = Some(42);

    let result =
        some_val.and_then(|x| if x > 10 { Some(x * 2) } else { None });
    assert_eq!(result, Some(84));
}

/// Provide default value with unwrap_or.
///
/// Returns the contained value or a default.
/// Use `unwrap_or_else` for lazy evaluation.
pub fn unwrap_or_default() {
    let none_val: Option<i32> = None;
    assert_eq!(none_val.unwrap_or(100), 100);
}

/// Convert Option to Result with ok_or.
///
/// Transforms `Some(v)` to `Ok(v)`, `None` to `Err(err)`.
pub fn ok_or_conversion() {
    let some_val = Some(42);
    let none_val: Option<i32> = None;

    let res: Result<i32, &str> = some_val.ok_or("error");
    assert_eq!(res, Ok(42));

    let res_err: Result<i32, &str> = none_val.ok_or("error");
    assert_eq!(res_err, Err("error"));
}

/// Filter value by condition.
///
/// Returns `Some` if predicate is true, `None` otherwise.
pub fn filter_by_condition() {
    let some_val = Some(42);

    let filtered = some_val.filter(|x| *x > 50);
    assert_eq!(filtered, None);

    let filtered = some_val.filter(|x| *x < 50);
    assert_eq!(filtered, Some(42));
}

/// Take value leaving None.
///
/// Replaces the value with `None` and returns the old value.
pub fn take_value() {
    let mut opt = Some(42);
    let taken = opt.take();
    assert_eq!(taken, Some(42));
    assert_eq!(opt, None);
}

/// Replace value inside Option.
///
/// Replaces the value and returns the old one.
pub fn replace_value() {
    let mut opt = Some(42);
    let old = opt.replace(100);
    assert_eq!(old, Some(42));
    assert_eq!(opt, Some(100));
}

/// Combine two Options with zip.
///
/// Returns `Some((a, b))` if both are `Some`, `None` otherwise.
pub fn zip_combine() {
    let a = Some(1);
    let b = Some(2);
    assert_eq!(a.zip(b), Some((1, 2)));
    assert_eq!(a.zip(None::<i32>), None);
}

/// Flatten nested Option.
///
/// Removes one level of `Option` nesting.
pub fn flatten_nested() {
    let nested: Option<Option<i32>> = Some(Some(42));
    assert_eq!(nested.flatten(), Some(42));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_some() {
        unwrap_some();
    }

    #[test]
    fn test_is_some_check() {
        is_some_check();
    }

    #[test]
    fn test_map_transform() {
        map_transform();
    }

    #[test]
    fn test_and_then_chain() {
        and_then_chain();
    }

    #[test]
    fn test_unwrap_or_default() {
        unwrap_or_default();
    }

    #[test]
    fn test_ok_or_conversion() {
        ok_or_conversion();
    }

    #[test]
    fn test_filter_by_condition() {
        filter_by_condition();
    }

    #[test]
    fn test_take_value() {
        take_value();
    }

    #[test]
    fn test_replace_value() {
        replace_value();
    }

    #[test]
    fn test_zip_combine() {
        zip_combine();
    }

    #[test]
    fn test_flatten_nested() {
        flatten_nested();
    }
}
