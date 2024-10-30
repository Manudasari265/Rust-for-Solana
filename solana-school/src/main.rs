// The task is to add code everywhere where you find // TODO.
// You have to finish the implementation of the methods of Calculator, Rectangle and Circle structs.
// The code does not compile yet, because some of the required methods are missing - you have to add them.
// Once you complete the TODOs, make sure that you delete all `todo!()` macros and
// you can try to run the tests using `cargo test` command and start debugging ;-)

#![allow(dead_code)]

use std::fmt;
mod tests;

// ------------------------------------------------------------------------------------------------
// Traits
//
/// Trait Shape defines basic shared methods for various shapes.
/// More information can be found here: https://doc.rust-lang.org/book/ch10-02-traits.html
pub trait Shape {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}
// ------------------------------------------------------------------------------------------------
// Structs
//
/// The struct Calculator stores two operands, can perform simple math operations,
/// and is also resilient to overflow and underflow.
pub struct Calculator {
    pub x: i64,
    pub y: i64,
}
/// The struct Rectangle stores both sides and can compute area and
/// circumference for itself.
pub struct Rectangle {
    a: f64,
    b: f64,
}
/// The struct Circle stores radius and can compute area and
/// circumference for itself.
pub struct Circle {
    r: f64,
}
// ------------------------------------------------------------------------------------------------
// Non-Trait implementations for Structs
//
impl Calculator {
    /// Constructor
    pub fn new(arg1: &i64, arg2: &i64) -> Self {
        Self { x: *arg1, y: *arg2 }
    }
    /// Addition with Underflow/Overflow Resilience
    pub fn addition(&self) -> Option<i64> {
        // TODO Implement addition of calculator's x and y values.
        // Notice the Option<i64> return type: https://doc.rust-lang.org/std/option/index.html
        // Return None in case of under/overflow.
        // Try to check the documentation of i64 type if you can
        // find some useful methods: https://doc.rust-lang.org/std/primitive.i64.html#implementations
        todo!()
    }
    /// Subtraction with Underflow/Overflow Resilience
    pub fn subtraction(&self) -> Option<i64> {
        // TODO Implement subtraction of calculator's x and y values.
        // Return None in case of under/overflow.
        todo!()
    }
    /// Multiplication with Underflow/Overflow Resilience
    pub fn multiplication(&self) -> Option<i64> {
        // TODO Implement multiplication of calculator's x and y values.
        // Return None in case of under/overflow.
        todo!()
    }
    /// Division with Underflow/Overflow Resilience
    pub fn division(&self) -> Option<i64> {
        // TODO Implement division of calculator's x and y values.
        // Return None in case of division by zero.
        todo!()
    }
}