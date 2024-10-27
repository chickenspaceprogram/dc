use crate::number::{Arithmetic, MathError};

#[derive(Clone)]
pub struct Integer128 {
    pub value: i128,
}

impl Arithmetic for Integer128 {
    fn add(&mut self, num: Integer128) -> Result<(), MathError> {
        match self.value.checked_add(num.value) {
            Some(val) => {
                self.value = val;
                return Ok(())
            },
            None => Err(MathError),
        }
    }

    fn subtract(&mut self, num: Self) -> Result<(), MathError> {
        match self.value.checked_sub(num.value) {
            Some(val) => {
                self.value = val;
                return Ok(())
            },
            None => Err(MathError),
        }
    }

    fn multiply(&mut self, num: Self) -> Result<(), MathError> {
        match self.value.checked_mul(num.value) {
            Some(val) => {
                self.value = val;
                return Ok(())
            },
            None => {
                println!("num1: {}\nnum2: {}", self.value, num.value);
                return Err(MathError)
            },
        }
    }

    fn divide(&mut self, num: Self) -> Result<(), MathError> {
        match self.value.checked_div(num.value) {
            Some(val) => {
                self.value = val;
                return Ok(())
            },
            None => Err(MathError),
        }
    }

    fn modulo(&mut self, num: Self) -> Result<(), MathError> {
        match self.value.checked_rem(num.value) {
            Some(val) => {
                self.value = val;
                return Ok(())
            },
            None => Err(MathError),
        }
    }

    fn pow(&mut self, num: Self) -> Result<(), MathError> {
        match num.value.try_into() {
            Ok(val) => {
                match self.value.checked_pow(val) {
                    Some(result) => self.value = result,
                    None => return Err(MathError)
                }
            },
            Err(_e) => return Err(MathError),
        }
        return Ok(())
    }

    fn sqrt(&mut self) -> Result<(), MathError> {
        return Err(MathError)
    }

    fn print(&self) {
        println!("{}", self.value);
    }
}

impl Integer128 {
    pub fn new(num: i128) -> Integer128 {
        let int64 = Integer128 {
            value: num,
        };
        return int64
    }
}