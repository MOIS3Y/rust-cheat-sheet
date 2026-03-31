//! Closures — anonymous functions capturing environment.
//!
//! Demonstrates closure syntax, capture modes, Fn traits,
//! and common patterns with iterators.

/// Basic closure syntax.
///
/// Closures defined with `|args| body` syntax.
pub fn closure_basic() {
    let add_one = |x: i32| x + 1;
    let result = add_one(5);
    assert_eq!(result, 6);
}

/// Closure with multiple parameters.
///
/// Multiple args separated by commas.
pub fn closure_multiple_params() {
    let add = |a: i32, b: i32| a + b;
    let result = add(3, 4);
    assert_eq!(result, 7);
}

/// Closure with type annotations.
///
/// Types optional, inferred by compiler.
pub fn closure_type_annotations() {
    let add = |a: i32, b: i32| -> i32 { a + b };
    let result = add(10, 20);
    assert_eq!(result, 30);
}

/// Closure with block body.
///
/// Multi-line closures use `{ }` block.
pub fn closure_block_body() {
    let multiply = |a: i32, b: i32| {
        let result = a * b;
        result
    };
    let result = multiply(3, 4);
    assert_eq!(result, 12);
}

/// Closure capturing by reference.
///
/// Immutable borrow of environment.
pub fn closure_capture_ref() {
    let x = 10;
    let add_x = |y| x + y;
    let result = add_x(5);
    assert_eq!(result, 15);
    // x still accessible
    assert_eq!(x, 10);
}

/// Closure capturing by mutable reference.
///
/// Mutable borrow allows modification.
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
/// Transfers ownership to closure.
pub fn closure_move() {
    let s = String::from("hello");
    let print_s = move || {
        println!("{s}");
    };
    print_s();
    // s no longer accessible here
}

/// FnOnce — called once.
///
/// Consumes captured values.
pub fn fn_once_trait() {
    let s = String::from("hello");
    let consume = move || {
        println!("{s}");
    };
    consume();
    // consume() can only be called once
}

/// FnMut — can mutate environment.
///
/// Called multiple times, modifies captures.
pub fn fn_mut_trait() {
    let mut count = 0;
    let mut increment = || {
        count += 1;
    };
    increment();
    increment();
    assert_eq!(count, 2);
}

/// Fn — immutable access.
///
/// Called multiple times, no mutation.
pub fn fn_trait() {
    let x = 10;
    let add_x = |y| x + y;
    assert_eq!(add_x(5), 15);
    assert_eq!(add_x(10), 20);
}

/// Closure as function parameter.
///
/// Accept closures via generic or trait object.
pub fn closure_as_param() {
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    let result = apply(double, 5);
    assert_eq!(result, 10);
}

/// Closure returning closure.
///
/// Use `impl Fn` for return type.
pub fn closure_return_closure() {
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }

    let add_five = make_adder(5);
    assert_eq!(add_five(10), 15);
}

/// Closure in struct.
///
/// Store closures in fields.
pub fn closure_in_struct() {
    struct Processor<F>
    where
        F: Fn(i32) -> i32,
    {
        func: F,
    }

    let processor = Processor {
        func: |x| x * 2,
    };
    let result = (processor.func)(5);
    assert_eq!(result, 10);
}

/// Closure with iterators.
///
/// Common pattern: map, filter.
pub fn closure_with_iterators() {
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}

/// Closure with filter.
///
/// Predicate closure for filtering.
pub fn closure_with_filter() {
    let nums = vec![1, 2, 3, 4, 5];
    let evens: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 0).collect();
    assert_eq!(evens, vec![2, 4]);
}

/// Closure with fold.
///
/// Accumulate values with closure.
pub fn closure_with_fold() {
    let nums = vec![1, 2, 3, 4, 5];
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 15);
}

/// Chaining iterator closures.
///
/// Multiple operations in sequence.
pub fn closure_chain() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = nums
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 10)
        .collect();
    assert_eq!(result, vec![20, 40, 60]);
}

/// Closure with `move` for threads.
///
/// Required for sending to threads.
pub fn closure_move_for_thread() {
    let data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        sum
    });
    let result = handle.join().unwrap();
    assert_eq!(result, 6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_basic() {
        closure_basic();
    }

    #[test]
    fn test_closure_multiple_params() {
        closure_multiple_params();
    }

    #[test]
    fn test_closure_type_annotations() {
        closure_type_annotations();
    }

    #[test]
    fn test_closure_block_body() {
        closure_block_body();
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
    fn test_fn_once_trait() {
        fn_once_trait();
    }

    #[test]
    fn test_fn_mut_trait() {
        fn_mut_trait();
    }

    #[test]
    fn test_fn_trait() {
        fn_trait();
    }

    #[test]
    fn test_closure_as_param() {
        closure_as_param();
    }

    #[test]
    fn test_closure_return_closure() {
        closure_return_closure();
    }

    #[test]
    fn test_closure_in_struct() {
        closure_in_struct();
    }

    #[test]
    fn test_closure_with_iterators() {
        closure_with_iterators();
    }

    #[test]
    fn test_closure_with_filter() {
        closure_with_filter();
    }

    #[test]
    fn test_closure_with_fold() {
        closure_with_fold();
    }

    #[test]
    fn test_closure_chain() {
        closure_chain();
    }

    #[test]
    fn test_closure_move_for_thread() {
        closure_move_for_thread();
    }
}
