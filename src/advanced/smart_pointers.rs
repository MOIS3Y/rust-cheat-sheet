//! Smart pointer operations.
//!
//! Demonstrates common methods for working with smart pointers:
//! `Box<T>`, `Rc<T>`, `Arc<T>`, and `RefCell<T>`.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// Heap allocation with Box.
///
/// `Box<T>` allocates values on the heap.
/// Dereference with `*` to access the value.
pub fn box_heap() {
    let boxed = Box::new(42);
    assert_eq!(*boxed, 42);
}

/// Reference counting with Rc.
///
/// `Rc<T>` enables shared ownership in single-threaded contexts.
/// Use `Rc::clone` to increment the reference count.
pub fn rc_clone() {
    let rc1 = Rc::new(42);
    let rc2 = Rc::clone(&rc1);
    assert_eq!(Rc::strong_count(&rc1), 2);
    assert_eq!(*rc2, 42);
}

/// Shared mutable state with Rc and RefCell.
///
/// Combines `Rc<T>` for shared ownership with
/// `RefCell<T>` for interior mutability.
pub fn rc_refcell() {
    let shared = Rc::new(RefCell::new(42));
    let shared2 = Rc::clone(&shared);

    *shared.borrow_mut() += 10;
    assert_eq!(*shared.borrow(), 52);
    assert_eq!(*shared2.borrow(), 52);
}

/// Thread-safe reference counting with Arc.
///
/// `Arc<T>` is the thread-safe version of `Rc<T>`.
/// Use for sharing data across threads.
pub fn arc_thread_safe() {
    let arc = Arc::new(42);
    let _arc_clone = Arc::clone(&arc);
    assert_eq!(*arc, 42);
}

/// Weak references to prevent cycles.
///
/// `Weak<T>` holds a non-owning reference.
/// Use `upgrade()` to get an `Option<Rc<T>>`.
pub fn weak_reference() {
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);

    assert_eq!(weak.upgrade(), Some(rc.clone()));

    drop(rc);
    assert_eq!(weak.upgrade(), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_heap() {
        box_heap();
    }

    #[test]
    fn test_rc_clone() {
        rc_clone();
    }

    #[test]
    fn test_rc_refcell() {
        rc_refcell();
    }

    #[test]
    fn test_arc_thread_safe() {
        arc_thread_safe();
    }

    #[test]
    fn test_weak_reference() {
        weak_reference();
    }
}
