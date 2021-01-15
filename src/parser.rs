use crate::executor::*;
use crate::lexer::Token;
use std::fmt;


#[derive(Clone, Debug)]
pub enum Variable {
    RealVar(char),
}

#[derive(Clone, Debug)]
pub enum Value {
    NumValue(f64),
    StringValue(String),
}

impl Value {
    // Convenience method, since a "bool" in tibasic is just a float
    pub fn bool(b: bool) -> Value {
        Value::NumValue(match b {
            true => 1.0,
            false => 0.0,
        })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::NumValue(number) => {
                write!(f, "{}", number)
            }
            Value::StringValue(string) => {
                write!(f, "{}", string)
            }
        }
    }
}


impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::NumValue(n1) => match other {
                Value::NumValue(n2) => n1 == n2,
                _ => panic!("Not implemented!"),
            },
            _ => panic!("Not implemented!"),
        }
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Value::NumValue(n) => (n == &1.0) == *other,
            _ => panic!("Not implemented!"),
        }
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Value::NumValue(n) => n == other,
            _ => panic!("Not implemented!"),
        }
    }
}

impl Eval for Value {
    fn eval(&self, _: &mut Context) -> EvalResult {
        Ok(self.clone())
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::NumValue(n) => write!(f, "{:?}", n),
            Value::StringValue(s) => write!(f, "{:?}", s),
        }
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        match self {
            Value::NumValue(n) => Box::new(Value::NumValue(*n)),
            Value::StringValue(s) => Box::new(Value::StringValue(s.clone())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    If(If),
    Then,
    Else,
    For(For),
    While(i64),
    Repeat(i64),
    End,
    Disp(ValRef),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Box<dyn Eval>),
    Command(Command),
}

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    prog: &'a mut Program,
    i: usize,
}

type PlRes = Result<Box<dyn Eval>, ParserError>;

impl<'a> Parser<'a> {
    fn token(&self) -> &Token {
        return &self.tokens[self.i];
    }

    fn advance(&mut self) {
        if self.i == self.tokens.len() - 1 {
            panic!("The impossible happened")
        };
        self.i += 1;
    }

    fn match_if_is(&mut self, token: Token) -> bool {
        // if the current token is token, match
        if self.tokens[self.i] == token {
            self.advance();
            return true;
        }
        false
    }

    fn match_token(&mut self, token: Token) -> Result<(), ParserError> {
        if self.tokens[self.i] != token {
            return Err(ParserError::MissingToken(token));
        }
        self.advance();
        Ok(())
    }

    fn more_tokens(&mut self) -> bool {
        self.i < self.tokens.len() && self.token() != &Token::EndOfInput
    }

    // NOTE ON PRIORITY LEVELS
    // See `grammar.md` for an explaination of the grammar, this will make a lot more
    // sense if you consult that document

    fn pl_12(&mut self) -> PlRes {
        // Storing Variables
        let lhs = self.pl_11()?;
        if self.match_if_is(Token::Store) {
            match self.token().clone() {
                Token::RealVar(name) => {
                    self.advance();
                    Ok(Box::new(StoreNode {
                        val: lhs,
                        var: Variable::RealVar(name.clone()),
                    }))
                }
                _ => Err(ParserError::SyntaxError),
            }
        } else {
            Ok(lhs)
        }
    }

    fn pl_11(&mut self) -> PlRes {
        self.pl_10()
    }

    fn pl_10(&mut self) -> PlRes {
        // Logical or, xor
        let lhs = self.pl_9()?;

        if self.match_if_is(Token::Or) {
            let rhs = self.pl_10()?;
            Ok(Box::new(BinaryOp::or(lhs, rhs)))
        } else if self.match_if_is(Token::Xor) {
            let rhs = self.pl_10()?;
            Ok(Box::new(BinaryOp::xor(lhs, rhs)))
        } else {
            Ok(lhs)
        }
    }

    fn pl_9(&mut self) -> PlRes {
        // Logical and
        let lhs = self.pl_8()?;

        if self.match_if_is(Token::And) {
            let rhs = self.pl_9()?;
            Ok(Box::new(BinaryOp::and(lhs, rhs)))
        } else {
            Ok(lhs)
        }
    }

