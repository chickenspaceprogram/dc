use crate::{integer::Integer128, number::Token, number::Control, number::Operator, calculator::ParseError};

pub fn parse_int64(input: String) -> Result<Vec<Token<Integer128>>, ParseError> {
    let mut tokens: Vec<Token<Integer128>> = Vec::new();
    let mut current_num = Integer128::new(0);
    let mut num_sign = NumSign::Positive;
    let mut need_push_number = NeedPushNum::No;

    let mut iter = input.chars().peekable();
    for i in iter.clone() {
        match i {
            // Operators
            '+' => tokens.push(Token::Op(Operator::Add)),
            '-' => tokens.push(Token::Op(Operator::Subtract)),
            '*' => tokens.push(Token::Op(Operator::Multiply)),
            '/' => tokens.push(Token::Op(Operator::Divide)),
            '%' => tokens.push(Token::Op(Operator::Mod)),
            '^' => tokens.push(Token::Op(Operator::Pow)),
            'v' => tokens.push(Token::Op(Operator::Sqrt)),
            '_' => {
                match iter.peek() {
                    Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => num_sign = NumSign::Negative,
                    _ => return Err(ParseError),
                }
            }

            // Control sequences
            'p' => tokens.push(Token::Ctrl(Control::PrintFirst)),
            'f' => tokens.push(Token::Ctrl(Control::PrintAll)),
            'R' => tokens.push(Token::Ctrl(Control::Pop)),
            'c' => tokens.push(Token::Ctrl(Control::Clear)),
            'd' => tokens.push(Token::Ctrl(Control::Duplicate)),
            'r' => tokens.push(Token::Ctrl(Control::Swap)),
            'n' => {
                tokens.push(Token::Ctrl(Control::PrintFirst));
                tokens.push(Token::Ctrl(Control::Pop));
            }

            // Numbers
            '0' => {
                if current_num.value != 0 {
                    current_num.value *= 10;
                }
                else {
                    need_push_number = NeedPushNum::Yes;
                }
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'  => {
                if current_num.value == 0 {
                    let digit: i128 = i.to_digit(10).unwrap().into();
                    current_num.value += digit; // unwrap can be used since by definition we are only matching digits
                    need_push_number = NeedPushNum::Yes;
                    match num_sign {
                        NumSign::Negative => current_num.value = -current_num.value,
                        NumSign::Positive => (),
                    }
                }
                else {
                    current_num.value *= 10;
                    let digit: i128 = i.to_digit(10).unwrap().into();
                    current_num.value += digit; // unwrap can be used since by definition we are only matching digits
                }
            }

            ' ' | '\n' => {
                match need_push_number {
                    NeedPushNum::Yes => {
                        tokens.push(Token::Num(current_num.clone())); // yeah the clone is a slight cost but it's fiiine
                        current_num.value = 0;
                        need_push_number = NeedPushNum::No;
                    },
                    NeedPushNum::No => (),
                }
            }

            _ => {
                println!("the thing was: {}", i as u32);
                return Err(ParseError)
            },

        }
    }
    return Ok(tokens)
}

enum NumSign {
    Positive,
    Negative,
}

enum NeedPushNum {
    Yes,
    No,
}