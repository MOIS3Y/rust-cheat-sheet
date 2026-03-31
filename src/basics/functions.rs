//! Functions — definition, parameters, and return values.
//!
//! Demonstrates function declaration syntax, parameters,
//! return types, and expression vs statement rules.

/// Function with no parameters.
///
/// Functions declared with `fn` keyword can have no parameters.
pub fn function_no_params() {

    fn greet() {
        println!("Hello!");
    }
    greet();
}

/// Function with single parameter.
///
/// Parameters declared as `name: Type`.
pub fn function_single_param() {

    fn greet(name: &str) {
        println!("Hello, {name}!");
    }
    greet("Alice");
}

/// Function with multiple parameters.
///
/// Multiple parameters separated by commas.
pub fn function_multiple_params() {

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let result = add(3, 5);
    assert_eq!(result, 8);
}

/// Function with return type.
///
/// Return type specified after `->`.
pub fn function_return_type() {

    fn double(x: i32) -> i32 {
        x * 2
    }
    let result = double(5);
    assert_eq!(result, 10);
}

/// Function returning with expression.
///
/// Last expression without semicolon is returned.
pub fn function_return_expression() {

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    let result = multiply(3, 4);
    assert_eq!(result, 12);
}

/// Function with early return.
///
/// `return` keyword exits function immediately.
pub fn function_early_return() {

    fn find_first_positive(numbers: &[i32]) -> i32 {
        for &n in numbers {
            if n > 0 {
                return n;
            }
        }
        0
    }
    let nums = [-1, -2, 3, 4];
    let result = find_first_positive(&nums);
    assert_eq!(result, 3);
}

/// Function returning tuple.
///
/// Multiple values returned as tuple.
pub fn function_return_tuple() {

    fn get_point() -> (i32, i32) {
        (10, 20)
    }
    let point = get_point();
    assert_eq!(point, (10, 20));
}

/// Function with mutable parameter.
///
/// `mut` makes parameter mutable inside function.
pub fn function_mutable_param() {

    fn increment(mut value: i32) -> i32 {
        value += 1;
        value
    }
    let result = increment(5);
    assert_eq!(result, 6);
}

/// Function with reference parameter.
///
/// Borrowing avoids ownership transfer.
pub fn function_reference_param() {

    fn double_ref(value: &i32) -> i32 {
        value * 2
    }
    let x = 10;
    let result = double_ref(&x);
    assert_eq!(result, 20);
    assert_eq!(x, 10);
}

/// Function with mutable reference parameter.
///
/// Mutable borrow allows modification.
pub fn function_mutable_ref_param() {

    fn add_one(value: &mut i32) {
        *value += 1;
    }
    let mut x = 10;
    add_one(&mut x);
    assert_eq!(x, 11);
}

/// Function with const parameter.
///
/// Const generics for compile-time constants.
pub fn function_const_param() {

    fn get_len<T, const N: usize>(_arr: [T; N]) -> usize {
        N
    }
    let arr = [1, 2, 3, 4, 5];
    let len = get_len(arr);
    assert_eq!(len, 5);
}

/// Recursive function.
///
/// Functions can call themselves.
pub fn function_recursive() {

    fn factorial(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
    assert_eq!(factorial(5), 120);
}

/// Closure with no capture.
///
/// Closures are anonymous functions.
pub fn closure_no_capture() {

    let add = |a: i32, b: i32| a + b;
    let result = add(3, 4);
    assert_eq!(result, 7);
}

/// Closure capturing by reference.
///
/// Closures capture variables from environment.
pub fn closure_capture_ref() {

    let x = 10;
    let add_x = |y| x + y;
    let result = add_x(5);
    assert_eq!(result, 15);
}

/// Closure capturing by mutable reference.
///
/// `mut` allows modifying captured variables.
pub fn closure_capture_mut() {

    let mut count = 0;
    let mut increment = || {
        count += 1;
    };
    increment();
    increment();
    assert_eq!(count, 2);
}

/// Closure with move.
///
/// `move` transfers ownership to closure.
pub fn closure_move() {

    let x = String::from("hello");
    let print_x = move || {
        println!("{x}");
    };
    print_x();
}

/// Function as parameter.
///
/// Functions passed as arguments.
pub fn function_as_parameter() {

    fn double(x: i32) -> i32 {
        x * 2
    }
    fn apply(f: fn(i32) -> i32, val: i32) -> i32 {
        f(val)
    }
    let result = apply(double, 5);
    assert_eq!(result, 10);
}

/// Function returning closure.
///
/// Closures returned with `impl Fn`.
pub fn function_return_closure() {

    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }
    let add_five = make_adder(5);
    let result = add_five(10);
    assert_eq!(result, 15);
}

/// Diverging function.
///
/// Functions that never return have `!` type.
pub fn function_diverging() {

    fn panic_on_negative(n: i32) -> ! {
        panic!("negative number: {n}")
    }
    // Example demonstrates the type signature.
    // Calling would panic, so we just verify the function exists.
    let _f: fn(i32) -> ! = panic_on_negative;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_no_params() {
        function_no_params();
    }

    #[test]
    fn test_function_single_param() {
        function_single_param();
    }

    #[test]
    fn test_function_multiple_params() {
        function_multiple_params();
    }

    #[test]
    fn test_function_return_type() {
        function_return_type();
    }

    #[test]
    fn test_function_return_expression() {
        function_return_expression();
    }

    #[test]
    fn test_function_early_return() {
        function_early_return();
    }

    #[test]
    fn test_function_return_tuple() {
        function_return_tuple();
    }

    #[test]
    fn test_function_mutable_param() {
        function_mutable_param();
    }

    #[test]
    fn test_function_reference_param() {
        function_reference_param();
    }

    #[test]
    fn test_function_mutable_ref_param() {
        function_mutable_ref_param();
    }

    #[test]
    fn test_function_const_param() {
        function_const_param();
    }

    #[test]
    fn test_function_recursive() {
        function_recursive();
    }

    #[test]
    fn test_closure_no_capture() {
        closure_no_capture();
    }

    #[test]
    fn test_closure_capture_ref() {
        closure_capture_ref();
    }

    #[test]
    fn test_closure_capture_mut() {
        closure_capture_mut();
    }

    #[test]
    fn test_closure_move() {
        closure_move();
    }

    #[test]
    fn test_function_as_parameter() {
        function_as_parameter();
    }

    #[test]
    fn test_function_return_closure() {
        function_return_closure();
    }

    #[test]
    fn test_function_diverging() {
        function_diverging();
    }
}
