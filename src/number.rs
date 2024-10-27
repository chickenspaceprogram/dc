use std::fmt::{Display, Formatter};
use std::fmt;
use std::error::Error;

pub trait Arithmetic: Clone {
    /// Adds `num` to the current number
    fn add(&mut self, num: Self) -> Result<(), MathError>;

    /// Subtracts `num` from the current number
    fn subtract(&mut self, num: Self) -> Result<(), MathError>;

    /// Multiplies the current number by `num``.
    fn multiply(&mut self, num: Self) -> Result<(), MathError>;

    /// Divides the current number by `num``.
    fn divide(&mut self, num: Self) -> Result<(), MathError>;

    /// Returns the current number, modulo `num``.
    fn modulo(&mut self, num: Self) -> Result<(), MathError>;

    /// Raises the current number to the power `num`.
    fn pow(&mut self, num: Self) -> Result<(), MathError>;

    /// Returns the square root of the number.
    fn sqrt(&mut self) -> Result<(), MathError>;

    fn print(&self);
}

#[derive(Debug, Clone)]
pub struct MathError;

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "An arithmetic error occurred.")
    }
}

impl Error for MathError{}

/// Can be any valid operator.
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Pow,
    Sqrt,
}

/// Defines all the different possible control sequences.
/// Control sequences do things like clearing the stack, printing numbers on the stack, etc.
pub enum Control {

    /// Prints the first item on the stack
    PrintFirst,

    /// Prints every item on the stack.
    PrintAll,

    /// Pops the top item off the stack.
    Pop,

    /// Clears every item from the stack.
    Clear,

    /// Duplicates the top item on the stack.
    Duplicate,

    /// Swaps the top two items on the stack.
    Swap,
}

/// Wraps all the different possible things a token could be into an enum.
pub enum Token<T: Arithmetic> {
    Op(Operator),
    Num(T),
    Ctrl(Control),
}

