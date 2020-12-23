use crate::lexer;

pub struct Program {
    statements: Vec<Statement>
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

struct Parser {
    tokens : Vec<lexer::Token>,
    pc : usize,  // "program counter"

}


pub fn parse(tokens : &Vec<lexer::Token>, program : &mut Program) {
    
}