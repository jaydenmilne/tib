use crate::lexer::Token;
use crate::parser::Statement;
use crate::parser::*;
use core::fmt::Debug;
use rand::Rng;
use std::collections::HashMap;

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
    ImmutableVariable,
    UnexpectedThen,
    FailedToFindForNode,
    UnknownLabel,
    NonNumericTypeInList,
    DimensionMismatch,
}

#[derive(Debug)]
pub struct Context {
    pub ans: Value,
    pub reals: HashMap<char, Value>,
    // rest of the variables/state will go here
}

impl Value {
    pub fn to_bool(&self) -> Result<bool, ExecError> {
        match self {
            Value::NumValue(n) => Ok(*n != 0.0),
            _ => Err(ExecError::TypeMismatch),
        }
    }
}

impl Context {
    fn set(&mut self, var: &Variable, val: Value) -> Result<Value, ExecError> {
        match var {
            Variable::RealVar(name) => {
                self.reals.insert(name.clone(), val.clone());
                Ok(val)
            }
            Variable::Ans => Err(ExecError::ImmutableVariable),
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
            Variable::Ans => Ok(self.ans.clone()),
        }
    }
}

#[derive(Debug, Clone)]
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
    pub label_cache: HashMap<String, usize>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            ctx: Context::new(),
            statements: Vec::new(),
            pc: 0,
            blockstack: Vec::new(),
            label_cache: HashMap::new(),
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
        // for a normal scan and advance, we want to execute what we found next
        // in this case, we don't want to
        self.advance();

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
        // of an else statement.
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
        self.advance();
        loop {
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
                                continue;
                            }

                            // Always point the pc at one before what we found
                            self.pc -= 1;
                            return Ok(());
                        }

                        Command::End => {
                            // Always point the pc at one before what we found
                            self.pc -= 1;
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
            self.advance();
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
            // println!("{:?}", self.blockstack);
            // wart of me battling the borrow checker VVV
            match self.next_statement()?.clone() {
                Statement::Expression(expr) => {
                    self.ctx.ans = expr.eval(&mut self.ctx)?;
                }
                Statement::Command(statement) => match statement {
                    Command::If(expr) => self.exec_if(expr)?,
                    Command::Then => self.exec_then()?,
                    Command::Else => self.exec_else()?,
                    Command::End => self.exec_end()?,
                    Command::Disp(val) => self.exec_disp(val)?,
                    Command::For(cmd) => self.exec_for(&cmd)?,
                    Command::While(expr) => self.exec_while(expr)?,
                    Command::Repeat(_cmd) => self.exec_repeat()?,
                    Command::Lbl(_) => (),
                    Command::Goto(label) => match self.label_cache.get(&label) {
                        Some(loc) => {
                            self.pc = *loc;
                        }
                        None => return Err(ExecError::UnknownLabel),
                    },
                    Command::DecrementSkip(var, val) => self.exec_ds_rs(&var, &val, true)?,
                    Command::IncrementSkip(var, val) => self.exec_ds_rs(&var, &val, false)?,

                    _ => return Err(ExecError::NotYetImplemented),
                },
            }
            self.advance();
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

    fn exec_ds_rs(
        &mut self,
        var: &Variable,
        value: &ValRef,
        decrement: bool,
    ) -> Result<(), ExecError> {
        let delta = if decrement { -1.0 } else { 1.0 };
        let new_value = BinaryOp::add(
            Box::new(self.ctx.get(&var)?),
            Box::new(Value::NumValue(delta)),
        )
        .eval(&mut self.ctx)?;
        self.ctx.set(&var, new_value)?;

        let mut skip = false;
        let lhs = Box::new(self.ctx.get(var)?.clone());
        let rhs = value.clone();
        if decrement {
            if BinaryOp::less(lhs, rhs).eval(&mut self.ctx)?.to_bool()? {
                skip = true;
            }
        } else {
            if BinaryOp::greater(lhs, rhs).eval(&mut self.ctx)?.to_bool()? {
                skip = true;
            }
        }

        if skip {
            self.advance();
        }

        Ok(())
    }

    fn exec_if(&mut self, condition: ValRef) -> Result<(), ExecError> {
        let result = condition.eval(&mut self.ctx)?;

        if result.to_bool()? {
            if self.next_then()? {
                self.blockstack.push(Block::IfBlock(self.pc, true));
                // point the pc at the then statement so that next iteration of main loop advances
                // past it (we don't want to execute a then ever)
                self.advance();
            } else {
                // do nothing, continue to the next statement as usual
            }
        } else {
            if self.next_then()? {
                // We need to skip to the End associated(?) with this If node, or go to the Else
                self.blockstack.push(Block::IfBlock(self.pc, false));
                self.scan_and_advance(true, false)?;
                // If the next thing is an else, advance since we don't want to execute the
                // else since we did not take the true branch.
                // If we didn't find an else, leave the pc where it is
                if let Statement::Command(Command::Else) = self.peek_next()? {
                    // if scan_and_advance found an else, we don't want to execute it since
                    // executing an else will just skip to the End
                    self.advance();
                };
            } else {
                // singleline if
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
        // An else should only be executed if we took the true branch of an if statement
        // the signal is to skip to the end.
        // If there is an Else, we need to skip to the End associated with this
        let top = self.blockstack.last().ok_or(ExecError::UnexpectedElse)?;
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

    fn exec_repeat_end(&mut self, pc: usize) -> Result<(), ExecError> {
        let cmd = &self.statements[pc].clone();
        if let Statement::Command(Command::Repeat(expr)) = cmd {
            let result = expr.eval(&mut self.ctx)?;
            if result.to_bool()? {
                // execute the loop again, set pc back to point at the repeat node
                self.pc = pc;
            } else {
                // We are done with the loop.
                // Remove the blockstack, make sure its the same one
                if let Some(Block::RepeatBlock(popped_pc)) = self.blockstack.pop() {
                    if pc != popped_pc {
                        return Err(ExecError::SyntaxError);
                    }
                // we are already pointing at the end, so just continue as normal...
                } else {
                    return Err(ExecError::SyntaxError);
                }
            }
            Ok(())
        } else {
            Err(ExecError::FailedToFindForNode)
        }
    }

    fn exec_while_end(&mut self, pc: usize) -> Result<(), ExecError> {
        let cmd = &self.statements[pc].clone();
        if let Statement::Command(Command::While(expr)) = cmd {
            let result = expr.eval(&mut self.ctx)?;
            if result.to_bool()? {
                // execute the loop again, set pc back to point at the While node
                self.pc = pc;
            } else {
                // We are done with the loop.
                // Remove the blockstack, make sure its the same one
                if let Some(Block::WhileBlock(popped_pc)) = self.blockstack.pop() {
                    if pc != popped_pc {
                        return Err(ExecError::SyntaxError);
                    }
                // we are already pointing at the end, so just continue as normal...
                } else {
                    return Err(ExecError::SyntaxError);
                }
            }
            Ok(())
        } else {
            Err(ExecError::FailedToFindForNode)
        }
    }

    fn exec_end(&mut self) -> Result<(), ExecError> {
        let top = self.blockstack.last().ok_or(ExecError::UnexpectedEnd)?;
        let top = top.clone();

        match top {
            Block::IfBlock(_, _) => {
                self.blockstack.pop();
                Ok(())
            }
            Block::ForBlock(pc) => {
                // println!("Regressing to {} ({:?})", pc, self.statements[pc.clone()]);
                // we need to go back to the for block
                self.exec_for_end(pc.clone())
            }
            Block::RepeatBlock(pc) => self.exec_repeat_end(pc.clone()),
            Block::WhileBlock(pc) => self.exec_while_end(pc.clone()),
            _ => Err(ExecError::SyntaxError),
        }
    }

    fn exec_while(&mut self, condition: ValRef) -> Result<(), ExecError> {
        self.blockstack.push(Block::WhileBlock(self.pc));
        // evaluate the condition
        let result = condition.eval(&mut self.ctx)?;

        if !result.to_bool()? {
            // condition is not true, skip to end
            self.scan_and_advance(false, false)?;
            // we are now pointing one before the End
        }

        Ok(())
    }

    fn exec_repeat(&mut self) -> Result<(), ExecError> {
        // only thing to do is put an entry on the blockstack
        self.blockstack.push(Block::RepeatBlock(self.pc));
        Ok(())
    }

    fn exec_disp(&mut self, val: ValRef) -> Result<(), ExecError> {
        let result = val.eval(&mut self.ctx)?;
        println!("{}", result);
        Ok(())
    }

    fn for_should_execute_loop(&mut self, cmd: &For) -> Result<bool, ExecError> {
        // Check to see if we have satisfied the condition
        let stop = cmd.stop.eval(&mut self.ctx)?;

        Ok(
            BinaryOp::less_equal(Box::new(self.ctx.get(&cmd.var)?), Box::new(stop.clone()))
                .eval(&mut self.ctx)?
                .to_bool()?,
        )
    }

    fn exec_for_end(&mut self, pc: usize) -> Result<(), ExecError> {
        let cmd = &self.statements[pc].clone();
        if let Statement::Command(Command::For(cmd)) = cmd {
            // increment the variable by inc
            let result = BinaryOp::add(cmd.inc.clone(), Box::new(self.ctx.get(&cmd.var)?))
                .eval(&mut self.ctx)?;

            self.ctx.set(&cmd.var, result)?;

            if self.for_should_execute_loop(cmd)? {
                // execute the loop again, set pc back to point at the for node
                self.pc = pc;
            } else {
                // We are done with the loop.
                // Remove the blockstack, make sure its the same one
                if let Some(Block::ForBlock(popped_pc)) = self.blockstack.pop() {
                    if pc != popped_pc {
                        return Err(ExecError::SyntaxError);
                    }
                // we are already pointing at the end, so just continue as normal...
                } else {
                    return Err(ExecError::SyntaxError);
                }
            }
            Ok(())
        } else {
            Err(ExecError::FailedToFindForNode)
        }
    }

    fn exec_for(&mut self, cmd: &For) -> Result<(), ExecError> {
        // A for block should only be executed when we are entering a loop, ie once
        // so this only handles initialization

        // Check to see if we have already execute this for once
        // If it is at the top of the blockstack, we don't need to initialize the variable

        // Evaluate what to set
        let start = cmd.start.eval(&mut self.ctx)?;
        // Set it
        self.ctx.set(&cmd.var, start)?;
        self.blockstack.push(Block::ForBlock(self.pc));

        // Check to see if we need to skip to the end
        if !self.for_should_execute_loop(cmd)? {
            // skip to the end
            self.scan_and_advance(false, false)?;
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
                Value::ValueList(list) => return self.num_list(nl, list, false),
                _ => Err(ExecError::TypeMismatch),
            },
            Value::ValueList(list) => match vright {
                Value::NumValue(nr) => return self.num_list(nr, list, true),
                Value::ValueList(listr) => return self.list_list(list, listr),
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
    fn num_list(&self, lhs: f64, list: Vec<Value>, swap: bool) -> EvalResult {
        // helper method to apply a binary operation to each element of a list
        let mut result: Vec<Value> = Vec::new();
        for val in list.iter() {
            match val {
                Value::NumValue(rhs) => {
                    if swap {
                        result.push((self.num_num)(*rhs, lhs)?)
                    } else {
                        result.push((self.num_num)(lhs, *rhs)?)
                    }
                }
                _ => return Err(ExecError::TypeMismatch),
            }
        }

        Ok(Value::ValueList(result))
    }

    fn list_list(&self, lhs_list: Vec<Value>, rhs_list: Vec<Value>) -> EvalResult {
        if lhs_list.len() != rhs_list.len() {
            return Err(ExecError::DimensionMismatch);
        }

        let mut result: Vec<Value> = Vec::new();

        for i in 0..lhs_list.len() {
            match lhs_list[i] {
                Value::NumValue(lhs) => match rhs_list[i] {
                    Value::NumValue(rhs) => result.push((self.num_num)(lhs, rhs)?),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Value::ValueList(result))
    }

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

#[derive(Debug)]
pub struct ExprList {
    pub exprs: Vec<ValRef>,
}

impl Eval for ExprList {
    fn eval(&self, ctx: &mut Context) -> EvalResult {
        let mut vals: Vec<Value> = Vec::new();

        for expr in self.exprs.iter() {
            let result = expr.eval(ctx)?;
            // make sure no one snuck in a non-numeric value in this
            match result {
                Value::NumValue(_) => (),
                _ => return Err(ExecError::NonNumericTypeInList),
            }
            vals.push(result);
        }

        Ok(Value::ValueList(vals))
    }

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.exprs)
    }

    fn clone_expr(&self) -> Box<dyn Eval> {
        Box::new(ExprList {
            // I hope this does what I think it does
            exprs: self.exprs.clone(),
        })
    }
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
    fn test_if_then_else_true() {
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
    }

    #[test]
    fn test_if_then_else_false() {
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
    }

    #[test]
    fn test_if_then_else_no_end() {
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

    #[test]
    fn test_basic_for_loop() {
        assert_eq!(
            exec(
                "
            3 -> B
            For(A, 0, 5)
            2*B -> B
            End
            B
        "
            ),
            192.0
        );
    }

    #[test]
    fn test_basic_for_loop_inc() {
        assert_eq!(
            exec(
                "
            3 -> B
            For(A, 0, 5, 2)
            2*B -> B
            End
            B
        "
            ),
            24.0
        );
    }

    #[test]
    fn test_basic_for_loop_inc_vars() {
        assert_eq!(
            exec(
                "
            3 -> B
            2 -> P
            5 -> Q
            0 -> R
            For(A, R, Q, P)
            2*B -> B
            End
            B
        "
            ),
            24.0
        );
    }

    #[test]
    fn test_nested_for_loop() {
        assert_eq!(
            exec(
                "
            --1 -> C
            For(A, --1, 0)
            For(B, --1, 0)
            1 -> C
            End
            End
            C
        "
            ),
            1.0
        );
    }

    #[test]
    fn test_super_nested_for_loop() {
        // This takes about a minute on a calculator lol
        assert_eq!(
            exec(
                "
            0 -> Z
            For(A, 0, 5)
            For(B, 0, 5)
            For(C, 0, 5)
            For(D, 0, 5)
            For(E, 0, 5)
            Z+1 -> Z
            End
            End
            End
            End
            End
            Z
        "
            ),
            7776.0
        );
    }

    #[test]
    fn test_naked_for_loop() {
        assert_eq!(
            exec(
                "
            For(A, 0, 5)
            1+1
        "
            ),
            2.0
        );
    }

    #[test]
    fn test_nested_naked_for_loop() {
        assert_eq!(
            exec(
                "
            For(A, 0, 5)
                For(B, 0, 5)
            1+1
        "
            ),
            2.0
        );
    }

    // todo: naked end and if test

    #[test]
    fn test_for_if() {
        assert_eq!(
            exec(
                "
            0 -> Z
            For(A, 0, 5
                If A = 5
                    2 -> Z
            End
            Z
        "
            ),
            2.0
        );
    }

    #[test]
    fn test_for_if_then() {
        assert_eq!(
            exec(
                "
            4 -> Z
            For(A, 0, 5
                If A = 5
                Then
                    2 -> Z
                End
            End
            Z
        "
            ),
            2.0
        );
    }

    #[test]
    fn test_for_if_then_else() {
        assert_eq!(
            exec(
                "
            0 -> B
            For(A, 0, 5)
                If A < 3
                Then
                    B + 2 -> B
                Else
                    B * 2 -> B
                End
            End
            B
        "
            ),
            48.0
        );
    }

    #[test]
    fn test_while_basic() {
        assert_eq!(
            exec(
                "
            0 -> B
            While B < 5
                B + 1 -> B
            End
            B
    "
            ),
            5.0
        );
    }

    #[test]
    fn test_while_skip_loop() {
        assert_eq!(
            exec(
                "
            0 -> B
            While B > 5
                B + 1 -> B
            End
            B
    "
            ),
            0.0
        );
    }

    #[test]
    fn test_while_nested_if() {
        assert_eq!(
            exec(
                "
            0 -> A
            0 -> B
            While B < 5
                If B < 3
                Then
                    B + A -> A
                Else
                    B * A -> A
                End
                B + 1 -> B
            End
            A
    "
            ),
            36.0
        );
    }

    #[test]
    fn test_while_nested_if_skip() {
        assert_eq!(
            exec(
                "
            0 -> A
            0 -> B
            While B > 5
                If B < 3
                Then
                    B + A -> A
                Else
                    B * A -> A
                End
                B + 1 -> B
            End
            A
    "
            ),
            0.0
        );
    }

    #[test]
    fn test_repeat_basic() {
        assert_eq!(
            exec(
                "
        0 -> B
        Repeat B < 5
            B + 1 -> B
        End
        B
    "
            ),
            5.0
        );
    }

    #[test]
    fn test_repeat_skip_loop() {
        assert_eq!(
            exec(
                "
        0 -> B
        Repeat B > 5
            B + 1 -> B
        End
        B
    "
            ),
            1.0
        );
    }

    #[test]
    fn test_lbl_goto_basic() {
        assert_eq!(
            exec(
                "
        0 -> A
        Lbl A
        A + 1 -> A
        If A < 5
            Goto A
        A
    "
            ),
            5.0
        );
    }

    #[test]
    fn test_decrement_skip_basic() {
        assert_eq!(
            exec(
                "
        5 -> A
        DS<(A, 6
        0 -> A
        A
    "
            ),
            4.0
        );

        assert_eq!(
            exec(
                "
        7 -> A
        DS<(A, 6)
        0 -> A
        A
    "
            ),
            0.0
        );
    }

    #[test]
    fn test_increment_skip_basic() {
        assert_eq!(
            exec(
                "
        7 -> A
        IS<(A, 6
        0 -> A
        A
    "
            ),
            8.0
        );

        assert_eq!(
            exec(
                "
        1 -> B
        IS<(B, 2)
        0 -> B
        B
    "
            ),
            0.0
        );
    }

    // Testing Todo:
    // - Ans is going to have a huge testing burden
    // - Test Ans interactions with IS< and DS<
}