    fn pl_8(&mut self) -> PlRes {
        // Relational Operators
        let lhs = self.pl_7()?;

        if self.match_if_is(Token::Equal) {
            let rhs = self.pl_8()?;
            Ok(Box::new(BinaryOp::equal(lhs, rhs)))
        } else if self.match_if_is(Token::NotEqual) {
            let rhs = self.pl_8()?;
            Ok(Box::new(BinaryOp::not_equal(lhs, rhs)))
        } else if self.match_if_is(Token::Greater) {
            let rhs = self.pl_8()?;
            Ok(Box::new(BinaryOp::greater(lhs, rhs)))
        } else if self.match_if_is(Token::GreaterEqual) {
            let rhs = self.pl_8()?;
            Ok(Box::new(BinaryOp::greater_equal(lhs, rhs)))
        } else if self.match_if_is(Token::Less) {
            let rhs = self.pl_8()?;
            Ok(Box::new(BinaryOp::less(lhs, rhs)))
        } else if self.match_if_is(Token::LessEqual) {
            let rhs = self.pl_8()?;
            Ok(Box::new(BinaryOp::less_equal(lhs, rhs)))
        } else {
            Ok(lhs)
        }
    }

    fn pl_7(&mut self) -> PlRes {
        let lhs = self.pl_6()?;
        if self.match_if_is(Token::Plus) {
            let rhs = self.pl_7()?;
            Ok(Box::new(BinaryOp::add(lhs, rhs)))
        } else if self.match_if_is(Token::Minus) {
            let rhs = self.pl_7()?;
            Ok(Box::new(BinaryOp::minus(lhs, rhs)))
        } else {
            Ok(lhs)
        }
    }

    fn pl_6(&mut self) -> PlRes {
        // Multiplication, division, implied multiplication
        let lhs = self.pl_5()?;

        if self.match_if_is(Token::Mult) {
            let rhs = self.pl_6()?;
            Ok(Box::new(BinaryOp::mult(lhs, rhs)))
        } else if self.match_if_is(Token::Divide) {
            let rhs = self.pl_6()?;
            Ok(Box::new(BinaryOp::divide(lhs, rhs)))
        // TODO: Adjacent Multiplication Here??
        } else {
            // Super ghetto: since this still may be adjacent multiplication,
            // recurse and try and grab something. If it works, hooray, otherwise,
            // swallow the error and move on
            let rhs = self.pl_6();
            let i_bak = self.i;
            match rhs {
                Ok(val) => {
                    // hooray, we did it
                    Ok(Box::new(BinaryOp::mult(lhs, val)))
                }
                Err(err) => {
                    // well, we tried
                    self.i = i_bak;
                    Ok(lhs)
                }
            }
        }
    }

    fn pl_5(&mut self) -> PlRes {
        // nPr, nCr
        self.pl_4_5()
    }

    fn pl_4_5(&mut self) -> PlRes {
        // Negation
        if self.match_if_is(Token::Negate) {
            let val = self.pl_4()?;
            Ok(Box::new(Negate { val }))
        } else {
            Ok(self.pl_4()?)
        }
    }

    fn pl_4(&mut self) -> PlRes {
        // Power, xroot
        let lhs = self.pl_3()?;

        if self.match_if_is(Token::Power) {
            let rhs = self.pl_3()?;
            Ok(Box::new(BinaryOp::power(lhs, rhs)))
        } else {
            Ok(lhs)
        }
    }

    fn pl_3(&mut self) -> PlRes {
        // Functions that follow their argument (eg !)
        self.pl_2()
    }

    fn pl_2(&mut self) -> PlRes {
        // Functions that precede their arguments (eg not(, sin(
        if self.match_if_is(Token::Not) {
            let val = self.pl_10()?; // todo: should this be "expression" or something?
            self.match_if_is(Token::Rparen);
            Ok(Box::new(Not { val }))
        } else {
            self.pl_1()
        }
    }

    fn pl_1(&mut self) -> PlRes {
        // Groupings, ie parens, brackets, curly braces
        if self.match_if_is(Token::Lparen) {
            let val = self.pl_10()?;
            self.match_if_is(Token::Rparen);
            Ok(val)
        } else {
            self.pl_0()
        }
    }

