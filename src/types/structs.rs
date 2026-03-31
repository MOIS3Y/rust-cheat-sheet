//! Struct definitions and operations.
//!
//! Demonstrates different struct types and their methods.

#![allow(clippy::all)]
#![allow(dead_code)]

/// Classic struct with named fields.
///
/// Regular structs have named fields accessed with dot notation.
pub fn classic_struct() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    assert_eq!(user.username, "alice");
    assert!(user.active);
}

#[derive(Debug, PartialEq)]
struct User {
    username: String,
    email: String,
    active: bool,
}

/// Tuple struct.
///
/// Tuple structs have unnamed fields accessed by index.
pub fn tuple_struct() {
    let color = Color(255, 0, 0);
    let point = Point(10, 20, 30);

    assert_eq!(color.0, 255);
    assert_eq!(point.2, 30);
}

#[derive(Debug, PartialEq)]
struct Color(u8, u8, u8);

#[derive(Debug, PartialEq)]
struct Point(i32, i32, i32);

/// Unit struct.
///
/// Unit structs have no fields, useful for traits and markers.
pub fn unit_struct() {
    let marker = Marker;
    assert!(is_marker(marker));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Marker;

fn is_marker(_m: Marker) -> bool {
    true
}

/// Struct update syntax.
///
/// Create a new struct using values from an existing one.
pub fn struct_update() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    let user2 = User {
        username: String::from("bob"),
        ..user1
    };

    assert_eq!(user2.username, "bob");
    assert_eq!(user2.email, "alice@example.com");
}

/// Struct destructuring.
///
/// Extract fields into variables.
pub fn struct_destructure() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    let User { username, email, .. } = user;
    assert_eq!(username, "alice");
    assert_eq!(email, "alice@example.com");
}

/// Mutable struct.
///
/// Struct fields can be mutated if the struct is mutable.
pub fn mutable_struct() {
    let mut user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    user.username = String::from("alice_updated");
    assert_eq!(user.username, "alice_updated");
}

/// Struct with methods.
///
/// Implement blocks add methods to structs.
pub fn struct_methods() {
    let mut player = Player {
        name: String::from("Hero"),
        health: 100,
        mana: 50,
        stamina: 75,
    };

    assert_eq!(player.health, 100);
    player.take_damage(20);
    assert_eq!(player.health, 80);
    player.restore_mana(10);
    assert_eq!(player.mana, 60);
}

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    mana: u32,
    stamina: u32,
}

impl Player {
    fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }

    fn restore_mana(&mut self, amount: u32) {
        self.mana = (self.mana + amount).min(100);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            health: 100,
            mana: 50,
            stamina: 75,
        }
    }
}

/// Associated functions (constructors).
///
/// Associated functions don't take self as parameter.
pub fn associated_function() {
    let player = Player::new("Warrior");
    assert_eq!(player.name, "Warrior");
    assert_eq!(player.health, 100);
    assert!(player.is_alive());
}

/// Struct with standard trait implementations.
///
/// Common traits: Debug, Clone, PartialEq, Default.
pub fn struct_with_traits() {
    let player1 = GameCharacter::new("Hero", 100);
    let player2 = player1.clone();
    let player3 = GameCharacter::new("Villain", 80);

    // PartialEq
    assert_eq!(player1, player2);
    assert_ne!(player1, player3);

    // Debug
    let debug_str = format!("{:?}", player1);
    assert!(debug_str.contains("Hero"));

    // Default
    let default = GameCharacter::default();
    assert_eq!(default.health, 100);
}

#[derive(Debug, Clone, PartialEq)]
struct GameCharacter {
    name: String,
    health: u32,
}

impl GameCharacter {
    fn new(name: &str, health: u32) -> Self {
        Self {
            name: String::from(name),
            health,
        }
    }
}

impl Default for GameCharacter {
    fn default() -> Self {
        Self {
            name: String::from("Unknown"),
            health: 100,
        }
    }
}

/// Struct with Display trait.
///
/// Display provides user-friendly string representation.
pub fn struct_with_display() {
    let player = GameCharacter::new("Hero", 100);
    let display_str = format!("{}", player);
    assert_eq!(display_str, "Hero (100 HP)");
}

use std::fmt;

impl fmt::Display for GameCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} HP)", self.name, self.health)
    }
}

/// Struct with multiple impl blocks.
///
/// A struct can have multiple impl blocks for organization.
pub fn multiple_impl_blocks() {
    let mut car = Car {
        brand: String::from("Toyota"),
        speed: 0,
        fuel: 50,
    };

    car.accelerate(30);
    assert_eq!(car.speed, 30);

    car.refuel(20);
    assert_eq!(car.fuel, 70);
}

/// Struct field init shorthand.
///
/// When variable names match field names,
/// you can use shorthand syntax.
pub fn struct_init_shorthand() {
    let username = String::from("alice");
    let email = String::from("alice@example.com");
    let active = true;

    // Shorthand: User { username, email, active }
    let user = User { username, email, active };

    assert_eq!(user.username, "alice");
    assert!(user.active);
}

/// Newtype pattern.
///
/// Wrap a type to add type safety and domain meaning.
/// Zero runtime overhead — erased at compile time.
pub fn newtype_pattern() {
    let user_id = UserId(42);
    let email = Email(String::from("test@example.com"));

    // Type safety: can't accidentally use UserId where Email is expected
    assert_eq!(user_id.0, 42);
    assert_eq!(email.0, "test@example.com");

    // Different types, can't mix them
    // let wrong: UserId = email;  // Won't compile!
}

#[derive(Debug, Clone, PartialEq)]
struct UserId(u32);

#[derive(Debug, Clone, PartialEq)]
struct Email(String);

#[derive(Debug)]
struct Car {
    brand: String,
    speed: u32,
    fuel: u32,
}

impl Car {
    fn accelerate(&mut self, amount: u32) {
        self.speed += amount;
    }

    fn brake(&mut self, amount: u32) {
        self.speed = self.speed.saturating_sub(amount);
    }
}

impl Car {
    fn refuel(&mut self, amount: u32) {
        self.fuel += amount;
    }

    fn drive(&mut self, distance: u32) {
        let consumption = distance / 10;
        self.fuel = self.fuel.saturating_sub(consumption);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_struct() {
        classic_struct();
    }

    #[test]
    fn test_tuple_struct() {
        tuple_struct();
    }

    #[test]
    fn test_unit_struct() {
        unit_struct();
    }

    #[test]
    fn test_struct_update() {
        struct_update();
    }

    #[test]
    fn test_struct_destructure() {
        struct_destructure();
    }

    #[test]
    fn test_mutable_struct() {
        mutable_struct();
    }

    #[test]
    fn test_struct_methods() {
        struct_methods();
    }

    #[test]
    fn test_associated_function() {
        associated_function();
    }

    #[test]
    fn test_struct_with_traits() {
        struct_with_traits();
    }

    #[test]
    fn test_struct_with_display() {
        struct_with_display();
    }

    #[test]
    fn test_multiple_impl_blocks() {
        multiple_impl_blocks();
    }

    #[test]
    fn test_struct_init_shorthand() {
        struct_init_shorthand();
    }

    #[test]
    fn test_newtype_pattern() {
        newtype_pattern();
    }
}
