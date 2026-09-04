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

pub fn tc_id(
    ast: &Ast,
    id: NodeID
) -> Result<String,CsError> {

    let node = ast.get(id).unwrap();

    let AstNodeKind::Identifier(id) = node.get_kind() else {
        return Err(CsError::UnexpectedError(id));
    };

    Ok(id)

}

