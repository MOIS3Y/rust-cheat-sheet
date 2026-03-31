//! Common trait operations.
//!
//! Demonstrates usage of standard traits:
//! `Clone`, `Default`, `From`/`Into`, `TryFrom`/`TryInto`, `AsRef`/`AsMut`.

use std::convert::TryFrom;

/// Clone trait for explicit copying.
///
/// Derive `Clone` to enable explicit copying of values.
/// Use `.clone()` to create a deep copy.
pub fn clone_copy() {
    #[derive(Debug, Clone)]
    struct MyStruct {
        data: String,
    }

    let original = MyStruct {
        data: String::from("hello"),
    };
    let cloned = original.clone();
    assert_eq!(cloned.data, "hello");
}

/// Default trait for default values.
///
/// Derive `Default` to get a default instance.
/// Useful for configuration and builders.
pub fn default_value() {
    #[derive(Debug, Default)]
    struct Config {
        timeout: u32,
        retries: u32,
    }

    let default_config = Config::default();
    assert_eq!(default_config.timeout, 0);
    assert_eq!(default_config.retries, 0);
}

/// From and Into for infallible conversions.
///
/// `From<T>` is implemented to convert from `T`.
/// `Into<U>` is automatically implemented when `From` exists.
pub fn from_into() {
    let _str_val = String::from("hello");
    let _from_str: String = "hello".to_string();

    let num: i32 = 42;
    let _float: f64 = num.into();
}

/// TryFrom and TryInto for fallible conversions.
///
/// Use when conversion might fail.
/// Returns `Result<T, Error>`.
pub fn try_from_into() {
    let big_num = 1000;
    let result = u8::try_from(big_num);
    assert!(result.is_err());

    let small_num = 100;
    let result = u8::try_from(small_num);
    assert_eq!(result, Ok(100));
}

/// AsRef for cheap reference conversions.
///
/// Converts to a reference type cheaply.
/// Useful for generic functions accepting multiple string types.
pub fn as_ref_generic() {
    fn takes_string_ref<T: AsRef<str>>(s: &T) -> &str {
        s.as_ref()
    }

    assert_eq!(takes_string_ref(&"hello"), "hello");
    assert_eq!(
        takes_string_ref(&String::from("hello")),
        "hello"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_copy() {
        clone_copy();
    }

    #[test]
    fn test_default_value() {
        default_value();
    }

    #[test]
    fn test_from_into() {
        from_into();
    }

    #[test]
    fn test_try_from_into() {
        try_from_into();
    }

    #[test]
    fn test_as_ref_generic() {
        as_ref_generic();
    }
}
