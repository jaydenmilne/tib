use logos::{Logos, Lexer};
use std::str::FromStr;

fn number(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    let num : f64 = f64::from_str(slice).ok()?;

    Some(num)
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {

    #[regex("[0-9]*[.]?[0-9]*", number)]
    Number(f64),
    #[token("or")]
    Or,
    #[token("xor")]
    Xor,
    #[token("and")]
    And,
    #[token("not(")]
    Not,
    #[token(")")]
    Rparen,
    #[token("=")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mult,
    #[token("/")]
    Divide,
    #[token("^")]
    Power,

    #[token("\r\n")]
    #[token("\n")]
    #[token(":")]
    EndOfLine,
    EndOfInput,

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
    all.push(Token::EndOfInput);
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
        assert_eq!(lex_str("1"), [Token::Number(1.0), Token::EndOfInput]);
        assert_eq!(lex_str("12345"), [Token::Number(12345.0), Token::EndOfInput]);
        assert_eq!(lex_str("0123456"), [Token::Number(123456.0), Token::EndOfInput]);
        assert_eq!(lex_str("     12 34"), [Token::Number(12.0), Token::Number(34.0), Token::EndOfInput])
    }

    #[test]
    fn test_floats() {
        assert_eq!(lex_str("1.4"), [Token::Number(1.4), Token::EndOfInput]);
        assert_eq!(lex_str(".4"), [Token::Number(0.4), Token::EndOfInput]);
        assert_eq!(lex_str("1."), [Token::Number(1.0), Token::EndOfInput]);
        assert_eq!(lex_str("\t1.0 0.4"), [Token::Number(1.0), Token::Number(0.4), Token::EndOfInput]);

        // this case fails on the calculator, but I think it should be a parsing error
        // not a lexing error
        assert_eq!(lex_str("1.0.1"), [Token::Number(1.0),Token::Number(0.1), Token::EndOfInput]);
    }

    #[test]
    fn test_binary_ops() {
        assert_eq!(lex_str("4+4"), [Token::Number(4.0), Token::Plus, Token::Number(4.0), Token::EndOfInput]);
        assert_eq!(lex_str("-"), [Token::Minus, Token::EndOfInput]);
        assert_eq!(lex_str("/"), [Token::Divide, Token::EndOfInput]);
        assert_eq!(lex_str("*"), [Token::Mult, Token::EndOfInput]);
    }


    #[test]
    fn test_logical_operators() {
        assert_eq!(lex_str("xor"), [Token::Xor, Token::EndOfInput]);
        assert_eq!(lex_str("or"), [Token::Or, Token::EndOfInput]);
        assert_eq!(lex_str("orxor"), [Token::Or, Token::Xor, Token::EndOfInput]);
        assert_eq!(lex_str("and"), [Token::And, Token::EndOfInput]);
        assert_eq!(lex_str("not("), [Token::Not, Token::EndOfInput]);
        assert_eq!(lex_str(")"), [Token::Rparen, Token::EndOfInput]);
        assert_eq!(lex_str("*"), [Token::Mult, Token::EndOfInput]);
    }

    #[test]
    fn test_equality_comparison() {
        assert_eq!(lex_str("<"), [Token::Less, Token::EndOfInput]);
        assert_eq!(lex_str("<="), [Token::LessEqual, Token::EndOfInput]);
        assert_eq!(lex_str(">"), [Token::Greater, Token::EndOfInput]);
        assert_eq!(lex_str(">="), [Token::GreaterEqual, Token::EndOfInput]);
        assert_eq!(lex_str("="), [Token::Equal, Token::EndOfInput]);
        assert_eq!(lex_str("<<=>>=="), [Token::Less, Token::LessEqual, Token::Greater, Token::GreaterEqual, Token::Equal, Token::EndOfInput])
    }

    #[test]
    #[should_panic(expected="Failed to parse token `.`")]
    fn test_unkown_token() {
        lex_str(".");
    }
}
