use logos::{Logos, Lexer};
use std::str::FromStr;

fn number(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    let num : f64 = f64::from_str(slice).ok()?;

    Some(num)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {

    #[regex("[0-9]*[.]?[0-9]*", number)]
    Number(f64),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Divide,

    #[token("\r\n")]
    #[token("\n")]
    #[token(":")]
    EndOfLine,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]  // todo: might need to refine this for strings?
    UnknownToken,
}


pub fn lex(input: &String) -> Vec<Token> {
    // Do magic!
    let lex = Token::lexer(input);
    let mut all : Vec<Token> = Vec::new();

    for (token, span) in lex.spanned() {
        if token == Token::UnknownToken {
            // todo: better error handling (this function should return Result, etc)
            // todo: store the line number in the EndofLine token
            panic!("Failed to parse token `{}`", &input[span]);
        }

        all.push(token);
    }
    all
}

pub fn lex_str(input: &str) -> Vec<Token> {
    lex(&String::from(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ints() {
        assert_eq!(lex_str("1"), [(Token::Number(1.0), 0..1)]);
        assert_eq!(lex_str("12345"), [(Token::Number(12345.0), 0..5)]);
        assert_eq!(lex_str("0123456"), [(Token::Number(123456.0), 0..7)]);
        assert_eq!(lex_str("     12 34"), [(Token::Number(12.0), 5..7), (Token::Number(34.0), 8..10)])
    }

    #[test]
    fn test_floats() {
        assert_eq!(lex_str("1.4"), [(Token::Number(1.4), 0..3)]);
        assert_eq!(lex_str(".4"), [(Token::Number(0.4), 0..2)]);
        assert_eq!(lex_str("1."), [(Token::Number(1.0), 0..2)]);
        assert_eq!(lex_str("\t1.0 0.4"), [(Token::Number(1.0), 1..4), (Token::Number(0.4), 5..8)]);

        // this case fails on the calculator, but I think it should be a parsing error
        // not a lexing error
        assert_eq!(lex_str("1.0.1"), [(Token::Number(1.0), 0..3),(Token::Number(0.1), 3..5)]);
    }

    #[test]
    fn test_binary_ops() {
        assert_eq!(lex_str("+-*/"), [(Token::Plus, 0..1), (Token::Minus, 1..2), (Token::Times, 2..3), (Token::Divide, 3..4)]);
        assert_eq!(lex_str("4+4"), [(Token::Number(4.0), 0..1), (Token::Plus, 1..2), (Token::Number(4.0), 2..3)]);
    }

    #[test]
    #[should_panic(expected="Failed to parse token `.`")]
    fn test_unkown_token() {
        lex_str(".");
    }
}
