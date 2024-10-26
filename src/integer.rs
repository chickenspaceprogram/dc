use crate::number::Arithmetic;

#[derive(Clone)]
pub struct Integer64 {
    value: i64,
}

impl Arithmetic for Integer64 {
    fn add<Integer64>(&mut self, num: Integer64) {
        self.value += num.value;
    }
}