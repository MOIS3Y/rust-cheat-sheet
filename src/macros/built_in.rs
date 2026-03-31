//! Built-in macros from the standard library.
//!
//! Demonstrates commonly used built-in macros:
//! printing, debugging, assertions, and compilation helpers.

/// `println!` macro for printing to stdout.
///
/// `println!` formats and prints with newline.
pub fn println_macro() {
    println!("Hello, world!");
    println!("Value: {}", 42);
    println!("Multiple: {}, {}", "foo", "bar");
}

/// `format!` macro for string formatting.
///
/// `format!` returns a `String` instead of printing.
pub fn format_macro() {
    let s = format!("Hello, {}!", "world");
    assert_eq!(s, "Hello, world!");
}

/// `print!` macro without newline.
///
/// `print!` is like `println!` but without `\n`.
pub fn print_macro() {
    print!("No newline");
    print!(" continued");
    println!();
}

/// `eprintln!` for stderr output.
///
/// `eprintln!` prints to standard error.
pub fn eprintln_macro() {
    eprintln!("This goes to stderr");
}

/// `dbg!` macro for quick debugging.
///
/// `dbg!` prints expression and value, returns value.
pub fn dbg_macro() {
    let x = 42;
    let result = dbg!(x * 2);
    assert_eq!(result, 84);
}

/// `assert!` macro for conditions.
///
/// `assert!` panics if condition is false.
pub fn assert_macro() {
    let condition = true;
    assert!(condition);
}

/// `assert!` with custom message.
///
/// Second argument is error message.
pub fn assert_with_message() {
    let x = 5;
    assert!(x > 0, "x must be positive, got {}", x);
}

/// `assert_eq!` for equality checks.
///
/// `assert_eq!` prints values on failure.
pub fn assert_eq_macro() {
    let a = 10;
    let b = 10;
    assert_eq!(a, b);
}

/// `assert_ne!` for inequality checks.
///
/// `assert_ne!` panics if values are equal.
pub fn assert_ne_macro() {
    let a = 10;
    let b = 20;
    assert_ne!(a, b);
}

/// `panic!` macro for explicit panics.
///
/// `panic!` causes immediate panic with message.
#[allow(dead_code)]
pub fn panic_macro() {
    fn always_fails() -> Result<(), &'static str> {
        Err("error occurred")
    }

    if let Err(e) = always_fails() {
        panic!("Fatal error: {}", e);
    }
}

/// `todo!` macro for unfinished code.
///
/// `todo!` panics with "not yet implemented" message.
#[allow(dead_code)]
pub fn todo_macro() {
    fn unimplemented_feature() {
        todo!("implement this later");
    }

    // Uncommenting will panic:
    // unimplemented_feature();
}

/// `unreachable!` for unreachable code.
///
/// `unreachable!` panics if code is reached.
pub fn unreachable_macro() {
    fn handle_option(x: Option<i32>) -> i32 {
        match x {
            Some(v) => v,
            None => unreachable!("None should not happen here"),
        }
    }

    let result = handle_option(Some(42));
    assert_eq!(result, 42);
}

/// `concat!` for compile-time string concat.
///
/// `concat!` joins literals at compile time.
pub fn concat_macro() {
    const GREETING: &str = concat!("Hello", ", ", "world", "!");
    assert_eq!(GREETING, "Hello, world!");
}

/// `stringify!` converts tokens to string.
///
/// `stringify!` turns expression into string literal.
pub fn stringify_macro() {
    let s = stringify!(1 + 2);
    assert_eq!(s, "1 + 2");
}

/// `env!` for compile-time env variables.
///
/// `env!` reads environment at compile time.
pub fn env_macro() {
    let home = env!("HOME");
    assert!(!home.is_empty());
}

/// `option_env!` for optional env variables.
///
/// `option_env!` returns `Option<&'static str>`.
pub fn option_env_macro() {
    match option_env!("CUSTOM_VAR") {
        Some(val) => println!("CUSTOM_VAR = {}", val),
        None => println!("CUSTOM_VAR not set"),
    }
}

/// `include_str!` for including files.
///
/// `include_str!` embeds file content as `&str`.
pub fn include_str_macro() {
    let content = include_str!("../../README.md");
    assert!(content.contains("Rust"));
}

/// `vec!` macro for creating vectors.
///
/// `vec!` creates Vec with initial values.
pub fn vec_macro() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![0; 5];
    assert_eq!(v1.len(), 3);
    assert_eq!(v2, vec![0, 0, 0, 0, 0]);
}

/// `matches!` macro for pattern matching.
///
/// `matches!` returns bool for pattern match.
pub fn matches_macro() {
    let x = Some(42);
    assert!(matches!(x, Some(_)));
    assert!(!matches!(x, None));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_println_macro() {
        println_macro();
    }

    #[test]
    fn test_format_macro() {
        format_macro();
    }

    #[test]
    fn test_print_macro() {
        print_macro();
    }

    #[test]
    fn test_eprintln_macro() {
        eprintln_macro();
    }

    #[test]
    fn test_dbg_macro() {
        dbg_macro();
    }

    #[test]
    fn test_assert_macro() {
        assert_macro();
    }

    #[test]
    fn test_assert_with_message() {
        assert_with_message();
    }

    #[test]
    fn test_assert_eq_macro() {
        assert_eq_macro();
    }

    #[test]
    fn test_assert_ne_macro() {
        assert_ne_macro();
    }

    #[test]
    fn test_unreachable_macro() {
        unreachable_macro();
    }

    #[test]
    fn test_concat_macro() {
        concat_macro();
    }

    #[test]
    fn test_stringify_macro() {
        stringify_macro();
    }

    #[test]
    fn test_env_macro() {
        env_macro();
    }

    #[test]
    fn test_option_env_macro() {
        option_env_macro();
    }

    #[test]
    fn test_include_str_macro() {
        include_str_macro();
    }

    #[test]
    fn test_vec_macro() {
        vec_macro();
    }

    #[test]
    fn test_matches_macro() {
        matches_macro();
    }
}
