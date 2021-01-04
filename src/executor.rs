use crate::parser::Statement;
use crate::parser::*;
use core::fmt::Debug;

use crate::lexer::Token;

#[derive(Debug)]
pub enum ExecError {
    TypeMismatch,
    DivideByZero,
    NotYetImplemented,
    UnexpectedEof,
    SyntaxError,
}

#[derive(Debug)]
pub struct Context {
    pub ans: Value,
    // rest of the variables/state will go here
}

impl Context {
    fn new() -> Context {
        // todo: init vars to random values to punish people using
        // uninitialized variables
        Context {
            ans: Value::NumValue(0.0),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub ctx: Context,
    pub statements: Vec<Statement>,
    pub pc: usize,
}


impl Program {
    pub fn new() -> Program {
        Program {
            ctx: Context::new(),
            statements: Vec::new(),
            pc: 0,
        }
    }

    fn over(&self) -> bool {
        return self.pc >= self.statements.len();
    }

    fn next_statement(&self) -> &Statement {
        let state = &self.statements[self.pc];
        state
    }

    fn advance(&mut self) {
        self.pc += 1;
    }

    fn peek_next(&mut self) -> Result<&Statement, ExecError> {
        if self.pc + 1 >= self.statements.len() {
            Err(ExecError::UnexpectedEof)
        } else {
            Ok(&self.statements[self.pc + 1])
        }
    }

    fn execute(&mut self) -> Result<(), ExecError> {
        while !self.over() {
            match self.next_statement().clone() {
                Statement::Expression(expr) => {
                    self.ctx.ans = expr.eval()?;
                }
                Statement::Command(statement) => {
                    match statement {
                        Command::If(cmd) => self.exec_if(&cmd)?,
                        Command::Then => self.exec_then()?,
                        Command::Else => self.exec_else()?,
                        _ => return Err(ExecError::NotYetImplemented)
                    }
                }
            }
            self.advance();
        }

        Ok(())
    }

    fn next_then(&mut self) -> Result<bool, ExecError> {
        match self.peek_next()? {
            Statement::Command(cmd) => {
                match cmd {
                    Command::Then => Ok(true),
                    _ => Ok(false)
                }
            },
            _ => Ok(false)
        }
    }

    fn exec_if(&mut self, cmd: &If) -> Result<(), ExecError> {
        let result = cmd.condition.eval()?;
        if result == true {
            if self.next_then()? {
                // consume the next then so we don't execute it (executing a then is a cardinal sin)

            } else {
                // do nothing, continue to the next statement as usual
            }
        } else {
            if self.next_then()? {

            } else {
                // skip just the next statement
                self.pc += 1;
            }
        }

        Ok(())
    }
    fn exec_then(&mut self, ) -> Result<(), ExecError> {
        Err(ExecError::SyntaxError)
    }
    fn exec_else(&mut self, ) -> Result<(), ExecError> {
        Ok(())
    }
}


pub type EvalResult = Result<Value, ExecError>;

pub trait Eval {
    fn eval(&self) -> EvalResult;
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
    fn clone_expr(&self) -> Box<dyn Eval>;
}

impl Debug for dyn Eval {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}

pub type ValRef = Box<dyn Eval>;

impl Clone for Box<dyn Eval> {
    fn clone(&self) -> Box<dyn Eval> {
        self.clone_expr()
    }
}
#[derive(Debug, Clone)]
pub struct If {
    pub condition: ValRef,
}

#[derive(Clone)]
pub struct Or {
    pub lhs: ValRef,
    pub rhs: ValRef,
}

pub struct BinaryOp {
    pub lhs: ValRef,
    pub rhs: ValRef,
    pub num_num: fn(f64, f64) -> EvalResult,
    pub token: Token,
}

impl Eval for BinaryOp {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => match vright {
                Value::NumValue(nr) => return (self.num_num)(nl, nr),
                _ => Err(ExecError::TypeMismatch),
            },
            _ => Err(ExecError::TypeMismatch),
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}({:?} || {:?})", self.token, self.lhs, self.rhs)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(BinaryOp{
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            num_num: self.num_num,
            token: self.token.clone()
        })
    }
}

