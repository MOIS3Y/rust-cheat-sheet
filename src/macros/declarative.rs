//! Declarative macros with `macro_rules!`.
//!
//! Demonstrates basic `macro_rules!` syntax,
//! pattern matching, and repetition.

/// Simple macro without parameters.
///
/// `macro_rules!` defines declarative macros.
/// Macro names conventionally end with `!`.
pub fn macro_simple() {
    macro_rules! say_hello {
        () => {
            println!("Hello from macro!");
        };
    }

    say_hello!();
}

/// Macro with expression parameter.
///
/// `$name:expr` matches any Rust expression.
pub fn macro_with_expr() {
    macro_rules! print_value {
        ($val:expr) => {
            println!("Value: {}", $val);
        };
    }

    print_value!(42);
    print_value!("hello");
}

/// Macro with multiple parameters.
///
/// Multiple parameters separated by commas.
pub fn macro_multiple_params() {
    macro_rules! print_pair {
        ($a:expr, $b:expr) => {
            println!("Pair: {}, {}", $a, $b);
        };
    }

    print_pair!(1, 2);
    print_pair!("foo", "bar");
}

/// Macro with identifier parameter.
///
/// `$name:ident` matches variable/function names.
pub fn macro_with_ident() {
    macro_rules! declare_var {
        ($name:ident) => {
            let $name = 42;
        };
    }

    declare_var!(x);
    assert_eq!(x, 42);
}

/// Macro with repetition (zero or more).
///
/// `$($val:expr),*` matches comma-separated expressions.
pub fn macro_repetition_zero_or_more() {
    macro_rules! print_all {
        ($($val:expr),*) => {
            $(println!("Value: {}", $val);)*
        };
    }

    print_all!(1);
    print_all!(1, 2, 3);
    print_all!();
}

/// Macro with repetition (one or more).
///
/// `$($val:expr),+` requires at least one expression.
pub fn macro_repetition_one_or_more() {
    macro_rules! print_non_empty {
        ($($val:expr),+) => {
            $(println!("Value: {}", $val);)*
        };
    }

    print_non_empty!(1);
    print_non_empty!(1, 2, 3);
}

/// Macro with optional trailing comma.
///
/// `$($val:expr),* $(,)?` allows optional trailing comma.
pub fn macro_trailing_comma() {
    macro_rules! print_flexible {
        ($($val:expr),* $(,)?) => {
            $(println!("Value: {}", $val);)*
        };
    }

    print_flexible!(1, 2, 3);
    print_flexible!(1, 2, 3,);
}

/// Macro returning expression.
///
/// Macros can return values for use in expressions.
pub fn macro_return_value() {
    macro_rules! double {
        ($val:expr) => {
            $val * 2
        };
    }

    let result = double!(5);
    assert_eq!(result, 10);
}

/// Macro with block parameter.
///
/// `$block:block` matches code blocks `{ }`.
pub fn macro_with_block() {
    macro_rules! run_twice {
        ($block:block) => {
            $block
            $block
        };
    }

    let mut count = 0;
    run_twice!({
        count += 1;
    });
    assert_eq!(count, 2);
}

/// Macro with lifetime parameter.
///
/// Macros can work with lifetimes in signatures.
pub fn macro_with_lifetime() {
    macro_rules! make_ref {
        ($val:expr) => {{
            let r: &i32 = &$val;
            r
        }};
    }

    let x = 42;
    let r = make_ref!(x);
    assert_eq!(*r, 42);
}

/// Macro with type parameter.
///
/// `$ty:ty` matches Rust types.
pub fn macro_with_type() {
    macro_rules! default_value {
        ($ty:ty) => {
            <$ty>::default()
        };
    }

    let s: String = default_value!(String);
    assert_eq!(s, "");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_simple() {
        macro_simple();
    }

    #[test]
    fn test_macro_with_expr() {
        macro_with_expr();
    }

    #[test]
    fn test_macro_multiple_params() {
        macro_multiple_params();
    }

    #[test]
    fn test_macro_with_ident() {
        macro_with_ident();
    }

    #[test]
    fn test_macro_repetition_zero_or_more() {
        macro_repetition_zero_or_more();
    }

    #[test]
    fn test_macro_repetition_one_or_more() {
        macro_repetition_one_or_more();
    }

    #[test]
    fn test_macro_trailing_comma() {
        macro_trailing_comma();
    }

    #[test]
    fn test_macro_return_value() {
        macro_return_value();
    }

    #[test]
    fn test_macro_with_block() {
        macro_with_block();
    }

    #[test]
    fn test_macro_with_lifetime() {
        macro_with_lifetime();
    }

    #[test]
    fn test_macro_with_type() {
        macro_with_type();
    }
}
