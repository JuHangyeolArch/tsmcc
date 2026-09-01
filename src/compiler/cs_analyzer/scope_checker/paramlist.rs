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
use crate::compiler::cs_analyzer::scope_checker::declarator;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;

pub fn sc_paramlist(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &mut ScopeStack,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let Some(lchild) = node.get_lchild() else {
        return Ok(());
    };

    let param = declarator::sc_declarator(ast,lchild)?;
    func_table
        .get_bypos(entry)
        .unwrap()
        .add_param(&param.get_name(),&param.get_type(),Some(lchild));
    ss.push(param.get_name());
    
    let rchild = node.get_rchild().unwrap();
    sc_paramlist(ast,func_table,rchild,entry,ss)
    
}
