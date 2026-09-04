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
use crate::compiler::cs_analyzer::type_checker::basicval;
use crate::compiler::cs_analyzer::type_checker::stars;

pub fn tc_valuekind(
    ast: &Ast,
    id: NodeID
) -> Result<ValueType,CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let base_kind = basicval::tc_basicval(ast,lchild)?;

    let rchild = node.get_rchild().unwrap();

    let depth = stars::tc_stars(ast,rchild)?;

    Ok(ValueType::new(base_kind, depth))
}
