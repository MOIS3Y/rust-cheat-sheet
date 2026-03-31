//! `Vec<T>` operations.
//!
//! Demonstrates common methods for working with `Vec<T>`,
//! the growable array type in Rust.

#![allow(clippy::all)]

/// Create new Vec.
///
/// Creates an empty vector with type inference.
pub fn create_empty() {
    let vec: Vec<i32> = Vec::new();
    assert!(vec.is_empty());
}

/// Create Vec with macro.
///
/// Uses the `vec!` macro for concise initialization.
pub fn create_macro() {
    let vec = vec![1, 2, 3];
    assert_eq!(vec.len(), 3);
}

/// Push element to Vec.
///
/// Adds an element to the end of the vector.
pub fn push_element() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec, vec![1, 2]);
}

/// Pop element from Vec.
///
/// Removes and returns the last element, or `None` if empty.
pub fn pop_element() {
    let mut vec = vec![1, 2, 3];
    let last = vec.pop();
    assert_eq!(last, Some(3));
    assert_eq!(vec, vec![1, 2]);
}

/// Access element by index.
///
/// Returns a reference to the element at the given index.
/// Panics if out of bounds.
pub fn index_access() {
    let vec = vec![1, 2, 3];
    assert_eq!(vec[0], 1);
}

/// Safe access with get.
///
/// Returns `Option<&T>` - `None` for out-of-bounds access.
pub fn get_safe() {
    let vec = vec![1, 2, 3];
    assert_eq!(vec.get(0), Some(&1));
    assert_eq!(vec.get(10), None);
}

/// Remove element at index.
///
/// Removes and returns the element at the given index.
pub fn remove_at() {
    let mut vec = vec![1, 2, 3];
    let removed = vec.remove(1);
    assert_eq!(removed, 2);
    assert_eq!(vec, vec![1, 3]);
}

/// Drain range of elements.
///
/// Removes elements in the specified range, returning them.
pub fn drain_range() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let drained: Vec<_> = vec.drain(1..4).collect();
    assert_eq!(drained, vec![2, 3, 4]);
    assert_eq!(vec, vec![1, 5]);
}

/// Clear all elements.
///
/// Removes all elements, keeping capacity.
pub fn clear_all() {
    let mut vec = vec![1, 2, 3];
    vec.clear();
    assert!(vec.is_empty());
}

/// Extend with iterator.
///
/// Extends the vector with elements from an iterator.
pub fn extend_iter() {
    let mut vec = vec![1, 2];
    vec.extend(vec![3, 4]);
    assert_eq!(vec, vec![1, 2, 3, 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        create_empty();
    }

    #[test]
    fn test_create_macro() {
        create_macro();
    }

    #[test]
    fn test_push_element() {
        push_element();
    }

    #[test]
    fn test_pop_element() {
        pop_element();
    }

    #[test]
    fn test_index_access() {
        index_access();
    }

    #[test]
    fn test_get_safe() {
        get_safe();
    }

    #[test]
    fn test_remove_at() {
        remove_at();
    }

    #[test]
    fn test_drain_range() {
        drain_range();
    }

    #[test]
    fn test_clear_all() {
        clear_all();
    }

    #[test]
    fn test_extend_iter() {
        extend_iter();
    }
}
