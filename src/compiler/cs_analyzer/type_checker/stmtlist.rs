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
use crate::compiler::cs_analyzer::type_checker::statement;

pub fn tc_stmtlist(
    ast: &Ast,
    func_table: &FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();

    let Some(lchild) = node.get_lchild() else {
        return Ok(());
    };
    
    statement::tc_statement(ast,func_table,lchild,entry)?;            
    let rchild = node.get_rchild().unwrap();
    tc_stmtlist(ast,func_table,rchild,entry)

    
}
