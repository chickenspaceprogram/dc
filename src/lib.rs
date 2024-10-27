use crate::number::{Arithmetic, Token, Control, Operator};
use std::fmt;
use std::error::Error;

pub mod number;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid token, could not parse expression.")
    }
}

impl Error for ParseError {}

/// # calculator
/// 
/// ### Description:
/// 
/// Parses and evaluates the reverse-polish-notation calculations described in `operations`.
/// 
/// ### Parameters:
/// 
/// `operations` :  A `Vec` containing all of the tokens in the expression `dc` is to parse.
///                 A `Token` type and associated enums are defined in `number`.
/// 
/// `stack` : The stack that `calculator` is to start with. Usually, this can be a new, empty vector.
/// 
/// ### Returns:
/// A `Result` type containing either whatever was left on the stack or some error type.
pub fn calculator<T: Arithmetic>(operations: Vec<Token<T>>, mut stack: Vec<T>) -> Result<Vec<T>, Box<dyn Error>> {
    for i in operations {
        match i {
            Token::Op(operation) => handle_op(&mut stack, operation)?,
            Token::Ctrl(control) => handle_ctrl(&mut stack, control)?,
            Token::Num(num) => stack.push(num),
        };
    }
    return Ok(stack);
}

fn handle_op<T: Arithmetic>(stack: &mut Vec<T>, op: Operator) -> Result <(), Box<dyn Error>>{
    match op {
        Operator::Add => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.add(first)?;
            stack.push(second);
            return Ok(());
        }
        Operator::Subtract => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.subtract(first)?;
            stack.push(second);
            return Ok(());
        }
        Operator::Multiply => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.multiply(first)?;
            stack.push(second);
            return Ok(());
        }
        Operator::Divide => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.divide(first)?;
            stack.push(second);
            return Ok(());
        }
        Operator::Mod => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.modulo(first)?;
            stack.push(second);
            return Ok(());
        }
        Operator::Pow => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.pow(first)?;
            stack.push(second);
            return Ok(());
        }
        Operator::Sqrt => {
            let mut first = stack.pop().ok_or_else(|| ParseError)?;
            first.sqrt()?;
            stack.push(first);
            return Ok(());
        }
    }
}

fn handle_ctrl<T: Arithmetic>(stack: &mut Vec<T>, control: Control) -> Result<(), ParseError> {
    match control {
        Control::PrintFirst => {
            match stack.last() {
                Some(num) => Ok(num.print()),
                None => Err(ParseError), // if there's nothing in the stack, don't print anything
            }
        },
        Control::PrintAll => Ok(stack.iter().rev().for_each(|j| j.print())),
        Control::Pop => opt_to_err(stack.pop()),
        Control::Clear => Ok(stack.clear()),
        Control::Duplicate => {
            match stack.last() {
                Some(num) => Ok(stack.push(num.clone())),
                None => Err(ParseError),
            }
        }
        Control::Swap => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let second: T = stack.pop().ok_or_else(|| ParseError)?;
            stack.push(first);
            stack.push(second);
            Ok(())
        }
    }
}

fn opt_to_err<T>(option: Option<T>) -> Result<(), ParseError> {
    match option {
        Some(_t) => Ok(()),
        None => Err(ParseError),
    }
}