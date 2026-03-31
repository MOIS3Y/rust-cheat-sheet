//! Pattern matching operations.
//!
//! Demonstrates match expressions and pattern matching patterns.

#![allow(clippy::all)]
#![allow(dead_code)]

/// Basic match expression.
///
/// Match on enum variants and values.
pub fn basic_match() {
    let x = 5;

    let result = match x {
        1 => "one",
        2 => "two",
        3 | 4 | 5 => "three to five",
        _ => "many",
    };

    assert_eq!(result, "three to five");
}

/// Match with enum.
///
/// Destructure enum variants in match arms.
pub fn match_with_enum() {
    let coin = Coin::Quarter(UsState::Alaska);

    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            assert_eq!(state, UsState::Alaska);
            25
        }
    };

    assert_eq!(value, 25);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum UsState {
    Alaska,
    California,
    NewYork,
}

/// Match guards.
///
/// Add conditions to match arms with if guards.
pub fn match_guards() {
    let pair = (2, -2);

    let result = match pair {
        (x, y) if x == y => "equal",
        (x, y) if x + y == 0 => "sum to zero",
        (x, _) if x > 0 => "positive first",
        _ => "other",
    };

    assert_eq!(result, "sum to zero");
}

/// @ patterns.
///
/// Bind a variable while also testing the pattern.
pub fn at_patterns() {
    let x = Some(5);

    let result = match x {
        Some(n @ 1..=10) => n * 2,
        Some(n) => n,
        None => 0,
    };

    assert_eq!(result, 10);
}

/// If let for single pattern.
///
/// Concise syntax for handling one pattern.
pub fn if_let() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        assert_eq!(max, 3);
    }

    let config_min = None;
    let mut counter = 0;

    if let Some(min) = config_min {
        counter = min;
    }

    assert_eq!(counter, 0);
}

/// While let for loops.
///
/// Loop while pattern matches.
pub fn while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut sum = 0;

    while let Some(top) = stack.pop() {
        sum += top;
    }

    assert_eq!(sum, 6);
}

/// Destructuring tuples.
///
/// Unpack tuple values with patterns.
pub fn destructure_tuple() {
    let pair = (1, "hello");

    let (x, y) = pair;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");

    // Ignore parts with _
    let (a, _) = (10, "ignored");
    assert_eq!(a, 10);
}

/// Destructuring structs.
///
/// Extract struct fields with patterns.
pub fn destructure_struct() {
    let point = Point3d { x: 1, y: 2, z: 3 };

    let Point3d { x, y, z } = point;
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);

    // Rename fields
    let Point3d { x: a, y: _, z: _ } = point;
    assert_eq!(a, 1);
}

#[derive(Debug, PartialEq)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

/// Destructuring nested patterns.
///
/// Match deeply nested structures.
pub fn nested_destructure() {
    let nested = ((1, 2), (3, 4));

    let ((a, b), (c, d)) = nested;
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
    assert_eq!(d, 4);
}

/// Ignore values with _.
///
/// Use _ to skip values you don't need.
pub fn ignore_with_underscore() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();

    // Skip first two elements
    let _ = iter.next();
    let _ = iter.next();

    let third = iter.next();
    assert_eq!(third, Some(&3));
}

/// Match ranges.
///
/// Match numeric ranges with ..= syntax.
pub fn match_ranges() {
    let age = 25;

    let category = match age {
        0..=12 => "child",
        13..=19 => "teen",
        20..=64 => "adult",
        _ => "senior",
    };

    assert_eq!(category, "adult");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        basic_match();
    }

    #[test]
    fn test_match_with_enum() {
        match_with_enum();
    }

    #[test]
    fn test_match_guards() {
        match_guards();
    }

    #[test]
    fn test_at_patterns() {
        at_patterns();
    }

    #[test]
    fn test_if_let() {
        if_let();
    }

    #[test]
    fn test_while_let() {
        while_let();
    }

    #[test]
    fn test_destructure_tuple() {
        destructure_tuple();
    }

    #[test]
    fn test_destructure_struct() {
        destructure_struct();
    }

    #[test]
    fn test_nested_destructure() {
        nested_destructure();
    }

    #[test]
    fn test_ignore_with_underscore() {
        ignore_with_underscore();
    }

    #[test]
    fn test_match_ranges() {
        match_ranges();
    }
}
