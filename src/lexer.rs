use logos::{Lexer, Logos};
use std::str::FromStr;
use regex::Regex;

fn number(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    let num: f64 = f64::from_str(slice).ok()?;

    Some(num)
}

fn number_var(lex: &mut Lexer<Token>) -> Option<char> {
    let slice = lex.slice();

    // we only want to represent this as the single character, not the string
    if slice == "Theta" {
        Some('θ')
    } else {
        Some(slice.chars().nth(0)?)
    }
}

fn parse_label(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = &lex.slice()[1..];
    let re = Regex::new(r"[A-Z|0-9|θ]{1,2}").unwrap();
    Some(String::from(re.find(slice)?.as_str()))
}


fn scientific_parser(lex: &mut Lexer<Token>) -> Option<i32> {
    let slice = lex.slice();
    match String::from(slice)[1..].parse::<i32>() {
        Ok(num) => Some(num),
        Err(_err) => None,
    }
}

#[derive(Debug)]
pub enum LexError {
    UnknownToken(String),
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
    #[token("(")]
    Lparen,
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
    #[token("--")]
    Negate,
    #[token("*")]
    Mult,
    #[token("/")]
    Divide,
    #[token("^")]
    Power,
    #[token("->")]
    Store,

    #[regex("e[-]?[0-9][0-9]?", scientific_parser)]
    Scientific(i32),

    #[token("Theta", number_var)]
    #[regex(r"[A-Z|θ]", number_var)]
    RealVar(char),

    // This is where I would bifrucate this enum into "statements" and "expressions"
    // things after this are "keywords" that aren't eval'd, instead they are executed
    #[token("If")]
    If,
    #[token("Then")]
    Then,
    #[token("Else")]
    Else,
    #[token("For(")]
    For,
    #[token("While")]
    While,
    #[token("Repeat")]
    Repeat,
    #[token("End")]
    End,
    #[token("Disp")]
    Disp,

    #[regex(r"Lbl\s*[A-Z|0-9|θ][A-Z|0-9|θ]?", parse_label)]
    Lbl(String),

    #[regex(r"Goto\s*[A-Z|0-9|θ][A-Z|0-9|θ]?", parse_label)]
    Goto(String),

    #[token(",")]
    Comma,
    #[token("\r\n")]
    #[token("\n")]
    #[token(":")]
    EndOfLine,
    EndOfInput,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)] // todo: might need to refine this for strings?
    UnknownToken,
}

pub fn lex(input: &String) -> Result<Vec<Token>, LexError> {
    // Do magic!
    let lex = Token::lexer(input);
    let mut all: Vec<Token> = Vec::new();

    for (token, span) in lex.spanned() {
        if token == Token::UnknownToken {
            // todo: better error handling (this function should return Result, etc)
            // todo: store the line number in the EndofLine token
            return Err(LexError::UnknownToken(String::from(&input[span])));
        }

        all.push(token);
    }
    all.push(Token::EndOfInput);
    Ok(all)
}

pub fn lex_str(input: &str) -> Vec<Token> {
    //
    lex(&String::from(input)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ints() {
        assert_eq!(lex_str("1"), [Token::Number(1.0), Token::EndOfInput]);
        assert_eq!(
            lex_str("12345"),
            [Token::Number(12345.0), Token::EndOfInput]
        );
        assert_eq!(
            lex_str("0123456"),
            [Token::Number(123456.0), Token::EndOfInput]
        );
        assert_eq!(
            lex_str("     12 34"),
            [Token::Number(12.0), Token::Number(34.0), Token::EndOfInput]
        )
    }

    #[test]
    fn test_floats() {
        assert_eq!(lex_str("1.4"), [Token::Number(1.4), Token::EndOfInput]);
        assert_eq!(lex_str(".4"), [Token::Number(0.4), Token::EndOfInput]);
        assert_eq!(lex_str("1."), [Token::Number(1.0), Token::EndOfInput]);
        assert_eq!(
            lex_str("\t1.0 0.4"),
            [Token::Number(1.0), Token::Number(0.4), Token::EndOfInput]
        );

        // this case fails on the calculator, but I think it should be a parsing error
        // not a lexing error
        assert_eq!(
            lex_str("1.0.1"),
            [Token::Number(1.0), Token::Number(0.1), Token::EndOfInput]
        );
    }

    #[test]
    fn test_binary_ops() {
        assert_eq!(
            lex_str("4+4"),
            [
                Token::Number(4.0),
                Token::Plus,
                Token::Number(4.0),
                Token::EndOfInput
            ]
        );
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
        assert_eq!(
            lex_str("<<=>>=="),
            [
                Token::Less,
                Token::LessEqual,
                Token::Greater,
                Token::GreaterEqual,
                Token::Equal,
                Token::EndOfInput
            ]
        )
    }

    #[test]
    fn test_unkown_token() {
        match lex(&String::from("burrito")) {
            Ok(_) => panic!("Didn't fail!"),
            Err(err) => match err {
                LexError::UnknownToken(_) => return,
            },
        }
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(
            lex_str("1e50"),
            [Token::Number(1.0), Token::Scientific(50), Token::EndOfInput]
        );
        assert_eq!(
            lex_str("1e-50"),
            [
                Token::Number(1.0),
                Token::Scientific(-50),
                Token::EndOfInput
            ]
        );
        assert_eq!(lex_str("e-50"), [Token::Scientific(-50), Token::EndOfInput]);
    }
}
