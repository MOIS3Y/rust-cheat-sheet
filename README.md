# Rust Cheat Sheet

Hands-on Rust reference covering syntax, types, methods, and idioms.

## Purpose

This project helps you quickly recall basic Rust methods and idiomatic
approaches without searching through documentation or making AI queries.

When learning a new language, you often forget available methods
and standard patterns — this cheat sheet provides quick reference examples.

Examples focus on demonstrating individual methods rather than complex logic.
This reduces cognitive load and lets you concentrate on language concepts.

## Project Structure

```
rust-cheat-sheet/
├── src/
│   ├── main.rs              # Entry point — runs all examples
│   ├── lib.rs               # Library root — exports modules
│   │
│   ├── basics/              # Basic types and operations
│   │   ├── options.rs
│   │   ├── results.rs
│   │   ├── strings.rs
│   │   ├── numbers.rs
│   │   ├── arrays_slices.rs
│   │   ├── documentation.rs
│   │   ├── functions.rs
│   │   ├── conditionals.rs
│   │   └── loops.rs
│   │
│   ├── collections/         # Collection types
│   │   ├── vecs.rs
│   │   ├── hashmaps.rs
│   │   └── iterators.rs
│   │
│   ├── error_handling/      # Error handling
│   │   ├── question_mark.rs
│   │   ├── from_trait.rs
│   │   ├── custom_error.rs
│   │   └── box_dyn_error.rs
│   │
│   ├── advanced/            # Advanced features
│   │   ├── smart_pointers.rs
│   │   └── traits.rs
│   │
│   ├── std_lib/             # Standard library
│   │   ├── time.rs
│   │   └── fs.rs
│   │
│   ├── ownership/           # Ownership system
│   │   ├── move_semantics.rs
│   │   ├── borrowing.rs
│   │   └── lifetimes.rs
│   │
│   └── types/               # Custom types
│       ├── structs.rs
│       ├── enums.rs
│       ├── pattern_matching.rs
│       └── custom_traits.rs
│
├── Cargo.toml
├── flake.nix
└── README.md
```

## Usage

### Browse source code

The primary way to use this cheat sheet is reading the source code.
Each module contains self-contained examples with documentation:

```bash
# Open a specific module in your editor
nvim src/basics/options.rs
zeditor src/collections/vecs.rs
```

### Run all examples

```bash
cargo run
```

### Build

```bash
cargo build
cargo build --release
```

### Run tests

```bash
cargo test
```

### Generate documentation

```bash
cargo doc --open
```

### Learn by doing

The best way to remember is to repeat the examples yourself:

1. Copy an example to your own project
2. Try different input values
3. Combine multiple methods together
4. Apply to your real-world code

Active practice beats passive reading.

## Development

### Using Nix

This project includes a Nix flake for reproducible development environment:

```bash
nix develop
```

### Requirements

- Rust 1.85+ (edition 2024)
- Nix (optional, for flake-based development)

## Module Organization

Each module contains:
- Standalone example functions demonstrating single methods
- Module-level documentation (`//!`)
- Function documentation (`///`)
- Unit tests in `#[cfg(test)]` blocks
- Doc tests where applicable

## Topics Covered

### Basics
- Option, Result — error handling types
- String, &str — text operations
- Numbers — arithmetic, overflow handling
- Arrays, Slices — fixed and dynamic views
- Functions — declaration, closures
- Conditionals — if, match, if let
- Loops — loop, while, for

### Collections
- Vec — dynamic arrays
- HashMap — hash tables
- Iterators — lazy sequences

### Error Handling
- Question mark — error propagation with `?`
- From trait — automatic error conversion
- Custom errors — defining error types
- Box<dyn Error> — type-erased errors

### Ownership
- Move semantics — ownership transfer
- Borrowing — references without ownership
- Lifetimes — reference validity

### Types
- Structs — custom data types
- Enums — sum types with variants
- Pattern matching — destructuring syntax
- Traits — shared behavior

### Advanced
- Smart pointers — Box, Rc, Arc, RefCell
- Standard traits — Clone, Default, From/Into

### Standard Library
- Time — Duration, Instant, SystemTime
- Filesystem — file and directory operations

## License

MIT
