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
use crate::compiler::cs_analyzer::scope_checker::factor;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;


pub fn sc_factypecast(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &mut ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let rchild = node.get_rchild().unwrap();

    factor::sc_factor(ast,func_table,rchild,entry,ss)?;
    
    Ok(())

}
