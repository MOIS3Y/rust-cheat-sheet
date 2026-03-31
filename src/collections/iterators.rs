//! Iterator operations.
//!
//! Demonstrates common methods for working with iterators,
//! the lazy sequences that power Rust's functional style.

#![allow(clippy::all)]

/// Collect iterator into Vec.
///
/// Consumes the iterator and collects elements into a Vec.
pub fn collect_vec() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}

/// Transform elements with map.
///
/// Applies a function to each element, returning a new iterator.
pub fn map_transform() {
    let numbers = vec![1, 2, 3];
    let strings: Vec<String> =
        numbers.iter().map(|x| x.to_string()).collect();
    assert_eq!(strings[0], "1");
}

/// Filter elements by predicate.
///
/// Returns only elements that satisfy the predicate.
pub fn filter_even() {
    let numbers = vec![1, 2, 3, 4, 5];
    let evens: Vec<_> =
        numbers.iter().filter(|&&x| x % 2 == 0).collect();
    assert_eq!(evens, vec![&2, &4]);
}

/// Filter and transform with filter_map.
///
/// Combines filtering and mapping in a single pass.
pub fn filter_map_parse() {
    let parsed: Vec<i32> = ["1", "2", "three", "4"]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    assert_eq!(parsed, vec![1, 2, 4]);
}

/// Sum with fold.
///
/// Accumulates values starting from an initial value.
pub fn fold_sum() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    assert_eq!(sum, 15);
}

/// Reduce with binary operation.
///
/// Combines elements using a binary function, no initial value.
pub fn reduce_product() {
    let numbers = vec![1, 2, 3, 4];
    let product: Option<i32> =
        numbers.iter().copied().reduce(|acc, x| acc * x);
    assert_eq!(product, Some(24));
}

/// Check if any element matches.
///
/// Returns `true` if at least one element satisfies the predicate.
pub fn any_even() {
    let numbers = vec![1, 3, 5, 6];
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    assert!(has_even);
}

/// Check if all elements match.
///
/// Returns `true` if all elements satisfy the predicate.
pub fn all_positive() {
    let numbers = vec![1, 2, 3, 4, 5];
    let all_positive = numbers.iter().all(|&x| x > 0);
    assert!(all_positive);
}

/// Find first matching element.
///
/// Returns `Option<&T>` with the first match.
pub fn find_first() {
    let numbers = vec![1, 2, 3, 4, 5];
    let found = numbers.iter().find(|&&x| x > 3);
    assert_eq!(found, Some(&4));
}

/// Add indices with enumerate.
///
/// Returns an iterator of (index, value) pairs.
pub fn enumerate() {
    let numbers = vec![1, 2, 3];
    let indexed: Vec<(usize, &i32)> =
        numbers.iter().enumerate().collect();
    assert_eq!(indexed[0], (0, &1));
}

/// Limit with take and skip.
///
/// `take(n)` limits to first n elements,
/// `skip(n)` skips first n elements.
pub fn take_skip() {
    let numbers = vec![1, 2, 3, 4, 5];
    let first_two: Vec<_> = numbers.iter().take(2).collect();
    assert_eq!(first_two, vec![&1, &2]);

    let after_two: Vec<_> = numbers.iter().skip(2).collect();
    assert_eq!(after_two, vec![&3, &4, &5]);
}

/// Combine iterators with zip.
///
/// Pairs elements from two iterators.
pub fn zip_iterators() {
    let letters = ['a', 'b', 'c'];
    let numbers = vec![1, 2, 3];
    let zipped: Vec<_> = numbers.iter().zip(letters.iter()).collect();
    assert_eq!(zipped, vec![(&1, &'a'), (&2, &'b'), (&3, &'c')]);
}

/// Chain iterators together.
///
/// Concatenates two iterators sequentially.
pub fn chain_iterators() {
    let first = [1, 2];
    let second = [3, 4];
    let chained: Vec<_> =
        first.iter().chain(second.iter()).collect();
    assert_eq!(chained, vec![&1, &2, &3, &4]);
}

/// Partition into two collections.
///
/// Splits elements into two collections based on predicate.
pub fn partition() {
    let (evens, odds): (Vec<i32>, Vec<i32>) =
        (1..=5).partition(|&x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4]);
    assert_eq!(odds, vec![1, 3, 5]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_vec() {
        collect_vec();
    }

    #[test]
    fn test_map_transform() {
        map_transform();
    }

    #[test]
    fn test_filter_even() {
        filter_even();
    }

    #[test]
    fn test_filter_map_parse() {
        filter_map_parse();
    }

    #[test]
    fn test_fold_sum() {
        fold_sum();
    }

    #[test]
    fn test_reduce_product() {
        reduce_product();
    }

    #[test]
    fn test_any_even() {
        any_even();
    }

    #[test]
    fn test_all_positive() {
        all_positive();
    }

    #[test]
    fn test_find_first() {
        find_first();
    }

    #[test]
    fn test_enumerate() {
        enumerate();
    }

    #[test]
    fn test_take_skip() {
        take_skip();
    }

    #[test]
    fn test_zip_iterators() {
        zip_iterators();
    }

    #[test]
    fn test_chain_iterators() {
        chain_iterators();
    }

    #[test]
    fn test_partition() {
        partition();
    }
}
