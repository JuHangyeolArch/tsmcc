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

mod scope_checker;
mod type_checker;
mod flow_checker;


#[derive(Debug, Clone, PartialEq)]
pub struct CsAnalyzer {
    scope_checked: bool,
    type_checked: bool,
    flow_checked: bool,
}

impl CsAnalyzer {

    pub fn new() -> Self {
        CsAnalyzer {
            scope_checked: false,
            type_checked: false,
            flow_checked: false,
        }
    }

    pub fn step(
        &mut self,
        ast: &Ast,
        func_table: &mut FuncTable
    )->  Option<Result<(),CompileError>> {

        if !self.scope_checked {
            if let Err(err) = scope_checker::scope_checker(ast,func_table) {
                return Some(Err(CompileError::CSError(err)));
            }               
            self.scope_checked = true;            
            println!("sc");
            return Some(Ok(()));
        }

        if !self.type_checked {
            if let Err(err) = type_checker::type_checker(ast,func_table) {
                return Some(Err(CompileError::CSError(err)));
            }               
            self.type_checked = true;            
            println!("tc");
        }


        if !self.flow_checked {
            if let Err(err) = flow_checker::flow_checker(ast,func_table) {
                return Some(Err(CompileError::CSError(err)));
            }               
            self.flow_checked = true;            

            println!("fc");
            return Some(Ok(()));
        }
        
        None
    }
    
}


#[derive(Debug,Clone,PartialEq)]
pub enum CsError {
    InvalidUse(NodeID),
    TypeMismatched(NodeID),
    DucplicateDeclaration(NodeID),
    AssginToRValue(NodeID),
    ReferenceToRValue(NodeID),
    CannotDereference(NodeID),
    InvalidArguement(NodeID),
    PossiblyNoReturn(usize),
    UnexpectedError,
}
