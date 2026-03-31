//! Loops — `loop`, `while`, `for`, and control flow.
//!
//! Demonstrates loop constructs, iteration, and
//! loop control with `break` and `continue`.

/// Infinite loop with break.
///
/// `loop` creates infinite loop, exited with `break`.
pub fn loop_with_break() {
    let mut count = 0;
    loop {
        count += 1;
        if count >= 3 {
            break;
        }
    }
    assert_eq!(count, 3);
}

/// Loop returning value.
///
/// `break` can return value from loop.
pub fn loop_return_value() {
    let mut n = 1;
    let result = loop {
        if n % 3 == 0 {
            break n;
        }
        n += 1;
    };
    assert_eq!(result, 3);
}

/// While loop.
///
/// `while` loops while condition is true.
pub fn while_loop() {
    let mut n = 0;
    while n < 3 {
        n += 1;
    }
    assert_eq!(n, 3);
}

/// While with decrement.
///
/// Countdown pattern with while.
pub fn while_countdown() {
    let mut n = 3;
    let mut sum = 0;
    while n > 0 {
        sum += n;
        n -= 1;
    }
    assert_eq!(sum, 6);
}

/// For loop over range.
///
/// `for` iterates over ranges and collections.
pub fn for_over_range() {
    let mut sum = 0;
    for i in 1..=3 {
        sum += i;
    }
    assert_eq!(sum, 6);
}

/// For over array.
///
/// Iterating over array elements.
pub fn for_over_array() {
    let arr = [1, 2, 3];
    let mut sum = 0;
    for value in arr {
        sum += value;
    }
    assert_eq!(sum, 6);
}

/// For with reference.
///
/// Borrowing elements avoids copy.
pub fn for_with_reference() {
    let arr = [10, 20, 30];
    let mut sum = 0;
    for value in &arr {
        sum += value;
    }
    assert_eq!(sum, 60);
}

/// For with enumerate.
///
/// `enumerate` adds index to iteration.
pub fn for_with_enumerate() {
    let arr = ['a', 'b', 'c'];
    let mut count = 0;
    for (i, _ch) in arr.iter().enumerate() {
        count = i;
    }
    assert_eq!(count, 2);
}

/// Continue keyword.
///
/// `continue` skips to next iteration.
pub fn continue_keyword() {
    let mut sum = 0;
    for i in 1..=5 {
        if i % 2 == 0 {
            continue;
        }
        sum += i;
    }
    assert_eq!(sum, 9);
}

/// Nested loops.
///
/// Loops can be nested for multi-dimensional iteration.
pub fn nested_loops() {
    let mut count = 0;
    for i in 1..=2 {
        for j in 1..=3 {
            count += i * j;
        }
    }
    assert_eq!(count, 18);
}

/// Loop labels.
///
/// Labels allow breaking from outer loops.
pub fn loop_labels() {
    let mut found = None;
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i + j == 5 {
                found = Some((i, j));
                break 'outer;
            }
        }
    }
    assert_eq!(found, Some((2, 3)));
}

/// For over string chars.
///
/// `chars()` iterates over Unicode characters.
pub fn for_over_chars() {
    let s = "abc";
    let mut count = 0;
    for _ch in s.chars() {
        count += 1;
    }
    assert_eq!(count, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_with_break() {
        loop_with_break();
    }

    #[test]
    fn test_loop_return_value() {
        loop_return_value();
    }

    #[test]
    fn test_while_loop() {
        while_loop();
    }

    #[test]
    fn test_while_countdown() {
        while_countdown();
    }

    #[test]
    fn test_for_over_range() {
        for_over_range();
    }

    #[test]
    fn test_for_over_array() {
        for_over_array();
    }

    #[test]
    fn test_for_with_reference() {
        for_with_reference();
    }

    #[test]
    fn test_for_with_enumerate() {
        for_with_enumerate();
    }

    #[test]
    fn test_continue_keyword() {
        continue_keyword();
    }

    #[test]
    fn test_nested_loops() {
        nested_loops();
    }

    #[test]
    fn test_loop_labels() {
        loop_labels();
    }

    #[test]
    fn test_for_over_chars() {
        for_over_chars();
    }
}
