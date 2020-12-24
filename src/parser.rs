use crate::lexer;
use crate::lexer::Token;

pub struct Program {
    statements: Vec<Statement>
    // todo: things like the label cache
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new()
        }
    }
}

pub struct Add {
    lhs: Box<dyn Eval>,
    rhs: Box<dyn Eval>,
}

impl Eval for Add {
    fn eval(&self) -> Value {
        let vleft = self.lhs.eval();
        let vright = self.rhs.eval();

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Value::NumValue(nl + nr),
                    _ => panic!("Not implemented!"),
                }
            _ => panic!("Not implemented!")
        }
    }
}

pub enum Value {
    NumValue(f64),
    StringValue(String),
}

impl Eval for Value {
    fn eval(&self) -> Value {
        Value(self.)
    }
}

pub trait Eval {
    fn eval(&self) -> Value;
}

pub struct Expression {
    val : Box<dyn Eval>,
}

pub struct Keyword {
    // todo (this will be for things like if statements, etc)
}

pub enum Statement {
    Expr(Expression),
    Keyword(Keyword)
}

struct Parser<'a> {
    tokens : &'a Vec<Token>,
    prog : &'a mut Program,
    i: usize,
}


impl<'a> Parser<'a> {

    fn token(&self) -> &Token {
        return &self.tokens[self.i]
    }

    fn advance(&mut self) {
        if self.i == self.tokens.len() - 1 {
            panic!("This happened Jayden");
        };
        self.i += 1;
    }

    fn match_if_is(&mut self, token : Token) -> bool {
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
        self.i < self.tokens.len()
    }

    fn pl_1(&mut self) -> Result<Box<impl Eval>, ParserError> {
        // skip straight to pl 6 for now, since we are only trying to get addition working
        self.pl_6()
    }

    fn pl_6(&mut self) -> Result<Box<impl Eval>, ParserError> {
        let e1 = self.pl_14()?;
        if self.match_if_is(Token::Plus) {
            let e2 = self.pl_6()?;
            return Ok(Box::new(Add {
                lhs: e1,
                rhs: e2
            }))
        } else {
            return Ok(e1);
        }
    }

    fn pl_14(&mut self) -> Result<Box<impl Eval>, ParserError> {
        if self.token() == Token::Number {
            let v = 
        }

    }

    fn statement(&mut self) -> Result<(), ParserError> {
        while self.token() != &Token::EndOfLine {
            self.pl_1()?;
        }
        Ok(())
    }
    fn tib_program(&mut self, program : &mut Program) -> Result<(), ParserError> {

        while self.more_tokens() {

        }

        Ok(())
    }
}

pub enum ParserError {
    MissingOperand,
    IllegalSyntax,
    MissingToken(Token),
    // this one is special, the REPL will not print any errors on this
    Incomplete 
}


pub fn parse(tokens : &Vec<Token>, program : &mut Program) -> Result<(), ParserError> {
    // Will modify program, that is this functions output
    // The basic idea of this function is that we parse tokens and add the resulting statements
    // into program. If 

    let mut parser = Parser {
        tokens,
        prog: program,
        i: 0
    };
    
    parser.tib_program(program)
}