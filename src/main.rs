pub mod number;
pub mod calculator;
pub mod integer;
fn main() {
    let input = include_str!("../../../rpn.txt");
    let tokenized_input = match number::parser::parse_int64(input.to_string()) {
        Ok(stuff) => stuff,
        Err(e) => {
            println!("an error occurred: {}", e);
            return
        },
    };
    calculator::calculator(tokenized_input, Vec::new()).unwrap();
}
