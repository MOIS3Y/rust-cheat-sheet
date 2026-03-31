//! String and str operations.
//!
//! Demonstrates common methods for working with `String` and `&str`,
//! the two string types in Rust.

/// Push character to String.
///
/// Adds a single character to the end of a mutable String.
/// Use `push_str` for appending string slices.
pub fn push_char() {
    let mut mutable = String::from("Hello");
    mutable.push('!');
    assert_eq!(mutable, "Hello!");
}

/// Push string to String.
///
/// Appends a string slice to the end of a mutable String.
pub fn push_str() {
    let mut mutable = String::from("Hello");
    mutable.push_str(" World");
    assert_eq!(mutable, "Hello World");
}

/// Get string length in bytes.
///
/// Returns the length of the string in bytes, not characters.
/// For Unicode strings, this may differ from character count.
pub fn len_bytes() {
    let s = String::from("Hello, Rust!");
    assert_eq!(s.len(), 12);
}

/// Check if string is empty.
///
/// Returns `true` if the string contains no characters.
pub fn is_empty_check() {
    let s = String::from("Hello, Rust!");
    assert!(!s.is_empty());
}

/// Check if string contains substring.
///
/// Returns `true` if the string contains the specified pattern.
pub fn contains_substring() {
    let s = String::from("Hello, Rust!");
    assert!(s.contains("Rust"));
}

/// Check string prefix and suffix.
///
/// `starts_with` and `ends_with` check for patterns
/// at the beginning and end of the string.
pub fn starts_ends_with() {
    let s = String::from("Hello, Rust!");
    assert!(s.starts_with("Hello"));
    assert!(s.ends_with("Rust!"));
}

/// Split string by delimiter.
///
/// Returns an iterator over substrings separated by the delimiter.
pub fn split_delimiter() {
    let s = String::from("Hello, Rust!");
    let parts: Vec<&str> = s.split(',').collect();
    assert_eq!(parts, vec!["Hello", " Rust!"]);
}

/// Split by whitespace.
///
/// Returns an iterator over non-whitespace substrings.
pub fn split_whitespace() {
    let whitespace: Vec<&str> =
        "a b c".split_whitespace().collect();
    assert_eq!(whitespace, vec!["a", "b", "c"]);
}

/// Trim whitespace from string.
///
/// Removes leading and trailing whitespace.
pub fn trim_whitespace() {
    assert_eq!("  hello  ".trim(), "hello");
}

/// Iterate over Unicode characters.
///
/// `chars()` returns an iterator over actual Unicode
/// scalar values, not bytes.
pub fn chars_unicode() {
    let unicode = "Привет";
    let chars: Vec<char> = unicode.chars().collect();
    assert_eq!(chars, vec!['П', 'р', 'и', 'в', 'е', 'т']);
    assert_eq!(unicode.len(), 12);
}

/// Convert string to bytes.
///
/// Returns a byte slice of the string contents.
pub fn as_bytes() {
    let s = String::from("Hello, Rust!");
    let bytes = s.as_bytes();
    assert_eq!(bytes[0], b'H');
}

/// Replace substring in string.
///
/// Returns a new String with all occurrences replaced.
pub fn replace_substring() {
    let s = String::from("Hello, Rust!");
    let replaced = s.replace("Rust", "Go");
    assert_eq!(replaced, "Hello, Go!");
}

/// Convert case of string.
///
/// `to_lowercase` and `to_uppercase` return new Strings
/// with converted case.
pub fn convert_case() {
    assert_eq!("Hello".to_lowercase(), "hello");
    assert_eq!("Hello".to_uppercase(), "HELLO");
}

/// Find substring position.
///
/// Returns the byte index of the first occurrence, or `None`.
pub fn find_substring() {
    let s = String::from("Hello, Rust!");
    assert_eq!(s.find("Rust"), Some(7));
}

/// Safe string slicing with get.
///
/// Returns `Option<&str>` - `None` for invalid byte ranges.
/// Important: uses byte indices, not character indices!
pub fn get_slice() {
    let s = String::from("Hello, Rust!");
    assert_eq!(s.get(0..5), Some("Hello"));
    assert_eq!(s.get(0..100), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_char() {
        push_char();
    }

    #[test]
    fn test_push_str() {
        push_str();
    }

    #[test]
    fn test_len_bytes() {
        len_bytes();
    }

    #[test]
    fn test_is_empty_check() {
        is_empty_check();
    }

    #[test]
    fn test_contains_substring() {
        contains_substring();
    }

    #[test]
    fn test_starts_ends_with() {
        starts_ends_with();
    }

    #[test]
    fn test_split_delimiter() {
        split_delimiter();
    }

    #[test]
    fn test_split_whitespace() {
        split_whitespace();
    }

    #[test]
    fn test_trim_whitespace() {
        trim_whitespace();
    }

    #[test]
    fn test_chars_unicode() {
        chars_unicode();
    }

    #[test]
    fn test_as_bytes() {
        as_bytes();
    }

    #[test]
    fn test_replace_substring() {
        replace_substring();
    }

    #[test]
    fn test_convert_case() {
        convert_case();
    }

    #[test]
    fn test_find_substring() {
        find_substring();
    }

    #[test]
    fn test_get_slice() {
        get_slice();
    }
}
