//! Array and slice operations.
//!
//! Demonstrates common methods for working with arrays and slices,
//! the fixed-size and dynamic view types for sequences in Rust.

/// Get slice length.
///
/// Returns the number of elements in the slice.
pub fn slice_len() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    assert_eq!(slice.len(), 3);
}

/// Check if slice is empty.
///
/// Returns `true` if the slice contains no elements.
pub fn is_empty_check() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    assert!(!slice.is_empty());
}

/// Safe element access with get.
///
/// Returns `Option<&T>` - `None` for out-of-bounds access.
pub fn get_element() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    assert_eq!(slice.first(), Some(&2));
    assert_eq!(slice.get(10), None);
}

/// Get first and last elements.
///
/// Returns `Option<&T>` for the first or last element.
pub fn first_last() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    assert_eq!(slice.first(), Some(&2));
    assert_eq!(slice.last(), Some(&4));
}

/// Split slice at index.
///
/// Splits the slice into two at the given index.
pub fn split_at() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    let (left, right) = slice.split_at(1);
    assert_eq!(left, &[2]);
    assert_eq!(right, &[3, 4]);
}

/// Binary search in sorted slice.
///
/// Returns `Ok(index)` if found, `Err(insert_pos)` otherwise.
/// Requires the slice to be sorted.
pub fn binary_search() {
    let sorted = [1, 2, 3, 4, 5];
    assert_eq!(sorted.binary_search(&3), Ok(2));
    assert_eq!(sorted.binary_search(&6), Err(5));
}

/// Check if slice contains element.
///
/// Returns `true` if the element is present in the slice.
pub fn contains_element() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    assert!(slice.contains(&3));
    assert!(!slice.contains(&10));
}

/// Repeat array elements.
///
/// Creates a new Vec with repeated copies of the slice.
pub fn repeat_elements() {
    let repeated = [1, 2].repeat(3);
    assert_eq!(repeated, vec![1, 2, 1, 2, 1, 2]);
}

/// Concatenate slices.
///
/// Joins multiple slices into a single Vec.
pub fn concat_slices() {
    let parts = [&[1, 2][..], &[3, 4][..]];
    let concatenated = parts.concat();
    assert_eq!(concatenated, vec![1, 2, 3, 4]);
}

/// Join slice elements with separator.
///
/// Concatenates elements with a separator between them.
/// Works with types that implement `Display`.
pub fn join_elements() {
    let parts = ["Hello", "World"];
    assert_eq!(parts.join(" "), "Hello World");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_len() {
        slice_len();
    }

    #[test]
    fn test_is_empty_check() {
        is_empty_check();
    }

    #[test]
    fn test_get_element() {
        get_element();
    }

    #[test]
    fn test_first_last() {
        first_last();
    }

    #[test]
    fn test_split_at() {
        split_at();
    }

    #[test]
    fn test_binary_search() {
        binary_search();
    }

    #[test]
    fn test_contains_element() {
        contains_element();
    }

    #[test]
    fn test_repeat_elements() {
        repeat_elements();
    }

    #[test]
    fn test_concat_slices() {
        concat_slices();
    }

    #[test]
    fn test_join_elements() {
        join_elements();
    }
}
