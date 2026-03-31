//! HashMap<K, V> operations.
//!
//! Demonstrates common methods for working with `HashMap<K, V>`,
//! the hash table type in Rust's standard library.

#![allow(clippy::all)]

use std::collections::HashMap;

/// Insert key-value pair.
///
/// Adds or updates a key-value pair in the map.
pub fn insert_pair() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("rust", 1);
    map.insert("go", 2);
    assert_eq!(map.len(), 2);
}

/// Get value by key.
///
/// Returns `Option<&V>` - `None` if key not found.
pub fn get_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("rust", 1);
    assert_eq!(map.get("rust"), Some(&1));
    assert_eq!(map.get("javascript"), None);
}

/// Check if key exists.
///
/// Returns `true` if the map contains the key.
pub fn contains_key() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("rust", 1);
    assert!(map.contains_key("rust"));
    assert!(!map.contains_key("javascript"));
}

/// Remove key-value pair.
///
/// Removes and returns the value, or `None` if key not found.
pub fn remove_key() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("rust", 1);
    let removed = map.remove("rust");
    assert_eq!(removed, Some(1));
    assert!(!map.contains_key("rust"));
}

/// Entry API for conditional insert.
///
/// `entry().or_insert()` inserts if key missing,
/// returns mutable reference to value.
pub fn entry_insert() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    *map.entry("rust").or_insert(0) += 1;
    assert_eq!(map.get("rust"), Some(&1));
}

/// Lazy default with or_insert_with.
///
/// Computes default value only if key is missing.
pub fn or_insert_with() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("new_key").or_insert_with(|| 42);
    assert_eq!(map.get("new_key"), Some(&42));
}

/// Insert default value with or_default.
///
/// Inserts `Default::default()` if key is missing.
pub fn or_default() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("another").or_default();
    assert_eq!(map.get("another"), Some(&0));
}

/// Modify existing value with and_modify.
///
/// Applies a function to the value if key exists.
pub fn and_modify() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("rust", 1);
    map.entry("rust").and_modify(|v| *v += 10).or_insert(0);
    assert_eq!(map.get("rust"), Some(&11));
}

/// Iterate over key-value pairs.
///
/// Returns an iterator over `(&K, &V)` pairs.
pub fn iter_pairs() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let count = map.iter().count();
    assert_eq!(count, 2);
}

/// Get keys and values iterators.
///
/// `keys()` and `values()` return iterators over references.
pub fn keys_values() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys.len(), 2);

    let values: Vec<_> = map.values().collect();
    assert_eq!(values.len(), 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_pair() {
        insert_pair();
    }

    #[test]
    fn test_get_value() {
        get_value();
    }

    #[test]
    fn test_contains_key() {
        contains_key();
    }

    #[test]
    fn test_remove_key() {
        remove_key();
    }

    #[test]
    fn test_entry_insert() {
        entry_insert();
    }

    #[test]
    fn test_or_insert_with() {
        or_insert_with();
    }

    #[test]
    fn test_or_default() {
        or_default();
    }

    #[test]
    fn test_and_modify() {
        and_modify();
    }

    #[test]
    fn test_iter_pairs() {
        iter_pairs();
    }

    #[test]
    fn test_keys_values() {
        keys_values();
    }
}
