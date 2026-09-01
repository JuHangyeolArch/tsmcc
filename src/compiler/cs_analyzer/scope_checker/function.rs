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
use crate::compiler::cs_analyzer::scope_checker::funciface;
use crate::compiler::cs_analyzer::scope_checker::statement::stmtcomp;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;



pub fn sc_function(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    ss: &mut ScopeStack
) -> Result<(),CsError> {
    
    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();
    ss.block_in();
    
    let entry = funciface::sc_funciface(ast,func_table,lchild,ss)?;
    
    let rchild = node.get_rchild().unwrap();

    stmtcomp::sc_stmtcomp(ast,func_table,rchild, entry,ss)?;

    ss.pop();

    Ok(())
}
