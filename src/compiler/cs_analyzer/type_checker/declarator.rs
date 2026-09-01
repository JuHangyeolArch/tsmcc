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
use crate::compiler::cs_analyzer::scope_checker::valuekind;
use crate::compiler::cs_analyzer::scope_checker::id;

pub fn sc_declarator(
    ast: &Ast,
    id: NodeID
) -> Result<Variable,CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();
    let valkind = valuekind::sc_valuekind(ast, lchild)?;
    let rchild = node.get_rchild().unwrap();
    let name = id::sc_id(ast,rchild)?;

    Ok(Variable::new(&name,&valkind,Some(id)))  
}

