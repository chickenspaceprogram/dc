pub trait Arithmetic: Clone {
    /// Adds `num` to the current number
    fn add<T: Arithmetic>(&mut self, num: T);

    /// Subtracts `num` from the current number
    fn subtract<T: Arithmetic>(&mut self, num: T);

    /// Multiplies the current number by `num``.
    fn multiply<T: Arithmetic>(&mut self, num: T);

    /// Divides the current number by `num``.
    fn divide<T: Arithmetic>(&mut self, num: T);

    /// Returns the current number, modulo `num``.
    fn modulo<T: Arithmetic>(&mut self, num: T);

    /// Raises the current number to the power `num`.
    fn pow<T: Arithmetic>(&mut self, num: T);

    /// Returns the square root of the number.
    fn sqrt(&mut self);

    /// Negates the number. 
    fn negate(&mut self);

    fn print(&self);
}

/// Can be any valid operator.
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Pow,
    Sqrt,
    Negate,
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

