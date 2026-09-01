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
use crate::compiler::cs_analyzer::scope_checker::statement;
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;

pub fn sc_stmtlist(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
    ss: &mut ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let Some(lchild) = node.get_lchild() else {
        return Ok(());
    };
    
    statement::sc_statement(ast,func_table,lchild,entry,ss)?;            
    let rchild = node.get_rchild().unwrap();
    sc_stmtlist(ast,func_table,rchild,entry,ss)

    
}
