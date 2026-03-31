//! Borrowing and references.
//!
//! Demonstrates how to borrow values without taking ownership.

#![allow(clippy::all)]

/// Immutable borrow.
///
/// Borrow a value immutably with `&`.
/// Multiple immutable borrows can exist simultaneously.
pub fn immutable_borrow() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // s, r1, and r2 are all valid
    assert_eq!(r1, "hello");
    assert_eq!(r2, "hello");
}

/// Mutable borrow.
///
/// Borrow a value mutably with `&mut`.
/// Only one mutable borrow can exist at a time.
pub fn mutable_borrow() {
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(", world");
    assert_eq!(r, "hello, world");
}

/// Borrow checker rules.
///
/// Can't have mutable and immutable borrows simultaneously.
/// This demonstrates the borrow checker in action.
pub fn borrow_rules() {
    let mut s = String::from("hello");

    // Immutable borrow first
    let r1 = &s;
    let r2 = &s;

    // Mutable borrow after immutable borrows are no longer used
    let _ = r1;
    let _ = r2;
    let r3 = &mut s;
    r3.push_str("!");

    assert_eq!(s, "hello!");
}

/// Deref coercion.
///
/// &String automatically coerces to &str.
pub fn deref_coercion() {
    let s = String::from("hello");
    let r: &str = &s;
    assert_eq!(r, "hello");
}

/// Borrow in function parameters.
///
/// Functions can borrow instead of taking ownership.
pub fn borrow_in_function() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    // s is still valid
    assert_eq!(len, 5);
    assert_eq!(s, "hello");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Mutable borrow in function.
///
/// Functions can mutably borrow to modify values.
pub fn mutable_borrow_in_function() {
    let mut s = String::from("hello");
    add_exclamation(&mut s);
    assert_eq!(s, "hello!");
}

fn add_exclamation(s: &mut String) {
    s.push('!');
}

/// Slice borrow.
///
/// Slices are borrowed views into a collection.
pub fn slice_borrow() {
    let s = String::from("hello world");
    let slice = &s[0..5];
    assert_eq!(slice, "hello");
}

/// Borrow with method chaining.
///
/// Methods can borrow self and return references.
pub fn borrow_with_methods() {
    let mut s = String::from("hello");
    let first = s.as_mut_str();
    first.make_ascii_uppercase();
    assert_eq!(s, "HELLO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_borrow() {
        immutable_borrow();
    }

    #[test]
    fn test_mutable_borrow() {
        mutable_borrow();
    }

    #[test]
    fn test_borrow_rules() {
        borrow_rules();
    }

    #[test]
    fn test_deref_coercion() {
        deref_coercion();
    }

    #[test]
    fn test_borrow_in_function() {
        borrow_in_function();
    }

    #[test]
    fn test_mutable_borrow_in_function() {
        mutable_borrow_in_function();
    }

    #[test]
    fn test_slice_borrow() {
        slice_borrow();
    }

    #[test]
    fn test_borrow_with_methods() {
        borrow_with_methods();
    }
}
