//! Generics — type parameters for reusable code.
//!
//! Demonstrates generic functions, structs, trait bounds,
//! and the `where` clause for complex constraints.

/// Generic function with type parameter.
///
/// `T` is a type parameter, resolved at compile time.
pub fn generic_function() {
    fn print_type<T: std::fmt::Display>(value: T) {
        println!("{value}");
    }

    print_type(42);
    print_type("hello");
}

/// Multiple type parameters.
///
/// Functions can have multiple generic types.
pub fn generic_multiple_types() {
    fn pair<T, U>(a: T, b: U) -> (T, U) {
        (a, b)
    }

    let result = pair(1, "two");
    assert_eq!(result, (1, "two"));
}

/// Generic struct.
///
/// Structs can hold values of generic type.
pub fn generic_struct() {
    struct Container<T> {
        value: T,
    }

    let int_container = Container { value: 42 };
    assert_eq!(int_container.value, 42);

    let str_container = Container { value: "hello" };
    assert_eq!(str_container.value, "hello");
}

/// Generic struct with multiple fields.
///
/// Same type parameter across fields.
pub fn generic_struct_multiple() {
    struct Pair<T> {
        first: T,
        second: T,
    }

    let pair = Pair {
        first: 10,
        second: 20,
    };
    assert_eq!(pair.first + pair.second, 30);
}

/// Generic enum.
///
/// Enums can also be generic (like Option, Result).
pub fn generic_enum() {
    enum Maybe<T> {
        Yes(T),
        No,
    }

    let yes: Maybe<i32> = Maybe::Yes(42);
    let no: Maybe<i32> = Maybe::No;

    match yes {
        Maybe::Yes(v) => assert_eq!(v, 42),
        Maybe::No => panic!("unexpected"),
    }
    match no {
        Maybe::No => {}
        Maybe::Yes(_) => panic!("unexpected"),
    }
}

/// Trait bound on function.
///
/// `T: Trait` restricts acceptable types.
pub fn trait_bound_function() {
    fn display_sum<T: std::fmt::Display + std::iter::Sum>(values: Vec<T>) -> String {
        let total: T = values.into_iter().sum();
        format!("{total}")
    }

    let nums = vec![1, 2, 3];
    let sum = display_sum(nums);
    assert_eq!(sum, "6");
}

/// Trait bound with `impl Trait` syntax.
///
/// Shorthand for simple cases.
pub fn impl_trait_syntax() {
    fn double(value: impl std::ops::Mul<i32, Output = i32> + Copy) -> i32 {
        value * 2
    }

    let result = double(5i32);
    assert_eq!(result, 10);
}

/// `where` clause for complex bounds.
///
/// Cleaner syntax for multiple constraints.
pub fn where_clause() {
    fn process<T, U>(t: T, u: U) -> String
    where
        T: std::fmt::Display + Clone + std::fmt::Debug,
        U: std::fmt::Debug,
    {
        format!("{t:?} and {u:?}")
    }

    let result = process("hello", 42);
    assert!(result.contains("hello"));
}

/// Generic impl block.
///
/// Methods on generic structs.
pub fn generic_impl() {
    struct Wrapper<T> {
        value: T,
    }

    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }

        fn get(&self) -> &T {
            &self.value
        }
    }

    let wrapper = Wrapper::new(100);
    assert_eq!(wrapper.get(), &100);
}

/// Impl for specific type.
///
/// Specialized implementation for concrete type.
pub fn impl_specific_type() {
    struct Wrapper<T> {
        value: T,
    }

    impl Wrapper<i32> {
        fn double(&self) -> i32 {
            self.value * 2
        }
    }

    let wrapper = Wrapper { value: 21 };
    assert_eq!(wrapper.double(), 42);
}

/// Associated types in traits.
///
/// Traits can define type placeholders.
pub fn associated_types() {
    trait Container {
        type Item;
        fn get(&self) -> Self::Item;
    }

    struct IntContainer {
        value: i32,
    }

    impl Container for IntContainer {
        type Item = i32;

        fn get(&self) -> Self::Item {
            self.value
        }
    }

    let container = IntContainer { value: 42 };
    assert_eq!(container.get(), 42);
}

/// Const generics.
///
/// Type parameters for compile-time constants.
pub fn const_generics() {
    fn get_array_len<T, const N: usize>(_arr: [T; N]) -> usize {
        N
    }

    let arr = [1, 2, 3, 4, 5];
    let len = get_array_len(arr);
    assert_eq!(len, 5);
}

/// Generic array struct.
///
/// Fixed-size array with const generic.
pub fn generic_array_struct() {
    struct FixedArray<T, const N: usize> {
        data: [T; N],
    }

    impl<T: Copy + Default, const N: usize> FixedArray<T, N> {
        fn new() -> Self {
            FixedArray {
                data: [T::default(); N],
            }
        }
    }

    let arr: FixedArray<i32, 3> = FixedArray::new();
    assert_eq!(arr.data, [0, 0, 0]);
}

/// Lifetime with generics.
///
/// Combining lifetimes and type parameters.
pub fn lifetime_with_generics() {
    struct RefWrapper<'a, T> {
        value: &'a T,
    }

    let x = 42;
    let wrapper = RefWrapper { value: &x };
    assert_eq!(*wrapper.value, 42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_function() {
        generic_function();
    }

    #[test]
    fn test_generic_multiple_types() {
        generic_multiple_types();
    }

    #[test]
    fn test_generic_struct() {
        generic_struct();
    }

    #[test]
    fn test_generic_struct_multiple() {
        generic_struct_multiple();
    }

    #[test]
    fn test_generic_enum() {
        generic_enum();
    }

    #[test]
    fn test_trait_bound_function() {
        trait_bound_function();
    }

    #[test]
    fn test_impl_trait_syntax() {
        impl_trait_syntax();
    }

    #[test]
    fn test_where_clause() {
        where_clause();
    }

    #[test]
    fn test_generic_impl() {
        generic_impl();
    }

    #[test]
    fn test_impl_specific_type() {
        impl_specific_type();
    }

    #[test]
    fn test_associated_types() {
        associated_types();
    }

    #[test]
    fn test_const_generics() {
        const_generics();
    }

    #[test]
    fn test_generic_array_struct() {
        generic_array_struct();
    }

    #[test]
    fn test_lifetime_with_generics() {
        lifetime_with_generics();
    }
}
