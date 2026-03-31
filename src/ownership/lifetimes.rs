//! Lifetime annotations.
//!
//! Demonstrates basic lifetime concepts in Rust.

#![allow(clippy::all)]
#![allow(dead_code)]

/// Lifetime elision.
///
/// Compiler infers lifetimes in simple cases.
/// This function compiles without explicit lifetime annotations.
pub fn lifetime_elision() {
    let s = String::from("hello");
    let result = first_word(&s);
    assert_eq!(result, "hello");
}

fn first_word(s: &str) -> &str {
    s
}

/// Explicit lifetime annotation.
///
/// When multiple references exist, lifetimes clarify
/// which output reference is valid as long as which input.
pub fn explicit_lifetime() {
    let s1 = String::from("longer");
    let s2 = String::from("short");
    let result = longest(&s1, &s2);
    assert_eq!(result, "longer");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Lifetime with struct.
///
/// Structs holding references need lifetime annotations.
pub fn lifetime_with_struct() {
    let s = String::from("hello");
    let holder = Holder { value: &s };
    assert_eq!(holder.value, "hello");
}

struct Holder<'a> {
    value: &'a str,
}

/// Lifetime with impl.
///
/// Methods on structs with lifetimes need annotations.
pub fn lifetime_with_impl() {
    let s = String::from("hello");
    let holder = Holder { value: &s };
    let retrieved = holder.get_value();
    assert_eq!(retrieved, "hello");
}

impl<'a> Holder<'a> {
    fn get_value(&self) -> &'a str {
        self.value
    }

    fn set_value(&mut self, new_value: &'a str) {
        self.value = new_value;
    }
}

/// Multiple lifetimes.
///
/// Different lifetimes for different references.
pub fn multiple_lifetimes() {
    let s1 = String::from("first");
    let s2 = String::from("second");

    let result = select_first(&s1, &s2);
    assert_eq!(result, "first");
}

fn select_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

/// Static lifetime.
///
/// 'static means the reference lives for the entire program.
pub fn static_lifetime() {
    let s: &'static str = "static string";
    assert_eq!(s, "static string");
}

/// Lifetime bounds on generics.
///
/// Require that a generic type contains references
/// with a specific lifetime.
pub fn lifetime_bounds() {
    let s = String::from("hello");
    let result = print_and_return(&s);
    assert_eq!(result, "hello");
}

fn print_and_return<'a, T>(x: &'a T) -> &'a T
where
    T: std::fmt::Display + 'a,
{
    let _ = format!("{}", x);
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_elision() {
        lifetime_elision();
    }

    #[test]
    fn test_explicit_lifetime() {
        explicit_lifetime();
    }

    #[test]
    fn test_lifetime_with_struct() {
        lifetime_with_struct();
    }

    #[test]
    fn test_lifetime_with_impl() {
        lifetime_with_impl();
    }

    #[test]
    fn test_multiple_lifetimes() {
        multiple_lifetimes();
    }

    #[test]
    fn test_static_lifetime() {
        static_lifetime();
    }

    #[test]
    fn test_lifetime_bounds() {
        lifetime_bounds();
    }
}
