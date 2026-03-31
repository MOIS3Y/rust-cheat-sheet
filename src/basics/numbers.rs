//! Numeric type operations.
//!
//! Demonstrates common methods for working with numeric types,
//! including overflow handling and mathematical operations.

#![allow(clippy::all)]

/// Saturating addition without overflow.
///
/// Returns the maximum value on overflow instead of wrapping.
pub fn saturating_add() {
    let max = u32::MAX;
    assert_eq!(max.saturating_add(1), u32::MAX);
    assert_eq!(0u32.saturating_sub(1), 0);
}

/// Checked arithmetic returning Option.
///
/// Returns `None` on overflow, `Some(result)` otherwise.
pub fn checked_arithmetic() {
    let max = u32::MAX;
    assert_eq!(max.checked_add(1), None);
    assert_eq!(10u32.checked_sub(5), Some(5));
}

/// Wrapping arithmetic with cyclic overflow.
///
/// Wraps around on overflow (two's complement).
pub fn wrapping_overflow() {
    let max = u32::MAX;
    assert_eq!(max.wrapping_add(1), 0);
}

/// Overflowing arithmetic returning flag.
///
/// Returns a tuple of (result, overflowed).
pub fn overflowing_flag() {
    let max = u32::MAX;
    let (result, overflowed) = max.overflowing_add(1);
    assert_eq!(result, 0);
    assert!(overflowed);
}

/// Power operation.
///
/// Raises the number to the specified power.
pub fn power_operation() {
    assert_eq!(2u32.pow(10), 1024);
}

/// Absolute value and sign.
///
/// `abs` returns absolute value, `signum` returns the sign.
pub fn abs_signum() {
    assert_eq!((-5i32).abs(), 5);
    assert_eq!((-5i32).signum(), -1);
    assert_eq!((0i32).signum(), 0);
    assert_eq!((5i32).signum(), 1);
}

/// Minimum and maximum values.
///
/// Returns the smaller or larger of two values.
pub fn min_max() {
    assert_eq!(10i32.min(5), 5);
    assert_eq!(10i32.max(5), 10);
}

/// Clamp value to range.
///
/// Restricts the value to the specified range.
pub fn clamp_range() {
    assert_eq!(5i32.clamp(10, 20), 10);
    assert_eq!(15i32.clamp(10, 20), 15);
    assert_eq!(25i32.clamp(10, 20), 20);
}

/// Check if number is power of two.
///
/// Returns `true` if the number is a power of two.
pub fn is_power_of_two() {
    assert!(16u32.is_power_of_two());
    assert!(!18u32.is_power_of_two());
}

/// Next power of two.
///
/// Returns the smallest power of two greater than or equal.
pub fn next_power_of_two() {
    assert_eq!(10u32.next_power_of_two(), 16);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saturating_add() {
        saturating_add();
    }

    #[test]
    fn test_checked_arithmetic() {
        checked_arithmetic();
    }

    #[test]
    fn test_wrapping_overflow() {
        wrapping_overflow();
    }

    #[test]
    fn test_overflowing_flag() {
        overflowing_flag();
    }

    #[test]
    fn test_power_operation() {
        power_operation();
    }

    #[test]
    fn test_abs_signum() {
        abs_signum();
    }

    #[test]
    fn test_min_max() {
        min_max();
    }

    #[test]
    fn test_clamp_range() {
        clamp_range();
    }

    #[test]
    fn test_is_power_of_two() {
        is_power_of_two();
    }

    #[test]
    fn test_next_power_of_two() {
        next_power_of_two();
    }
}
