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
use crate::compiler::cs_analyzer::scope_checker::function;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;


pub fn sc_funclist(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    ss: &mut ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let Some(lchild) = node.get_lchild() else {
        return Ok(());
    };

    function::sc_function(ast,func_table,lchild,ss)?;

    let rchild = node.get_rchild().unwrap();

    sc_funclist(ast,func_table,rchild,ss)
}