    fn pl_0(&mut self) -> PlRes {
        // Values & their equivalents
        // this is a wart of me fighting the borrow checker
        match self.token().clone() {
            Token::Number(n) => {
                self.advance();
                return Ok(Box::new(Value::NumValue(n)));
            }
            Token::RealVar(var) => {
                self.advance();
                return Ok(Box::new(VarRef {
                    var: Variable::RealVar(var),
                }));
            }
            Token::Scientific(exponent) => {
                self.advance();
                let base: f64 = 10.0;
                let exponent: f64 = exponent as f64;
                return Ok(Box::new(Value::NumValue(base.powf(exponent as f64))));
            }
            _ => Err(ParserError::UnexpectedToken(self.token().clone())),
        }
    }

    fn expression(&mut self) -> Result<Statement, ParserError> {
        let stat = Statement::Expression(self.pl_12()?);
        self.match_token(Token::EndOfLine)?;
        Ok(stat)
    }

    fn command(&mut self) -> Result<Statement, ParserError> {
        if self.match_if_is(Token::If) {
            let condition = self.pl_10()?;
            self.match_token(Token::EndOfLine)?;
            Ok(Statement::Command(Command::If(If { condition })))
        } else if self.match_if_is(Token::Then) {
            self.match_token(Token::EndOfLine)?;
            Ok(Statement::Command(Command::Then))
        } else if self.match_if_is(Token::Else) {
            self.match_token(Token::EndOfLine)?;
            Ok(Statement::Command(Command::Else))
        } else if self.match_if_is(Token::End) {
            self.match_token(Token::EndOfLine)?;
            Ok(Statement::Command(Command::End))
        } else if self.match_if_is(Token::Disp) {
            let val = self.pl_10()?;
            self.match_token(Token::EndOfLine)?;
            Ok(Statement::Command(Command::Disp(val)))
        } else if self.match_if_is(Token::For) {
            // syntax is a variable, start, stop [inc]
            if let Token::RealVar(name) = self.token().clone() {
                self.advance();
                self.match_token(Token::Comma)?;
                let start = self.pl_10()?;
                self.match_token(Token::Comma)?;
                let stop = self.pl_10()?;
                let inc = if self.match_if_is(Token::Comma) {
                    self.pl_10()?
                } else {
                    Box::new(Value::NumValue(1.0))
                };
                self.match_if_is(Token::Rparen);

                Ok(Statement::Command(Command::For(For {
                    var: Variable::RealVar(name.clone()),
                    start,
                    stop,
                    inc,
                })))
            } else {
                Err(ParserError::SyntaxError)
            }
        } else {
            Err(ParserError::NotYetImplemented(self.token().clone()))
        }
    }

    fn is_command(&mut self) -> bool {
        // Check if the next statement is a command or an expression
        match self.token() {
            Token::If
            | Token::Else
            | Token::For
            | Token::While
            | Token::Repeat
            | Token::End
            | Token::Then
            | Token::Disp => true,
            _ => false,
        }
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.is_command() {
            self.command()
        } else {
            self.expression()
        }
    }

    fn tib_program(&mut self) -> Result<(), ParserError> {
        while self.more_tokens() {
            if self.token() == &Token::EndOfLine {
                self.advance();
                continue;
            }
            let state = self.statement()?;
            self.prog.statements.push(state);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ParserError {
    MissingToken(Token),
    NotYetImplemented(Token),
    UnexpectedToken(Token),
    SyntaxError,
}

pub fn parse(tokens: &Vec<Token>, program: &mut Program) -> Result<(), ParserError> {
    // Will modify program, that is this functions output
    // The basic idea of this function is that we parse tokens and add the resulting statements
    // into program. If

    let mut parser = Parser {
        tokens,
        prog: program,
        i: 0,
    };

    parser.tib_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    #[test]
    fn test_simple_addition() {
        let mut program = Program::new();

        assert!(parse(&lex_str("2+2\n"), &mut program).is_ok());
        // todo: validate the statements in program
    }
}
