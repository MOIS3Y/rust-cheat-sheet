//! Move semantics operations.
//!
//! Demonstrates how ownership is transferred in Rust.

#![allow(clippy::all)]
#![allow(dead_code)]

/// Move ownership from one variable to another.
///
/// When assigning a non-Copy value, ownership moves
/// and the original variable becomes invalid.
pub fn move_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is no longer valid here.
    // The following code would not compile:
    // println!("{}", s1);  // error[E0382]: borrow of moved value

    assert_eq!(s2, "hello");
}

/// Move into function call.
///
/// Passing a value to a function transfers ownership.
/// The caller can't use the value after the call.
pub fn move_into_function() {
    let s = String::from("hello");
    take_ownership(s);

    // s is no longer valid here.
    // The following code would not compile:
    // println!("{}", s);  // error[E0382]: borrow of moved value
}

fn take_ownership(s: String) {
    assert_eq!(s, "hello");
}

/// Move out of function call.
///
/// Functions can return owned values, transferring
/// ownership to the caller.
pub fn move_out_of_function() {
    let s = give_ownership();
    assert_eq!(s, "hello");
}

fn give_ownership() -> String {
    String::from("hello")
}

/// Move and return ownership.
///
/// Functions can take ownership and return it back.
pub fn move_and_return() {
    let s1 = String::from("hello");
    let s2 = take_and_give_back(s1);

    // s1 is invalid, but s2 owns the value.
    // The following code would not compile:
    // println!("{}", s1);  // error[E0382]: borrow of moved value

    assert_eq!(s2, "hello");
}

fn take_and_give_back(s: String) -> String {
    s
}

/// Move with Copy types.
///
/// Copy types (like integers) are copied, not moved.
/// Both variables remain valid.
pub fn copy_types() {
    let x = 42;
    let y = x;
    // Both x and y are valid
    assert_eq!(x, 42);
    assert_eq!(y, 42);
}

/// Move with Clone trait.
///
/// Use .clone() for explicit deep copy of non-Copy types.
pub fn clone_instead_of_move() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // Both s1 and s2 are valid
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");
}

/// Move with struct containing owned data.
///
/// When a struct is moved, all its owned fields move too.
pub fn move_struct() {
    let player1 = Player {
        name: String::from("Hero"),
        health: 100,
    };

    let player2 = player1;
    // player1 is no longer valid

    assert_eq!(player2.name, "Hero");
    assert_eq!(player2.health, 100);
}

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
}

/// Move with enum containing owned data.
///
/// When an enum is moved, the active variant's data moves.
pub fn move_enum() {
    let msg1 = Message::Text(String::from("hello"));
    let msg2 = msg1;
    // msg1 is no longer valid

    match msg2 {
        Message::Text(text) => assert_eq!(text, "hello"),
        _ => panic!("Expected Text variant"),
    }
}

#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
}

/// Partial move from struct.
///
/// You can move individual fields while keeping the struct valid.
pub fn partial_move() {
    let player = Player {
        name: String::from("Hero"),
        health: 100,
    };

    let name = player.name;
    // player.name is moved, but player.health is still accessible

    assert_eq!(name, "Hero");
    assert_eq!(player.health, 100);
    // player itself is still usable (partially moved)
}

/// Move with nested types.
///
/// Nested owned types all move together.
pub fn move_nested() {
    let inventory = Inventory {
        items: vec![String::from("sword"), String::from("shield")],
        gold: 100,
    };

    let inventory2 = inventory;
    // inventory is no longer valid

    assert_eq!(inventory2.gold, 100);
    assert_eq!(inventory2.items.len(), 2);
}

#[derive(Debug)]
struct Inventory {
    items: Vec<String>,
    gold: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_ownership() {
        move_ownership();
    }

    #[test]
    fn test_move_into_function() {
        move_into_function();
    }

    #[test]
    fn test_move_out_of_function() {
        move_out_of_function();
    }

    #[test]
    fn test_move_and_return() {
        move_and_return();
    }

    #[test]
    fn test_copy_types() {
        copy_types();
    }

    #[test]
    fn test_clone_instead_of_move() {
        clone_instead_of_move();
    }

    #[test]
    fn test_move_struct() {
        move_struct();
    }

    #[test]
    fn test_move_enum() {
        move_enum();
    }

    #[test]
    fn test_partial_move() {
        partial_move();
    }

    #[test]
    fn test_move_nested() {
        move_nested();
    }
}
