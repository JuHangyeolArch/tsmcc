use std::ops::Range;
use std::fmt;
use std::error::Error;
use crate::compiler::ProducedBy;

use crate::compiler::SpanInfo;
use crate::compiler::CompileError;
use crate::compiler::ast::{
    Ast,
    AstNode,
    NodeID,
    AstNodeKind,
    AdditiveOpKind,
    MultiplicativeOpKind,
    StatementKind,
    FactorKind,
    BasicValueKind,
};
use crate::compiler::symtable::{
    FuncTable,
    FunctionInfo,
    ValueType,
    VarList,
    Variable,  
};

use crate::compiler::iloc::{
    IlocProgram,
    IlocLine,
    IlocInstruction,
    Register,
    IlocFunction,
    GotoLabel,
    IlocLabel
};


#[derive(Debug, Clone, PartialEq)]
pub struct IrGenerator {
    is_over: bool
}

mod program;
mod funclist;
mod function;
mod stmtlist;
mod statement;
mod assignexpr;
mod condexpr;
mod addexpr;
mod term;
mod unaryexpr;
mod negative;
mod factor;
mod loadval;
mod id;
mod arglist;


impl IrGenerator {

    pub fn new() -> Self {
        IrGenerator{
            is_over: false,
        }
    }

    pub fn step(&mut self, ast: &Ast, func_table: &FuncTable,iloc: &mut IlocProgram) -> Option<Result<(),CompileError>> {
        if !self.is_over {
            if let Err(e) = program::ir_program(ast, func_table, ast.get_root(),iloc) {
                return Some(Err(CompileError::IRGenError(e)));
                self.is_over = true;
            } else {
                return Some(Ok(()));

            };
        };

        None
    }

}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum SideKind {
    LValue,
    RValue,
}


#[derive(Debug,Clone,PartialEq)]
pub enum IrGenError {
    UnexpectedError,
}
