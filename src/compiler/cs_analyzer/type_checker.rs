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
use crate::compiler::cs_analyzer::CsError;

mod program;
mod funclist;
mod function;
mod valuekind;
mod basicval;
mod declarator;

mod stmtlist;
mod statement;

mod assignexpr;
mod condexpr;
mod addexpr;
mod term;
mod unaryexpr;
mod factor;

mod id;
mod stars;

mod arglist;

pub fn type_checker(
    ast: &Ast,
    func_table: &FuncTable
) -> Result<(),CsError> {
    
    program::tc_program(ast,func_table,ast.get_root())
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SideKind {
    Lvalue,
    Rvalue,
}
