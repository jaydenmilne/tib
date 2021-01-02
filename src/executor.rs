use crate::parser::*;
use crate::parser::Statement;

use crate::lexer::Token;
#[derive(Debug)]
pub struct Program {
    // todo: replace this with a "block"
    pub statements: Vec<Statement>,
    // "program counter"
    pc: usize,

    // The "last result" variable
    pub val: Value,
    // todo: things like the label cache, variables, etc

}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
            pc: 0,
            val: Value::NumValue(0.0)
        }
    }

    fn over(&self) -> bool {
        return self.pc >= self.statements.len()
    }

    fn next_statement(&mut self) -> &Statement {
        let state = &self.statements[self.pc];
        self.pc += 1;
        state
    }
}
#[derive(Debug)]
pub enum ExecError {
    TypeMismatch,
    DivideByZero
}

pub trait Eval {
    fn eval(&self) -> EvalResult;
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

use core::fmt::Debug;
impl Debug for dyn Eval {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}

pub type EvalResult = Result<Value, ExecError>;
pub type ValRef = Box<dyn Eval>;

pub struct Or {
    pub lhs: ValRef,
    pub rhs: ValRef
}

fn fb(f: f64) -> bool {
    f == 1.0
}

impl Eval for Or {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(fb(nl) || fb(nr))),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Or({:?} || {:?})", self.lhs, self.rhs)
    }
}

pub struct Xor {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Xor {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(fb(nl) ^ fb(nr))),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Xor({:?} ^ {:?})", self.lhs, self.rhs)
    }
}

pub struct And {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for And {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(fb(nl) && fb(nr))),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "And({:?} && {:?})", self.lhs, self.rhs)
    }
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
}
pub struct Equal {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Equal {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(nl == nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Eq({:?} == {:?})", self.lhs, self.rhs)
    }
}
pub struct NotEqual {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for NotEqual {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(nl != nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Neq({:?} != {:?})", self.lhs, self.rhs)
    }
}

pub struct Greater {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Greater {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(nl > nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Gr({:?} > {:?})", self.lhs, self.rhs)
    }
}

pub struct GreaterEqual {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for GreaterEqual {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(nl >= nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Greq({:?} >= {:?})", self.lhs, self.rhs)
    }
}

pub struct Less {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Less {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(nl < nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Less({:?} < {:?})", self.lhs, self.rhs)
    }
}

pub struct LessEqual {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for LessEqual {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::bool(nl <= nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Lesseq({:?} <= {:?})", self.lhs, self.rhs)
    }
}

pub struct Minus {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Minus {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::NumValue(nl - nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Minus({:?} - {:?})", self.lhs, self.rhs)
    }
}

pub struct Mult {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Mult {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::NumValue(nl * nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Mult({:?} * {:?})", self.lhs, self.rhs)
    }
}

pub struct Divide {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Divide {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => {
                        if nr == 0.0 {
                            Err(ExecError::DivideByZero)
                        } else {
                            Ok(Value::NumValue(nl / nr))
                        }
                    },
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Div({:?} / {:?})", self.lhs, self.rhs)
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
}

pub struct Power {
    pub lhs: ValRef,
    pub rhs: ValRef
}

impl Eval for Power {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::NumValue(nl.powf(nr))),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Pow({:?} ^ {:?})", self.lhs, self.rhs)
    }
}


pub struct Add {
    pub lhs: ValRef,
    pub rhs: ValRef,
}

impl Eval for Add {
    fn eval(&self) -> EvalResult {
        let vleft = self.lhs.eval()?;
        let vright = self.rhs.eval()?;

        match vleft {
            Value::NumValue(nl) => 
                match vright {
                    Value::NumValue(nr) => return Ok(Value::NumValue(nl + nr)),
                    _ => Err(ExecError::TypeMismatch),
                }
            _ => Err(ExecError::TypeMismatch)
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Add({:?} + {:?})", self.lhs, self.rhs)
    }
}

pub fn execute(program : &mut Program) -> Result<(), ExecError> {
    while !program.over() {
        let statement = program.next_statement();
        // todo: error checking
        match statement {
            Statement::Expr(expr) => {
                program.val = expr.eval()?;
            },
            Statement::Keywrd(kwrd) => {
                panic!("Not implemented!");
            }
        }
    }

    return Ok(());
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    fn exec(input : &str) -> Value {
        let mut program = Program::new();
        assert!(parse(&lex_str(input), &mut program).is_ok());
        assert!(execute(&mut program).is_ok());
        return program.val;
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