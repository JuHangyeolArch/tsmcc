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
use crate::compiler::cs_analyzer::scope_checker::ScopeStack;


pub fn sc_facvar(
    ast: &Ast,
    id: NodeID,
    ss: &mut ScopeStack
) -> Result<(),CsError> {

    let node = ast.get(id).unwrap();
    let lchild = node.get_lchild().unwrap();

    let var_name = id::sc_id(ast,lchild)?;

    if !ss.exist(var_name) {
        return Err(CsError::InvalidUse(id));
    };
    
    Ok(())

}
