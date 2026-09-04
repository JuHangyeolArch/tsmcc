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
use crate::compiler::cs_analyzer::scope_checker::valuekind;
use crate::compiler::symtable::{
    FuncTable,
    FunctionInfo,
    ValueType,
    VarList,
    Variable,  
};
use crate::compiler::cs_analyzer::CsError;
use crate::compiler::cs_analyzer::scope_checker::basicval;


pub fn sc_basicval(
    ast: &Ast,
    id: NodeID
) -> Result<BasicValueKind,CsError> {

    let node = ast.get(id).unwrap();

    let AstNodeKind::BasicValue(val) = node.get_kind() else {
        return Err(CsError::UnexpectedError(id));
    };

    Ok(val)

    
}
