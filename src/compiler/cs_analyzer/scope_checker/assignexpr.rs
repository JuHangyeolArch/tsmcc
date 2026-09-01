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
use crate::compiler::cs_analyzer::scope_checker::condexpr;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;

pub fn sc_assexpr(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    condexpr::sc_condexpr(ast,func_table,lchild,entry,ss)?;
    
    let Some(rchild) = node.get_rchild() else {
        return Ok(());
    };

    sc_assexpr(ast,func_table,rchild,entry,ss)?;
    
    Ok(())
}
