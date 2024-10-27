use crate::number::{Arithmetic, MathError};

#[derive(Clone)]
pub struct Integer64 {
    value: i64,
}

impl Arithmetic for Integer64 {
    fn add(&mut self, num: Integer64) -> Result<(), MathError> {
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
            None => Err(MathError),
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

    fn negate(&mut self) -> Result<(), MathError> {
        self.value = -self.value;
        return Ok(())
    }

    fn print(&self) {
        println!("{}", self.value);
    }
}