impl BinaryOp {
    pub fn or(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn or(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(fb(lhs) || fb(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Or,
            num_num: or
        }
    }

    pub fn xor(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn xor(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(fb(lhs) ^ fb(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Xor,
            num_num: xor
        }
    }

    pub fn and(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn and(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(fb(lhs) && fb(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::And,
            num_num: and
        }
    }

    pub fn equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn equal(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs == rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Equal,
            num_num: equal
        }
    }

    pub fn not_equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn not_equal(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs != rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::NotEqual,
            num_num: not_equal
        }
    }
    
    pub fn greater(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn greater(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs > rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Greater,
            num_num: greater
        }
    }

    pub fn greater_equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn greater_equal(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs >= rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::GreaterEqual,
            num_num: greater_equal
        }
    }

    pub fn less(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn less(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs < rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Less,
            num_num: less
        }
    }

    pub fn less_equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn less_equal(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs <= rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::LessEqual,
            num_num: less_equal
        }
    }

    pub fn add(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn add(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs + rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Plus,
            num_num: add
        }
    }

    pub fn minus(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn minus(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs - rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Minus,
            num_num: minus
        }
    }

    pub fn mult(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn mult(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs * rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Mult,
            num_num: mult
        }
    }

    pub fn divide(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn divide(lhs : f64, rhs: f64) -> EvalResult {
            if rhs == 0.0 {
                Err(ExecError::DivideByZero)
            } else {
                Ok(Value::NumValue(lhs / rhs))
            }
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Divide,
            num_num: divide
        }
    }

    pub fn power(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn power(lhs : f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs.powf(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Power,
            num_num: power
        }
    }
}


fn fb(f: f64) -> bool {
    f == 1.0
}

pub struct Not {
    pub val: ValRef,
}

impl Eval for Not {
    fn eval(&self) -> EvalResult {
        let val = self.val.eval()?;

        match val {
            Value::NumValue(n) => return Ok(Value::bool(!fb(n))),
            _ => Err(ExecError::TypeMismatch),
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Not({:?})", self.val)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(Not{
            val: self.val.clone(),
        })
    }
}
pub struct Negate {
    pub val: ValRef,
}

impl Eval for Negate {
    fn eval(&self) -> EvalResult {
        let val = self.val.eval()?;

        match val {
            Value::NumValue(n) => return Ok(Value::NumValue(-1.0 * n)),
            _ => Err(ExecError::TypeMismatch),
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Neg({:?})", self.val)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(Negate{
            val: self.val.clone(),
        })
    }

}

pub fn execute(program: &mut Program) -> Result<(), ExecError> {
    program.execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    fn exec(input: &str) -> Value {
        let mut program = Program::new();
        assert!(parse(&lex_str(input), &mut program).is_ok());
        assert!(execute(&mut program).is_ok());
        return program.ctx.ans;
    }
    #[test]
    fn test_binary_ops_numbers() {
        assert_eq!(exec("2+2"), 4.0);
        assert_eq!(exec("2-2"), 0.0);
        assert_eq!(exec("2*2"), 4.0);
        assert_eq!(exec("2/4"), 0.5);
        assert_eq!(exec("-2"), -2.0);
        assert_eq!(exec("2^4"), 16.0);
        // todo: validate the statements in program
    }

    #[test]
    fn test_logic_numbers() {
        assert_eq!(exec("1 or 0"), true);
        assert_eq!(exec("0 or 0"), false);

        assert_eq!(exec("1 xor 0"), true);
        assert_eq!(exec("1 or 1"), true);
        assert_eq!(exec("0 or 0"), false);

        assert_eq!(exec("1 and 1"), true);
        assert_eq!(exec("1 and 0"), false);
        assert_eq!(exec("0 and 0"), false);

        assert_eq!(exec("not(1)"), false);
        assert_eq!(exec("not(0)"), true);
        assert_eq!(exec("not(1"), false);
        assert_eq!(exec("not(0"), true);
    }

    #[test]
    fn test_equality_ops() {
        assert_eq!(exec("1 = 1"), true);
        assert_eq!(exec("1 = 0"), false);

        assert_eq!(exec("1 != 1"), false);
        assert_eq!(exec("1 != 0"), true);

        assert_eq!(exec("1 > 0"), true);
        assert_eq!(exec("1 > 1"), false);
        assert_eq!(exec("1 > 2"), false);

        assert_eq!(exec("1 > 0"), true);
        assert_eq!(exec("1 >= 1"), true);
        assert_eq!(exec("1 > 2"), false);

        assert_eq!(exec("1 < 0"), false);
        assert_eq!(exec("1 < 1"), false);
        assert_eq!(exec("1 < 2"), true);

        assert_eq!(exec("1 < 0"), false);
        assert_eq!(exec("1 <= 1"), true);
        assert_eq!(exec("1 < 2"), true);
        // todo: validate the statements in program
    }
}
