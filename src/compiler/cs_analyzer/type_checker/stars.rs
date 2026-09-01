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


pub fn tc_stars(
    ast: &Ast,
    id: NodeID
) -> Result<u32,CsError> {

    let node = ast.get(id).unwrap();
    
    match node.get_rchild() {
        Some(rchild) => Ok(tc_stars(ast,rchild)? + 1),
        None => Ok(0)
    }

}
