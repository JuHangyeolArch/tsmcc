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
mod funciface;
pub mod valuekind;
mod basicval;

mod paramlist;

mod declist;
mod declaration;
mod declarator;

mod stmtlist;
mod statement;

mod assignexpr;
mod condexpr;
mod addexpr;
mod term;
mod unaryexpr;
mod factor;

pub mod id;
mod stars;

mod arglist;


#[derive(Debug,Clone,PartialEq)]
enum ScopeBlock {
    Var(String),
    Block,
}

impl ScopeBlock {
    pub fn block() -> Self {
        ScopeBlock::Block
    }

    pub fn var(name: &str) -> Self {
        ScopeBlock::Var(name.to_string())
    }
    

}
#[derive(Debug,Clone,PartialEq)]
pub struct ScopeStack {
    stack: Vec<ScopeBlock>
}

impl ScopeStack  {
    pub fn new() -> Self {
        ScopeStack   {
            stack: Vec::new()
        }
    }

    pub fn block_in(&mut self) {
        self.stack.push(ScopeBlock::block())
    }

    pub fn push(&mut self, name: String) {
        
        println!("p {}",name);
        self.stack.push(ScopeBlock::var(&name))
    }
    
    pub fn pop(&mut self) {
        while let Some(ScopeBlock::Var(_)) = self.stack.pop() {}
    }

    pub fn exist(&self, name: String) -> bool {

        println!("{}",name);
        self.stack
            .iter()
            .any(|block| {
                let ScopeBlock::Var(s) = block else {
                    return false;
                };
                *s == name
            }
            )
    }


}
pub fn scope_checker(
    ast: &Ast,
    func_table: &mut FuncTable
) -> Result<(),CsError> {

    
    program::sc_program(
        ast,func_table,
        ast.get_root(),
        &mut ScopeStack::new()
    )
    
}

