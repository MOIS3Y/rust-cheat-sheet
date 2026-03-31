//! Advanced macro techniques.
//!
//! Demonstrates advanced patterns:
//! TT muncher, recursion, fragment specifiers.

/// Macro with different arm patterns.
///
/// Macros can have multiple arms like match.
pub fn macro_multiple_arms() {
    macro_rules! describe {
        (number $n:expr) => {
            format!("Number: {}", $n)
        };
        (text $t:expr) => {
            format!("Text: {}", $t)
        };
    }

    let num = describe!(number 42);
    let txt = describe!(text "hello");

    assert_eq!(num, "Number: 42");
    assert_eq!(txt, "Text: hello");
}

/// TT muncher pattern for parsing tokens.
///
/// TT muncher processes tokens one by one.
pub fn tt_muncher_pattern() {
    macro_rules! count_args {
        () => {
            0
        };
        ($first:tt $($rest:tt)*) => {
            1 + count_args!($($rest)*)
        };
    }

    let count = count_args!(a b c d);
    assert_eq!(count, 4);
}

/// Macro with internal rules.
///
/// Internal rules help with recursion.
pub fn macro_internal_rules() {
    macro_rules! sum {
        ($($vals:expr),*) => {
            sum!(@internal [$($vals),*])
        };
        (@internal [$first:expr $(,$rest:expr)*]) => {
            $first + sum!(@internal [$($rest),*])
        };
        (@internal []) => {
            0
        };
    }

    let result = sum!(1, 2, 3, 4);
    assert_eq!(result, 10);
}

/// Macro generating struct with methods.
///
/// Macros can generate entire code blocks.
pub fn macro_generate_struct() {
    macro_rules! simple_struct {
        ($name:ident { $($field:ident: $ty:ty),* }) => {
            struct $name {
                $(pub $field: $ty,)*
            }
        };
    }

    simple_struct!(Point { x: i32, y: i32 });

    let p = Point { x: 10, y: 20 };
    assert_eq!(p.x, 10);
    assert_eq!(p.y, 20);
}

/// Macro for implementing Deref.
///
/// Common pattern for newtype wrappers.
pub fn macro_implement_deref() {
    macro_rules! newtype {
        ($name:ident($ty:ty)) => {
            struct $name($ty);

            impl std::ops::Deref for $name {
                type Target = $ty;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        };
    }

    newtype!(Wrapper(i32));

    let w = Wrapper(42);
    assert_eq!(*w, 42);
}

/// Macro with visibility modifier.
///
/// `$vis:vis` matches pub, pub(crate), etc.
pub fn macro_with_visibility() {
    macro_rules! make_pub_struct {
        ($vis:vis $name:ident) => {
            $vis struct $name {
                pub value: i32,
            }
        };
    }

    make_pub_struct!(pub PublicStruct);

    let s = PublicStruct { value: 42 };
    assert_eq!(s.value, 42);

    // Demonstrate private struct
    #[allow(dead_code)]
    {
        make_pub_struct!(PrivateStruct);
    }
}

/// Macro for constant definition.
///
/// Useful for defining related constants.
pub fn macro_define_constants() {
    macro_rules! define_consts {
        ($($name:ident = $val:expr),* $(,)?) => {
            $(const $name: i32 = $val;)*
        };
    }

    define_consts! {
        MAX_SIZE = 100,
        MIN_SIZE = 1,
        DEFAULT = 50,
    }

    assert_eq!(MAX_SIZE, 100);
    assert_eq!(MIN_SIZE, 1);
    assert_eq!(DEFAULT, 50);
}

/// Macro with literal pattern.
///
/// Match specific literal values.
pub fn macro_literal_pattern() {
    macro_rules! handle_code {
        (200) => { "OK" };
        (404) => { "Not Found" };
        (500) => { "Internal Error" };
        ($code:expr) => { "Unknown" };
    }

    assert_eq!(handle_code!(200), "OK");
    assert_eq!(handle_code!(404), "Not Found");
    assert_eq!(handle_code!(999), "Unknown");
}

/// Macro for builder pattern.
///
/// Generate builder struct with methods.
pub fn macro_builder_pattern() {
    macro_rules! builder {
        ($name:ident { $($field:ident: $ty:ty),* }) => {
            struct $name {
                $(pub $field: Option<$ty>,)*
            }

            impl $name {
                fn new() -> Self {
                    $name {
                        $($field: None,)*
                    }
                }

                $(
                    fn $field(mut self, val: $ty) -> Self {
                        self.$field = Some(val);
                        self
                    }
                )*
            }
        };
    }

    builder!(Config { timeout: u32, retries: u32 });

    let _config = Config::new()
        .timeout(30)
        .retries(3);
}

/// Macro hygiene demonstration.
///
/// Macro variables don't conflict with outer scope.
pub fn macro_hygiene() {
    let x = 100;

    macro_rules! make_x {
        () => {{
            let x = 42;
            x
        }};
    }

    let result = make_x!();
    assert_eq!(result, 42);
    assert_eq!(x, 100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_multiple_arms() {
        macro_multiple_arms();
    }

    #[test]
    fn test_tt_muncher_pattern() {
        tt_muncher_pattern();
    }

    #[test]
    fn test_macro_internal_rules() {
        macro_internal_rules();
    }

    #[test]
    fn test_macro_generate_struct() {
        macro_generate_struct();
    }

    #[test]
    fn test_macro_implement_deref() {
        macro_implement_deref();
    }

    #[test]
    fn test_macro_with_visibility() {
        macro_with_visibility();
    }

    #[test]
    fn test_macro_define_constants() {
        macro_define_constants();
    }

    #[test]
    fn test_macro_literal_pattern() {
        macro_literal_pattern();
    }

    #[test]
    fn test_macro_builder_pattern() {
        macro_builder_pattern();
    }

    #[test]
    fn test_macro_hygiene() {
        macro_hygiene();
    }
}
