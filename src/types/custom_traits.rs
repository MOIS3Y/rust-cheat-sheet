//! Custom trait definitions and implementations.
//!
//! Demonstrates how to define and implement custom traits.

#![allow(clippy::all)]
#![allow(dead_code)]

/// Simple trait definition.
///
/// Traits define shared behavior that types can implement.
pub fn simple_trait() {
    let dog = Dog {
        name: String::from("Buddy"),
    };

    let cat = Cat {
        name: String::from("Whiskers"),
    };

    assert_eq!(dog.speak(), "Woof!");
    assert_eq!(cat.speak(), "Meow!");
}

trait Speak {
    fn speak(&self) -> &str;
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
}

struct Cat {
    name: String,
}

impl Speak for Cat {
    fn speak(&self) -> &str {
        "Meow!"
    }
}

/// Trait with default implementation.
///
/// Traits can provide default method implementations.
pub fn trait_with_default() {
    let bird = Bird {
        name: String::from("Tweety"),
    };

    // Uses default implementation from Animal trait
    assert_eq!(Animal::move_action(&bird), "Flying");

    // Custom implementation
    assert_eq!(Animal::speak(&bird), "Chirp!");
}

struct Bird {
    name: String,
}

trait Animal {
    fn speak(&self) -> &str;

    fn move_action(&self) -> &str {
        "Moving"
    }
}

impl Animal for Bird {
    fn speak(&self) -> &str {
        "Chirp!"
    }

    fn move_action(&self) -> &str {
        "Flying"
    }
}

impl Speak for Bird {
    fn speak(&self) -> &str {
        "Chirp!"
    }
}

/// Trait bounds on functions.
///
/// Functions can require types to implement traits.
pub fn trait_bounds() {
    let dog = Dog {
        name: String::from("Buddy"),
    };

    let result = make_sound(&dog);
    assert_eq!(result, "Animal says: Woof!");
}

fn make_sound<T: Speak>(animal: &T) -> String {
    format!("Animal says: {}", animal.speak())
}

/// Multiple trait bounds.
///
/// Functions can require multiple traits.
pub fn multiple_trait_bounds() {
    let creature = Creature {
        name: String::from("Griffin"),
    };

    describe(&creature);
}

#[derive(Debug)]
struct Creature {
    name: String,
}

trait Named {
    fn name(&self) -> &str;
}

impl Named for Creature {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Speak for Creature {
    fn speak(&self) -> &str {
        "Roar!"
    }
}

fn describe<T: Named + Speak + std::fmt::Display>(creature: &T) {
    let _ = format!(
        "{} is a {} who says {}",
        creature.name(),
        creature,
        creature.speak()
    );
}

impl std::fmt::Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Creature({})", self.name)
    }
}

/// Trait bound with where clause.
///
/// Complex bounds use where clauses.
/// `Box` is used to store different types as trait objects.
pub fn where_clause() {
    let items = vec![
        Box::new(Dog {
            name: String::from("Buddy"),
        }) as Box<dyn Speak>,
        Box::new(Cat {
            name: String::from("Whiskers"),
        }) as Box<dyn Speak>,
    ];

    let sounds: Vec<&str> = items.iter().map(|a| a.speak()).collect();
    assert_eq!(sounds, vec!["Woof!", "Meow!"]);
}

/// Trait object (dynamic dispatch).
///
/// Trait objects allow storing different types uniformly.
/// `Box` is used because trait objects have dynamic size
/// (different types implementing the trait may have different sizes).
pub fn trait_objects() {
    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog {
            name: String::from("Buddy"),
        }),
        Box::new(Cat {
            name: String::from("Whiskers"),
        }),
        Box::new(Bird {
            name: String::from("Tweety"),
        }),
    ];

    let sounds: Vec<&str> = animals.iter().map(|a| a.speak()).collect();
    assert_eq!(sounds, vec!["Woof!", "Meow!", "Chirp!"]);
}

/// Trait with associated types.
///
/// Associated types are placeholders used in trait methods.
pub fn associated_types() {
    let container = DataContainer {
        data: vec![1, 2, 3],
    };

    assert_eq!(container.first(), Some(&1));
    assert_eq!(container.len(), 3);
}

trait Container {
    type Item;

    fn first(&self) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

struct DataContainer<T> {
    data: Vec<T>,
}

impl<T> Container for DataContainer<T> {
    type Item = T;

    fn first(&self) -> Option<&Self::Item> {
        self.data.first()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

/// Trait with generics and lifetimes.
///
/// Traits can have generic parameters and lifetimes.
pub fn trait_with_lifetime() {
    let text = String::from("Hello, World!");
    let printer = TextPrinter { text: &text };

    assert_eq!(printer.print(), "Hello, World!");
}

struct TextPrinter<'a> {
    text: &'a str,
}

trait Printer<'a> {
    fn print(&self) -> &'a str;
}

impl<'a> Printer<'a> for TextPrinter<'a> {
    fn print(&self) -> &'a str {
        self.text
    }
}

/// Trait inheritance (supertraits).
///
/// A trait can require another trait to be implemented first.
pub fn trait_inheritance() {
    let dog = PetDog {
        name: String::from("Buddy"),
        breed: String::from("Labrador"),
    };

    // Can use methods from both traits
    assert_eq!(dog.speak(), "Woof!");
    assert_eq!(dog.name(), "Buddy");
    assert_eq!(dog.description(), "Buddy says: Woof!");
}

#[derive(Debug)]
struct PetDog {
    name: String,
    breed: String,
}

// HasName is a supertrait of Pet
trait HasName {
    fn name(&self) -> &str;
}

trait Pet: HasName {
    fn speak(&self) -> &str;

    fn description(&self) -> String {
        format!("{} says: {}", self.name(), self.speak())
    }
}

impl HasName for PetDog {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pet for PetDog {
    fn speak(&self) -> &str {
        "Woof!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_trait() {
        simple_trait();
    }

    #[test]
    fn test_trait_with_default() {
        trait_with_default();
    }

    #[test]
    fn test_trait_bounds() {
        trait_bounds();
    }

    #[test]
    fn test_multiple_trait_bounds() {
        multiple_trait_bounds();
    }

    #[test]
    fn test_where_clause() {
        where_clause();
    }

    #[test]
    fn test_trait_objects() {
        trait_objects();
    }

    #[test]
    fn test_associated_types() {
        associated_types();
    }

    #[test]
    fn test_trait_with_lifetime() {
        trait_with_lifetime();
    }

    #[test]
    fn test_trait_inheritance() {
        trait_inheritance();
    }
}
