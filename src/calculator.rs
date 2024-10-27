use crate::number::{Arithmetic, Token, Control, Operator};
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid token, could not parse expression.")
    }
}

impl Error for ParseError {}

/// Does the calculations using the operands, control sequences, and numbers in `operations`, and returns whatever was left in the stack.
pub fn calculator<T: Arithmetic>(operations: Vec<Token<T>>) -> Result<Vec<T>, Box<dyn Error>> {
    let mut stack: Vec<T> = Vec::new();
    for i in operations {
        match i {
            Token::Op(operation) => handle_op(&mut stack, operation)?,
            Token::Ctrl(control) => handle_ctrl(&mut stack, control)?,
            Token::Num(num) => stack.push(num),
        };
    }
    return Ok(stack);
}

fn handle_op<T: Arithmetic>(stack: &mut Vec<T>, op: Operator) -> Result <(), ParseError>{
    match op {
        Operator::Add => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.add(first);
            stack.push(second);
            return Ok(());
        }
        Operator::Subtract => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.subtract(first);
            stack.push(second);
            return Ok(());
        }
        Operator::Multiply => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.multiply(first);
            stack.push(second);
            return Ok(());
        }
        Operator::Divide => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.multiply(first);
            stack.push(second);
            return Ok(());
        }
        Operator::Mod => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.modulo(first);
            stack.push(second);
            return Ok(());
        }
        Operator::Pow => {
            let first = stack.pop().ok_or_else(|| ParseError)?;
            let mut second = stack.pop().ok_or_else(|| ParseError)?;
            second.pow(first);
            stack.push(second);
            return Ok(());
        }
        Operator::Sqrt => {
            let mut first = stack.pop().ok_or_else(|| ParseError)?;
            first.sqrt();
            stack.push(first);
            return Ok(());
        }
        Operator::Negate => { // this only actually gets used when the _ operator is entered alone. it negates the top thing on the stack
            let mut first = stack.pop().ok_or_else(|| ParseError)?;
            first.negate();
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