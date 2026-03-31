//! Error handling patterns and best practices.
//!
//! This module covers:
//! - `?` operator for error propagation
//! - `From` trait for error conversion
//! - Custom error types
//! - `Box<dyn Error>` for type-erased errors

pub mod box_dyn_error;
pub mod custom_error;
pub mod from_trait;
pub mod question_mark;
