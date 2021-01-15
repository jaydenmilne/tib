use crate::parser::Statement;
use crate::parser::*;
use core::fmt::Debug;
use rand::Rng;
use std::collections::HashMap;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum ExecError {
    TypeMismatch,
    DivideByZero,
    NotYetImplemented,
    UnexpectedEof,
    SyntaxError,
    UnexpectedElse,
    UnexpectedEnd,
    EmptyBlock,
    UnexpectedThen,
}

#[derive(Debug)]
pub struct Context {
    pub ans: Value,
    pub reals: HashMap<char, Value>,
    // rest of the variables/state will go here
}

impl Context {
    fn set(&mut self, var: &Variable, val: Value) -> Result<Value, ExecError> {
        match var {
            Variable::RealVar(name) => {
                self.reals.insert(name.clone(), val.clone());
                Ok(val)
            }
        }
    }

    fn get(&mut self, var: &Variable) -> Result<Value, ExecError> {
        match var {
            Variable::RealVar(name) => {
                // check if is in hashmap
                match self.reals.get(&name) {
                    Some(val) => Ok(val.clone()),
                    // tried to access an uninitialized variable! Punish them for their
                    // insolence
                    None => {
                        let mut rng = rand::thread_rng();
                        Ok(Value::NumValue(rng.gen_range(-10e20..10e20)))
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Block {
    // Enum for entries on the blockstack. The usize is the index of the
    // node that is being repeated
    IfBlock(usize, bool), // extra bool is to keep track of if we took the true arm
    ForBlock(usize),
    WhileBlock(usize),
    RepeatBlock(usize),
}

impl Context {
    fn new() -> Context {
        // todo: init vars to random values to punish people using
        // uninitialized variables
        Context {
            ans: Value::NumValue(0.0),
            reals: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub ctx: Context,
    pub statements: Vec<Statement>,
    pub pc: usize,
    pub blockstack: Vec<Block>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            ctx: Context::new(),
            statements: Vec::new(),
            pc: 0,
            blockstack: Vec::new(),
        }
    }

    fn over(&self) -> bool {
        return self.pc >= self.statements.len();
    }

    fn next_statement(&self) -> Result<&Statement, ExecError> {
        if self.pc >= self.statements.len() {
            Err(ExecError::UnexpectedEof)
        } else {
            let state = &self.statements[self.pc];
            Ok(state)
        }
    }

    fn enter_block(
        &mut self,
        expect_else: bool,
        skip_else: bool,
        block: Block,
    ) -> Result<(), ExecError> {
        let inserted_loc = self.pc;
        self.blockstack.push(block);

        self.scan_and_advance(expect_else, skip_else)?;

        // make sure that we are popping off the same one we put on
        let removed = self.blockstack.pop().ok_or(ExecError::SyntaxError)?;
        let removed_loc: usize;
        match removed {
            Block::IfBlock(loc, _) => removed_loc = loc,
            Block::WhileBlock(loc) => removed_loc = loc,
            Block::ForBlock(loc) => removed_loc = loc,
            Block::RepeatBlock(loc) => removed_loc = loc,
        }

        if removed_loc != inserted_loc {
            Err(ExecError::SyntaxError)
        } else {
            Ok(())
        }
    }

    fn scan_and_advance(&mut self, expect_else: bool, skip_else: bool) -> Result<(), ExecError> {
        // Used when we have determined we need to skip some code, becuase
        // we are not going to execute a loop body or we are skipping an arm
        // of an else statement
        // Scan for an End command (or an Else command too if we are scanning
        // because of an If command). As we go along, we need to respect and
        // keep track of other block commands on the block stack to make sure
        // we don't steal someone else's End or Else. We can't do any caching
        // or parsing beforehand because in tib End can be "turned off", eg
        //
        // 1 -> A
        // If 1
        //   Then
        //   Lbl A
        //   If A
        //     End
        // 0 -> A
        // Goto A
        // is a valid program (loops forever)
        loop {
            self.advance();
            match self.next_statement()? {
                Statement::Expression(_) => (),
                Statement::Command(cmd) => {
                    match cmd {
                        Command::Else => {
                            if !expect_else {
                                return Err(ExecError::UnexpectedElse);
                            }
                            if skip_else {
                                // if we found an else, and we don't care about executing what is after
                                // it, then we can just pretend it doesn't exist
                                self.advance();
                            }
                            return Ok(());
                        }

                        Command::End => {
                            // we found an end, continue execution at the next statement
                            self.advance();
                            // println!("Found end, resuming execution at {}", self.pc);
                            break;
                        }
                        Command::If(_) => {
                            self.enter_block(true, true, Block::IfBlock(self.pc, false))?;
                        }
                        Command::While(_) => {
                            self.enter_block(false, false, Block::WhileBlock(self.pc))?;
                        }
                        Command::Repeat(_) => {
                            self.enter_block(false, false, Block::RepeatBlock(self.pc))?;
                        }
                        Command::For(_) => {
                            self.enter_block(false, false, Block::ForBlock(self.pc))?;
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(())
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
            let mut advance = true;
            // println!("{:?}", self.blockstack);
            // wart of me battling the borrow checker VVV
            match self.next_statement()?.clone() {
                Statement::Expression(expr) => {
                    self.ctx.ans = expr.eval(&mut self.ctx)?;
                }
                Statement::Command(statement) => match statement {
                    Command::If(cmd) => self.exec_if(&cmd)?,
                    Command::Then => self.exec_then()?,
                    Command::Else => self.exec_else()?,
                    Command::End => self.exec_end(&mut advance)?,
                    Command::Disp(val) => self.exec_disp(val)?,
                    Command::For(cmd) => self.exec_for(&cmd, &mut advance)?,
                    _ => return Err(ExecError::NotYetImplemented),
                },
            }
            if advance {
                self.advance();
            }
        }

        Ok(())
    }

    fn next_then(&mut self) -> Result<bool, ExecError> {
        match self.peek_next()? {
            Statement::Command(cmd) => match cmd {
                Command::Then => Ok(true),
                _ => Ok(false),
            },
            _ => Ok(false),
        }
    }

    fn exec_if(&mut self, cmd: &If) -> Result<(), ExecError> {
        let result = cmd.condition.eval(&mut self.ctx)?;

        if result == true {
            if self.next_then()? {
                self.blockstack.push(Block::IfBlock(self.pc, true));
                // consume the next then so we don't execute it (executing a then is a cardinal sin)
                self.advance();
            } else {
                // do nothing, continue to the next statement as usual
            }
        } else {
            if self.next_then()? {
                // We need to skip to the End associated(?) with this If node, or go to the Else
                self.blockstack.push(Block::IfBlock(self.pc, false));
                self.scan_and_advance(true, false)?;
            } else {
                // skip just the next statement
                self.advance();
            }
        }

        Ok(())
    }

    fn exec_then(&mut self) -> Result<(), ExecError> {
        Err(ExecError::UnexpectedThen)
    }

    fn exec_else(&mut self) -> Result<(), ExecError> {
        // If there is an Else, we need to skip to the End associated with this
        let top = self.blockstack.pop().ok_or(ExecError::UnexpectedElse)?;
        match top {
            Block::IfBlock(_, took_true) => {
                if !took_true {
                    Err(ExecError::UnexpectedElse)
                } else {
                    // We are due for an If node, we took the true node
                    // we need to scan for the end
                    self.scan_and_advance(false, false)?;
                    Ok(())
                }
            }
            _ => Err(ExecError::UnexpectedElse),
        }
    }

    fn exec_end(&mut self, advance: &mut bool) -> Result<(), ExecError> {
        let top = self.blockstack.last().ok_or(ExecError::UnexpectedEnd)?;
        match top {
            Block::IfBlock(_, _) => {
                self.blockstack.pop();
                Ok(())
            },
            Block::ForBlock(pc) => {
                // println!("Regressing to {} ({:?})", pc, self.statements[pc.clone()]);
                // we need to go back to the for block
                *advance = false;
                self.pc = pc.clone();
                Ok(())
            }
            _ => Err(ExecError::NotYetImplemented),
        }
    }

    fn exec_disp(&mut self, val: ValRef) -> Result<(), ExecError> {
        let result = val.eval(&mut self.ctx)?;
        println!("{}", result);
        Ok(())
    }

    fn exec_for(&mut self, cmd: &For, advance: &mut bool) -> Result<(), ExecError> {
        // Check to see if we have already execute this for once
        // If it is at the top of the blockstack, we don't need to initialize the variable
        let mut init = true;

        if let Some(Block::ForBlock(i)) = self.blockstack.last() {
            if i == &self.pc {
                init = false
            }
        };

        if init {
            // Evaluate what to set
            let start = cmd.start.eval(&mut self.ctx)?;
            // Set it
            self.ctx.set(&cmd.var, start)?;
            self.blockstack.push(Block::ForBlock(self.pc));
        } else {
            // increment the variable by inc
            let result = BinaryOp::add(
                cmd.inc.clone(),
                Box::new(self.ctx.get(&cmd.var)?)
            ).eval(&mut self.ctx)?;

            self.ctx.set(&cmd.var, result)?;            
        }

        // Check to see if we have satisfied the condition
        let stop = cmd.stop.eval(&mut self.ctx)?;

        if BinaryOp::less(
            Box::new(self.ctx.get(&cmd.var)?),
            Box::new(stop.clone())
        ).eval(&mut self.ctx)? == 1.0 {
            // execute the loop again, do nothing
        } else {
            // Remove the blockstack, make sure its the same one
            if let Some(Block::ForBlock(pc)) = self.blockstack.pop() {
                if pc != self.pc {
                    return Err(ExecError::SyntaxError);
                }
                // jump to the End associated with this loop
                self.scan_and_advance(false, false)?;
                // we have already moved the pc to where we want it to be
                *advance = false;
            } else {
                return Err(ExecError::SyntaxError);
            }
        }

        Ok(())
    }
}

pub type EvalResult = Result<Value, ExecError>;

pub trait Eval {
    fn eval(&self, ctx: &mut Context) -> EvalResult;
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
#[derive(Debug, Clone)]
pub struct For {
    pub var: Variable,
    pub start: ValRef,
    pub stop: ValRef,
    pub inc: ValRef,
}

#[derive(Clone)]
pub struct Or {
    pub lhs: ValRef,
    pub rhs: ValRef,
}

pub struct StoreNode {
    pub val: ValRef,
    pub var: Variable,
}

impl Eval for StoreNode {
    fn eval(&self, ctx: &mut Context) -> EvalResult {
        let val = self.val.eval(ctx)?;
        ctx.set(&self.var, val)
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Store({:?}->{:?})", self.val, self.var)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(StoreNode {
            var: self.var.clone(),
            val: self.val.clone(),
        })
    }
}

pub struct VarRef {
    pub var: Variable,
}

impl Eval for VarRef {
    fn eval(&self, ctx: &mut Context) -> EvalResult {
        ctx.get(&self.var)
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Var({:?})", self.var)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(VarRef {
            var: self.var.clone(),
        })
    }
}

pub struct BinaryOp {
    pub lhs: ValRef,
    pub rhs: ValRef,
    pub num_num: fn(f64, f64) -> EvalResult,
    pub token: Token,
}

impl Eval for BinaryOp {
    fn eval(&self, ctx: &mut Context) -> EvalResult {
        let vleft = self.lhs.eval(ctx)?;
        let vright = self.rhs.eval(ctx)?;

        match vleft {
            Value::NumValue(nl) => match vright {
                Value::NumValue(nr) => return (self.num_num)(nl, nr),
                _ => Err(ExecError::TypeMismatch),
            },
            _ => Err(ExecError::TypeMismatch),
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}({:?}, {:?})", self.token, self.lhs, self.rhs)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(BinaryOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            num_num: self.num_num,
            token: self.token.clone(),
        })
    }
}

impl BinaryOp {
    pub fn or(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn or(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(fb(lhs) || fb(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Or,
            num_num: or,
        }
    }

    pub fn xor(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn xor(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(fb(lhs) ^ fb(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Xor,
            num_num: xor,
        }
    }

    pub fn and(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn and(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(fb(lhs) && fb(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::And,
            num_num: and,
        }
    }

    pub fn equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn equal(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs == rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Equal,
            num_num: equal,
        }
    }

    pub fn not_equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn not_equal(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs != rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::NotEqual,
            num_num: not_equal,
        }
    }

    pub fn greater(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn greater(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs > rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Greater,
            num_num: greater,
        }
    }

    pub fn greater_equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn greater_equal(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs >= rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::GreaterEqual,
            num_num: greater_equal,
        }
    }

    pub fn less(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn less(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs < rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Less,
            num_num: less,
        }
    }

    pub fn less_equal(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn less_equal(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::bool(lhs <= rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::LessEqual,
            num_num: less_equal,
        }
    }

    pub fn add(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn add(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs + rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Plus,
            num_num: add,
        }
    }

    pub fn minus(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn minus(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs - rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Minus,
            num_num: minus,
        }
    }

    pub fn mult(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn mult(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs * rhs))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Mult,
            num_num: mult,
        }
    }

    pub fn divide(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn divide(lhs: f64, rhs: f64) -> EvalResult {
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
            num_num: divide,
        }
    }

    pub fn power(lhs: ValRef, rhs: ValRef) -> BinaryOp {
        fn power(lhs: f64, rhs: f64) -> EvalResult {
            Ok(Value::NumValue(lhs.powf(rhs)))
        }

        BinaryOp {
            lhs: lhs,
            rhs: rhs,
            token: Token::Power,
            num_num: power,
        }
    }
}

fn fb(f: f64) -> bool {
    f != 0.0
}

pub struct Not {
    pub val: ValRef,
}

impl Eval for Not {
    fn eval(&self, ctx: &mut Context) -> EvalResult {
        let val = self.val.eval(ctx)?;

        match val {
            Value::NumValue(n) => return Ok(Value::bool(!fb(n))),
            _ => Err(ExecError::TypeMismatch),
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Not({:?})", self.val)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(Not {
            val: self.val.clone(),
        })
    }
}
pub struct Negate {
    pub val: ValRef,
}

impl Eval for Negate {
    fn eval(&self, ctx: &mut Context) -> EvalResult {
        let val = self.val.eval(ctx)?;

        match val {
            Value::NumValue(n) => return Ok(Value::NumValue(-1.0 * n)),
            _ => Err(ExecError::TypeMismatch),
        }
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Neg({:?})", self.val)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(Negate {
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

    fn exec_str(input: String) -> Value {
        let mut program = Program::new();
        match parse(&lex_str(&input), &mut program) {
            Result::Ok(_) => (),
            Result::Err(err) => panic!("{:?} oops", err),
        }
        match execute(&mut program) {
            Result::Ok(_) => (),
            Result::Err(err) => {
                if err != ExecError::UnexpectedEof {
                    panic!("{:?} oops 2", err)
                }
            }
        }

        return program.ctx.ans;
    }

    fn exec(input: &str) -> Value {
        exec_str(String::from(input))
    }
    #[test]
    fn test_binary_ops_numbers() {
        assert_eq!(exec("2+2\n"), 4.0);
        assert_eq!(exec("2-2\n"), 0.0);
        assert_eq!(exec("2*2\n"), 4.0);
        assert_eq!(exec("2/4\n"), 0.5);
        assert_eq!(exec("--2\n"), -2.0);
        assert_eq!(exec("2^4\n"), 16.0);
        // todo: validate the statements in program
    }

    #[test]
    fn test_logic_binary() {
        assert_eq!(exec("1 or 0\n"), true);
        assert_eq!(exec("1 or 0\n"), true);
        assert_eq!(exec("0 or 0\n"), false);

        assert_eq!(exec("1 xor 0\n"), true);
        assert_eq!(exec("1 xor 1\n"), false);
        assert_eq!(exec("0 or 0\n"), false);

        assert_eq!(exec("1 and 1\n"), true);
        assert_eq!(exec("1 and 0\n"), false);
        assert_eq!(exec("0 and 0\n"), false);

        assert_eq!(exec("not(1)\n"), false);
        assert_eq!(exec("not(0)\n"), true);
        assert_eq!(exec("not(1\n"), false);
        assert_eq!(exec("not(0\n"), true);
    }

    #[test]
    fn test_logic_numbers() {
        // believe it or not, this was a bug
        assert_eq!(exec("2 or 0\n"), true);
        assert_eq!(exec("12 xor 0\n"), true);
        assert_eq!(exec("not(13)\n"), false);
    }

    #[test]
    fn test_logic_expression() {
        assert_eq!(exec("1+1 or 0\n"), true);
    }

    #[test]
    fn test_equality_ops() {
        assert_eq!(exec("1 = 1\n"), true);
        assert_eq!(exec("1 = 0\n"), false);

        assert_eq!(exec("1 != 1\n"), false);
        assert_eq!(exec("1 != 0\n"), true);

        assert_eq!(exec("1 > 0\n"), true);
        assert_eq!(exec("1 > 1\n"), false);
        assert_eq!(exec("1 > 2\n"), false);

        assert_eq!(exec("1 > 0\n"), true);
        assert_eq!(exec("1 >= 1\n"), true);
        assert_eq!(exec("1 > 2\n"), false);

        assert_eq!(exec("1 < 0\n"), false);
        assert_eq!(exec("1 < 1\n"), false);
        assert_eq!(exec("1 < 2\n"), true);

        assert_eq!(exec("1 < 0\n"), false);
        assert_eq!(exec("1 <= 1\n"), true);
        assert_eq!(exec("1 < 2\n"), true);
        // todo: validate the statements in program
    }

    #[test]
    fn test_if_basic() {
        assert_eq!(
            exec(
                "
            If 0
            1+1
        "
            ),
            0.0
        );

        assert_eq!(
            exec(
                "
            If 1
            1+1
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 1+1 or 0
            1+1
        "
            ),
            2.0
        );
    }

    #[test]
    fn test_if_then() {
        assert_eq!(
            exec(
                "
            If 1
            Then
            1+1
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 0
            Then
            1+1
            End
        "
            ),
            0.0
        );

        assert_eq!(
            exec(
                "
            If 1
            Then
            1+1
        "
            ),
            2.0
        );
    }

    #[test]
    fn test_if_then_else() {
        assert_eq!(
            exec(
                "
            If 1
            Then
                1+1
            Else
                2+2
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 0
            Then
                1+1
            Else
                3+3
            End
        "
            ),
            6.0
        );

        assert_eq!(
            exec(
                "
            If 1
            Then
                1+1
            Else
                4+4
        "
            ),
            2.0
        );
    }

    #[test]
    fn test_singleline_if() {
        assert_eq!(
            exec(
                "
            1+1
            If 0: Then
                1+2
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 1: Then
                1+1
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 0: Then
                1+1
            Else:
                2+2
            End
        "
            ),
            4.0
        );
    }

    #[test]
    fn test_if_nested() {
        assert_eq!(
            exec(
                "
            2+2
            If 1
            Then
                If 0
                    1+1
            End
        "
            ),
            4.0
        );

        assert_eq!(
            exec(
                "
            2+2
            If 1
            Then
                If 0
                    1+1
        "
            ),
            4.0
        );

        assert_eq!(
            exec(
                "
            2+2
            If 1
            Then
                If 1
                    1+1
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 1
            Then
                1+1
                If 0
                Then
                    2+2
                End
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 1
            Then
                1+1
                If 0
                Then
                    2+2
                Else
                    3+3
                End
            End
        "
            ),
            6.0
        );

        assert_eq!(
            exec(
                "
            If 1
            Then
                If 1
                Then
                    If 1
                    Then
                        1+1
                    End
                End
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 1
            Then
                If 0
                Then
                3/69
                Else
                    If 1
                    Then
                        1+1
                    End
                End
            End
        "
            ),
            2.0
        );

        assert_eq!(
            exec(
                "
            If 1: Then
                If 0: Then
                    1+1
                Else
                    3+3
            
            "
            ),
            6.0
        );

        assert_eq!(
            exec(
                "
            If 1: Then
                If 0: Then
                    1+1
                Else
                    If 1: Then
                        3*2
                    Else
                        2+2
            
            "
            ),
            6.0
        );
    }

    #[test]
    fn test_all_real_vars() {
        for chr in b'A'..b'Z' {
            let var = String::from_utf8([chr].to_vec()).unwrap();
            assert_eq!(exec_str(format!("2->{var}\n{var}\n", var = var)), 2.0)
        }
    }

    #[test]
    fn test_vars_as_expressions() {
        assert_eq!(
            exec(
                "
                2->A
                A+A
            "
            ),
            4.0
        );

        assert_eq!(
            exec(
                "
                2->A
                3->A
                A
            "
            ),
            3.0
        );
    }

    #[test]
    fn test_large_input() {
        assert_eq!(
        exec("(1+2+3+4+5+6+7+8+9+1+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+91+2+3+4+5+6+7+8+9)\n")
        , 15588.0);
        assert_eq!(
            exec("((((((((((((((((((((((((((((((((((((((((((((((((((50)49)48)47)46)45)44)43)42)41)40)39)38)37)36)35)34)33)32)31)30)29)28)27)26)25)24)23)22)21)20)19)18)17)16)15)14)13)12)11)10)9)8)7)6)5)4)3)2)1)\n")
            , 30414093201713376000000000000000000000000000000000000000000000000.0)
    }

    #[test]
    fn test_numbers_operators() {
        // assert_eq!(exec("1+2-3*4/5"), 0.6);
        assert_eq!(exec("2(3-4)\n"), -2.0);
        assert_eq!(exec("2(4.5\n"), 9.0);
        assert_eq!(exec("3(4(5(6(7(8(9))))))\n"), 181440.0);
        assert_eq!(exec("3(4(5(6(7(8(9)))))\n"), 181440.0);
        assert_eq!(exec("3(4(5(6(7(8(9\n"), 181440.0);
        assert_eq!(exec("2^(4*4+3\n"), 524288.0);
        assert_eq!(exec("(5)4\n"), 20.0);
        assert_eq!(exec("(((5)4)3)2\n"), 120.0);
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(exec("e1\n"), 10.0);
        assert_eq!(exec("e10\n"), 1.0e10);
        assert_eq!(exec("e-10\n"), 1.0e-10);
        assert_eq!(exec("4e5\n"), 400000.0);
    }
}
