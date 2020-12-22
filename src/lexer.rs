use logos::{Logos};


//fn number(lex: &mut Lexer<Token>) -> Option<u64> {
//    let slice = lex.slice();
//
//}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {

    #[regex("[0-9]*[.]?[0-9]*")]
    Number,

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
    EndOfLine,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    UnknownToken,
}


pub fn lex(input: &String) -> Vec<(Token, std::ops::Range<usize>)> {
    // Do magic!
    let lex = Token::lexer(input);
    let all : Vec<_> = lex.spanned().collect();

    all
}

pub fn lex_str(input: &str) -> Vec<(Token, std::ops::Range<usize>)> {
    lex(&String::from(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ints() {
        assert_eq!(lex_str("1"), [(Token::Number, 0..1)]);
        assert_eq!(lex_str("12345"), [(Token::Number, 0..5)]);
        assert_eq!(lex_str("0123456"), [(Token::Number, 0..7)]);
        assert_eq!(lex_str("     12 34"), [(Token::Number, 5..7), (Token::Number, 8..10)])
    }

    #[test]
    fn test_floats() {
        assert_eq!(lex_str("1.4"), [(Token::Number, 0..3)]);
        assert_eq!(lex_str(".4"), [(Token::Number, 0..2)]);
        assert_eq!(lex_str("1."), [(Token::Number, 0..2)]);
        assert_eq!(lex_str("\t1.0 0.4"), [(Token::Number, 1..4), (Token::Number, 5..8)]);

        // this case fails on the calculator, but I think it should be a parsing error
        // not a lexing error
        assert_eq!(lex_str("1.0.1"), [(Token::Number, 0..3),(Token::Number, 3..5)]);
    }

    #[test]
    fn test_binary_ops() {
        assert_eq!(lex_str("+-*/"), [(Token::Plus, 0..1), (Token::Minus, 1..2), (Token::Times, 2..3), (Token::Divide, 3..4)]);
        assert_eq!(lex_str("4+4"), [(Token::Number, 0..1), (Token::Plus, 1..2), (Token::Number, 2..3)]);
    }
}
