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
use crate::compiler::cs_analyzer::scope_checker::id;

pub fn sc_facnum(
    ast: &Ast,
    func_table: &mut FuncTable,
    id: NodeID,
    entry: usize,
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let var_name = id::sc_id(ast,lchild)?;

    let Some(_) = func_table
        .get_bypos(entry)
        .unwrap()
        .get_vars()
        .get_byname(&var_name) else {
            return Err(CsError::InvalidUse(id));
        };
    
    Ok(())

}
