//! Enum definitions and operations.
//!
//! Demonstrates enum types and their usage patterns.

#![allow(clippy::all)]
#![allow(dead_code)]

/// Simple enum with variants.
///
/// Enums define a type that can be one of several variants.
pub fn simple_enum() {
    let direction = Direction::North;
    let another = Direction::South;

    assert_ne!(direction, another);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Enum with data.
///
/// Each variant can hold different types of data.
pub fn enum_with_data() {
    let message = Message::Write(String::from("hello"));
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };

    match message {
        Message::Write(text) => assert_eq!(text, "hello"),
        _ => panic!("Expected Write variant"),
    }

    match move_msg {
        Message::Move { x, y } => {
            assert_eq!(x, 10);
            assert_eq!(y, 20);
        }
        _ => panic!("Expected Move variant"),
    }

    match quit {
        Message::Quit => (),
        _ => panic!("Expected Quit variant"),
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// Option enum.
///
/// Option<T> represents an optional value.
pub fn option_enum() {
    let some: Option<i32> = Some(42);
    let none: Option<i32> = None;

    match some {
        Some(value) => assert_eq!(value, 42),
        None => panic!("Expected Some"),
    }

    assert!(none.is_none());
}

/// Result enum.
///
/// Result<T, E> represents a value or an error.
pub fn result_enum() {
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("error");

    match ok {
        Ok(value) => assert_eq!(value, 42),
        Err(_) => panic!("Expected Ok"),
    }

    match err {
        Err(e) => assert_eq!(e, "error"),
        Ok(_) => panic!("Expected Err"),
    }
}

/// Enum with methods.
///
/// Enums can have impl blocks with methods.
pub fn enum_methods() {
    let light = TrafficLight::Red;
    assert_eq!(light.duration(), 60);

    let light = TrafficLight::Yellow;
    assert_eq!(light.duration(), 5);

    let light = TrafficLight::Green;
    assert_eq!(light.duration(), 30);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }

    fn next(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

/// Enum with utility methods.
///
/// Using the next() method on enum.
pub fn enum_next() {
    let mut light = TrafficLight::Red;

    light = light.next();
    assert_eq!(light, TrafficLight::Green);

    light = light.next();
    assert_eq!(light, TrafficLight::Yellow);
}

/// Nullable enum pattern.
///
/// Custom nullable type similar to Option.
pub fn nullable_pattern() {
    let some = Nullable::Some(42);
    let none: Nullable<i32> = Nullable::None;

    match some {
        Nullable::Some(v) => assert_eq!(v, 42),
        Nullable::None => panic!("Expected Some"),
    }

    assert!(matches!(none, Nullable::None));
}

#[derive(Debug, PartialEq)]
enum Nullable<T> {
    Some(T),
    None,
}

/// Enum with standard trait implementations.
///
/// Common traits: Debug, Clone, PartialEq, Copy.
pub fn enum_with_traits() {
    let state1 = GameState::Playing;
    let state2 = GameState::Playing;
    let state3 = GameState::Paused;

    // PartialEq
    assert_eq!(state1, state2);
    assert_ne!(state1, state3);

    // Clone (via Copy)
    let cloned = state1;
    assert_eq!(state1, cloned);

    // Debug
    let debug_str = format!("{:?}", state1);
    assert_eq!(debug_str, "Playing");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Playing,
    Paused,
    GameOver,
}

/// Enum with Display trait.
///
/// Display provides user-friendly string representation.
pub fn enum_with_display() {
    let state = GameState::Playing;
    let display_str = format!("{}", state);
    assert_eq!(display_str, "Playing");

    let state = GameState::Paused;
    let display_str = format!("{}", state);
    assert_eq!(display_str, "Paused");
}

use std::fmt;

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Playing => write!(f, "Playing"),
            GameState::Paused => write!(f, "Paused"),
            GameState::GameOver => write!(f, "Game Over"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_enum() {
        simple_enum();
    }

    #[test]
    fn test_enum_with_data() {
        enum_with_data();
    }

    #[test]
    fn test_option_enum() {
        option_enum();
    }

    #[test]
    fn test_result_enum() {
        result_enum();
    }

    #[test]
    fn test_enum_methods() {
        enum_methods();
    }

    #[test]
    fn test_enum_next() {
        enum_next();
    }

    #[test]
    fn test_nullable_pattern() {
        nullable_pattern();
    }

    #[test]
    fn test_enum_with_traits() {
        enum_with_traits();
    }

    #[test]
    fn test_enum_with_display() {
        enum_with_display();
    }
}
