/// Defines traits and enums necessary to create a general number type that `dc` can do math with.

use std::fmt::{Display, Formatter};
use std::fmt;
use std::error::Error;

/// A trait describing all of the methods required by a basic implementation of `dc`.
/// This is suitably general that it should be possible to make implementations of `dc` for whatever is desired, from emulating TI graphing calculators, to emulating the original Unix `dc`, to even doing matrix math.
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

    /// Prints the number.
    fn print(&self);
}

#[derive(Debug, Clone)]
/// An error for when some sort of mathematical error has occurred.
/// For example, if an overflow occurs, or a specific operation is not permitted (like division by 0).
pub struct MathError;

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "An arithmetic error occurred.")
    }
}

impl Error for MathError{}

/// An enum containing all of the valid operations.
#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Pow,
    Sqrt,
}

/// An enum defining all the different possible control sequences.
/// Control sequences do things like clearing the stack, printing numbers on the stack, etc.
#[derive(Clone, Debug)]
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

/// An enum wrapping all the different possible things a token could be.
#[derive(Clone, Debug)]
pub enum Token<T: Arithmetic> {
    Op(Operator),
    Num(T),
    Ctrl(Control),
}

